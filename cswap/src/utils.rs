use bit_vec::BitVec;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::multi::many1;
use nom::{bytes::complete::tag, IResult};
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;

use std::cmp::Ord;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::solver::all_non_err_timed_out;
use crate::solver::RSolve;
use std::cmp::min;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

#[derive(Eq, PartialEq, Clone)]
pub struct RunStats {
    iter_subs: (u64, u64),
    all_non_errs_are_timeouts: (u64, u64),
    unique_subs: BTreeSet<u64>,
    has_sats: (u64, u64),
    has_unsats: (u64, u64),
    all_err: (u64, u64),
}

impl fmt::Debug for RunStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RunStats: {{ iter_subs: {:?}, unique_subs: {:?}, has_sats: {:?}, has_unsats: {:?}, all_non_errs_are_timeouts: {:?}, all_err: {:?} }}", self.iter_subs, self.unique_subs.len(), self.has_sats, self.has_unsats,self.all_non_errs_are_timeouts, self.all_err)
    }
}

impl RunStats {
    pub fn new() -> Self {
        RunStats {
            iter_subs: (0, 0),
            all_non_errs_are_timeouts: (0, 0),
            unique_subs: BTreeSet::new(),
            has_sats: (0, 0),
            has_unsats: (0, 0),
            all_err: (0, 0),
        }
    }

    pub fn record_sub(&mut self, sub_str: &str) -> bool {
        self.iter_subs.1 = self.iter_subs.1 + 1;
        let mut s = DefaultHasher::new();
        sub_str.hash(&mut s);
        self.unique_subs.insert(s.finish())
    }

    pub fn record_iter(&mut self) {
        self.iter_subs.0 = self.iter_subs.0 + 1;
    }

    pub fn record_stats_for_iter_results(&mut self, solver_results: &Vec<RSolve>) {
        if solver_results.iter().any(|r| r.has_sat()) {
            self.has_sats.0 = self.has_sats.0 + 1;
        }
        if solver_results.iter().any(|r| r.has_unsat()) {
            self.has_unsats.0 = self.has_unsats.0 + 1;
        }
        if all_non_err_timed_out(solver_results) {
            self.all_non_errs_are_timeouts.0 = self.all_non_errs_are_timeouts.0 + 1;
        }
        if solver_results.iter().all(|r| r.has_unrecoverable_error()) {
            self.all_err.0 = self.all_err.0 + 1;
        }
    }

    pub fn record_stats_for_sub_results(&mut self, solver_results: &Vec<RSolve>) {
        if solver_results.iter().any(|r| r.has_sat()) {
            self.has_sats.1 = self.has_sats.1 + 1;
        }
        if solver_results.iter().any(|r| r.has_unsat()) {
            self.has_unsats.1 = self.has_unsats.1 + 1;
        }
        if all_non_err_timed_out(solver_results) {
            self.all_non_errs_are_timeouts.1 = self.all_non_errs_are_timeouts.1 + 1;
        }
        if solver_results.iter().all(|r| r.has_unrecoverable_error()) {
            self.all_err.1 = self.all_err.1 + 1;
        }
    }

    pub fn merge_in_place(&mut self, other: &Self) {
        self.iter_subs = add_t(self.iter_subs, other.iter_subs);
        self.has_sats = add_t(self.has_sats, other.has_sats);
        self.has_unsats = add_t(self.has_unsats, other.has_unsats);
        self.all_non_errs_are_timeouts = add_t(
            self.all_non_errs_are_timeouts,
            other.all_non_errs_are_timeouts,
        );
        self.all_err = add_t(self.all_err, other.all_err);
        self.unique_subs = self
            .unique_subs
            .union(&other.unique_subs)
            .cloned()
            .collect();
    }
}

fn add_t(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
    (a.0 + b.0, a.1 + b.1)
}

pub struct MyBackoff {
    current_wait: Duration,
    max: Duration,
    min: Duration,
}

impl MyBackoff {
    pub fn new() -> Self {
        let min = Duration::from_millis(5);
        MyBackoff {
            min: min,
            max: Duration::from_secs(5),
            current_wait: min,
        }
    }

    pub fn snooze(&mut self) {
        if self.max > self.current_wait {
            thread::sleep(self.current_wait);
            self.current_wait = 2 * self.current_wait;
        } else {
            thread::sleep(self.max);
        }
    }

    pub fn reset(&mut self) {
        self.current_wait = self.min;
    }
}

#[derive(Clone)]
pub struct Timer {
    start: SystemTime,
    time_to_wait: Duration,
}

impl Timer {
    pub fn new_started(time: Duration) -> Self {
        Timer {
            start: SystemTime::now(),
            time_to_wait: time,
        }
    }

    pub fn start(&mut self, time: Duration) {
        self.start = SystemTime::now();
        self.time_to_wait = time;
    }

    pub fn is_done(&self) -> bool {
        let elapsed = match SystemTime::now().duration_since(self.start) {
            Ok(elapsed) => elapsed,
            Err(_) => return false,
        };

        match elapsed.cmp(&self.time_to_wait) {
            std::cmp::Ordering::Greater => true,
            _ => false,
        }
    }

    pub fn reset(&mut self) {
        self.start = SystemTime::now();
    }
}

pub struct StageComplete {
    number_workers_finished: Mutex<u8>,
    total_workers: u8,
}

impl StageComplete {
    pub fn new(num_workers: u8) -> Self {
        StageComplete {
            number_workers_finished: Mutex::new(0),
            total_workers: num_workers,
        }
    }

    pub fn finished() -> Self {
        StageComplete::new(0)
    }

    pub fn finish(&self) {
        let mut guard = self.number_workers_finished.lock().unwrap();
        *guard = *guard + 1;
    }

    pub fn is_complete(&self) -> bool {
        let guard = self.number_workers_finished.lock().unwrap();
        *guard == self.total_workers
    }
}

pub struct RandUniqPermGen {
    rng: Xoshiro256Plus,
    numbits: usize,
    buf: Vec<u8>,
    seen_masked: BTreeMap<BitVec, BTreeSet<BitVec>>,
    retries: u32,
    max: u32,
    use_max: bool,
    use_retries: bool,
    max_mask_size: usize,
}

impl RandUniqPermGen {
    pub fn new_masked_with_retries(
        seed: u64,
        numbits: usize,
        maxiter: u32,
        max_mask_size: usize,
    ) -> Self {
        let buf = BitVec::from_elem(numbits, false).to_bytes();
        let seen_masked = BTreeMap::new();
        let rng = Xoshiro256Plus::seed_from_u64(seed);

        let true_max = if (maxiter as f64).log2() < (numbits as f64) {
            maxiter
        } else {
            (numbits as f64).exp2() as u32
        };

        RandUniqPermGen {
            rng: rng,
            numbits: numbits,
            buf: buf,
            seen_masked: seen_masked,
            retries: 2 * true_max,
            max: true_max,
            use_max: true,
            use_retries: true,
            max_mask_size: max_mask_size,
        }
    }

    pub fn new_definite_seeded(seed: u64, numbits: usize, maxiter: u32) -> Self {
        let buf = BitVec::from_elem(numbits, false).to_bytes();
        let seen_masked = BTreeMap::new();
        let rng = Xoshiro256Plus::seed_from_u64(seed);

        let true_max = if (maxiter as f64).log2() < (numbits as f64) {
            maxiter
        } else {
            (numbits as f64).exp2() as u32
        };

        RandUniqPermGen {
            rng: rng,
            numbits: numbits,
            buf: buf,
            seen_masked: seen_masked,
            retries: 0,
            max: true_max,
            use_max: true,
            use_retries: false,
            max_mask_size: numbits,
        }
    }

    pub fn new_definite(numbits: usize, maxiter: u32) -> Self {
        Self::new_definite_seeded(0, numbits, maxiter)
    }

    pub fn rand_perm(numbits: usize, seed: u64) -> BitVec {
        let mut buf = BitVec::from_elem(numbits, false).to_bytes();
        let mut rng = Xoshiro256Plus::seed_from_u64(seed);
        rng.fill(&mut buf[..]);
        let mut bv = BitVec::from_bytes(&buf[..]);
        bv.truncate(numbits);
        bv
    }

    fn num_seen(&self) -> usize {
        self.seen_masked.values().fold(0, |a, set| set.len() + a)
    }

    pub fn mask(&mut self) -> Option<BitVec> {
        if self.max_mask_size != self.numbits {
            assert!(self.use_retries, "must use retries with mask");
        }

        if self.numbits == 0 {
            return None;
        }

        let mask_size = min(self.max_mask_size, self.numbits);
        if self.use_max && self.max <= self.num_seen() as u32 {
            return None;
        }

        let mut attempt = 0;
        while (!self.use_retries && self.use_max) || (self.use_retries && attempt < self.retries) {
            // if the mask is filling more than half full, we will empty from full
            // Otherwise fill from empty
            let desired = !(mask_size >= self.numbits / 2);
            let mut mask = BitVec::from_elem(self.numbits, !desired);
            let mut num_bits_in_desired_state = 0;
            let desired_state_has_this_many_desired_bits = if desired {
                mask_size
            } else {
                self.numbits - mask_size
            };
            while num_bits_in_desired_state != desired_state_has_this_many_desired_bits {
                let i = self.rng.gen_range(0, self.numbits);
                if mask[i] != desired {
                    mask.set(i, desired);
                    num_bits_in_desired_state = num_bits_in_desired_state + 1;
                }
            }
            if (Self::get_or_insert(&mut self.seen_masked, &mask).len() as f64).log2()
                < mask_size as f64
            {
                return Some(mask);
            }
            attempt = attempt + 1;
        }
        None
    }

    fn get_or_insert<'a, K, T>(m: &'a mut BTreeMap<K, BTreeSet<T>>, key: &K) -> &'a mut BTreeSet<T>
    where
        K: Ord,
        K: Clone,
        T: Ord,
    {
        if !m.contains_key(key) {
            m.insert(key.clone(), BTreeSet::new());
        }
        m.get_mut(key).unwrap()
    }

    #[allow(unused)]
    pub fn sample_and_mask(&mut self) -> Option<(BitVec, BitVec)> {
        if self.use_max && self.max <= self.num_seen() as u32 {
            return None;
        }

        let mask = self.mask()?;
        let mut is_new = false;
        let mut attempt = 0;
        while (!self.use_retries && self.use_max) || (self.use_retries && attempt < self.retries) {
            self.rng.fill(&mut self.buf[..]);
            let mut bv = BitVec::from_bytes(&self.buf[..]);
            bv.truncate(min(self.max_mask_size, self.numbits));
            is_new = Self::get_or_insert(&mut self.seen_masked, &mask).insert(bv.clone());
            if is_new {
                return Some((bv, mask));
            }
            attempt = attempt + 1;
        }

        None
    }

    #[allow(unused)]
    pub fn sample(&mut self) -> Option<BitVec> {
        self.sample_and_mask().map(|(s, _)| s)
    }
}

fn dynamic_format_parser<'a, 'b>(
    (s, vs): (&'a str, Vec<&'a str>),
) -> IResult<(&'a str, Vec<&'a str>), Vec<&'a str>> {
    let replace = |(s, mut vs): (&'a str, Vec<&'a str>)| {
        tag("{}")(s)
            .map_err(|e| match e {
                nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
                nom::Err::Error((i, ek)) => nom::Err::Error(((i, vs.clone()), ek)),
                nom::Err::Failure((i, ek)) => nom::Err::Failure(((i, vs.clone()), ek)),
            })
            .map(|(i, _o)| {
                let v = vs.pop().unwrap_or("");
                ((i, vs), v)
            })
    };

    let keep = |(s, vs): (&'a str, Vec<&'a str>)| {
        take_until("{}")(s)
            .map_err(|e| match e {
                nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
                nom::Err::Error((i, ek)) => nom::Err::Error(((i, vs.clone()), ek)),
                nom::Err::Failure((i, ek)) => nom::Err::Failure(((i, vs.clone()), ek)),
            })
            .map(|(i, o)| ((i, vs), o))
    };

    many1(alt((replace, keep)))((s, vs)).map(|(i, mut o)| {
        o.push(i.0); // if there are no more {}, push the rest of the str
        (("", i.1), o)
    })
}

pub type DFormatParseError<'a> =
    nom::Err<((&'a str, std::vec::Vec<&'a str>), nom::error::ErrorKind)>;

pub fn dyn_fmt<'a>(s: &'a str, mut vs: Vec<&'a str>) -> Result<String, DFormatParseError<'a>> {
    if vs.len() == 0 {
        return Ok(s.to_owned());
    }

    vs.reverse();
    let (_rem, ss) = dynamic_format_parser((s, vs))?;
    let cap = ss.iter().map(|s| s.len()).sum();
    let mut v = Vec::with_capacity(cap);
    for s in ss {
        v.extend_from_slice(s.as_bytes());
    }
    unsafe { Ok(String::from_utf8_unchecked(v)) }
}

pub fn to_strs(bv: &BitVec) -> Vec<&'static str> {
    bv.iter()
        .map(|b| if b { "true" } else { "false" })
        .collect::<Vec<&str>>()
}
#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn mask3() {
        let mut ru = RandUniqPermGen::new_masked_with_retries(1, 3, 100, 1);
        assert_debug_snapshot!(ru.sample_and_mask().unwrap());
    }

    #[test]
    fn mask() {
        let mut ru = RandUniqPermGen::new_masked_with_retries(1, 2, 100, 1);
        let mut ress = vec![];
        ress.push(ru.sample_and_mask());
        ress.push(ru.sample_and_mask());
        ress.push(ru.sample_and_mask());
        ress.push(ru.sample_and_mask());
        ress.push(ru.sample_and_mask());
        assert_debug_snapshot!(ress);
    }

    #[test]
    fn bigger_mask() {
        let mut ru = RandUniqPermGen::new_masked_with_retries(1, 2, 100, 3);
        let mut ress = vec![];
        ress.push(ru.sample_and_mask());
        ress.push(ru.sample_and_mask());
        ress.push(ru.sample_and_mask());
        ress.push(ru.sample_and_mask());
        ress.push(ru.sample_and_mask());
        assert_debug_snapshot!(ress);
    }

    #[test]
    fn dfmt() {
        assert_eq!(
            dyn_fmt("{} asdf {}!", vec!["sub", "stitute"]).unwrap(),
            "sub asdf stitute!"
        );
    }

    #[test]
    fn dfmt_no_replace() {
        assert_eq!(dyn_fmt("asdf!", vec![]).unwrap(), "asdf!");
    }

    #[test]
    fn timer_test() {
        let timer = Timer::new_started(Duration::from_millis(200));
        thread::sleep(Duration::from_millis(100));
        assert!(!timer.is_done());
        thread::sleep(Duration::from_millis(200));
        assert!(timer.is_done());
    }

    #[test]
    fn ru_definite_reaches_maxiter() {
        let mut rng = RandUniqPermGen::new_definite(10, 1);
        assert!(rng.sample().is_some());
        assert!(rng.sample().is_none());
    }

    #[test]
    fn ru_definite_reaches_maxpossible() {
        let mut rng = RandUniqPermGen::new_definite(1, 100);
        assert!(rng.sample().is_some());
        assert!(rng.sample().is_some());
        assert!(rng.sample().is_none());
    }

    #[test]
    fn ru_definite_correct_size() {
        let mut rng = RandUniqPermGen::new_definite(9, 1);
        assert_eq!(rng.sample().expect("should hold value").len(), 9);
    }

    #[test]
    fn ru_definite_distinct() {
        let mut set = BTreeSet::new();
        let mut rng = RandUniqPermGen::new_definite(2, 4);
        set.insert(rng.sample().expect("Should hold value"));
        set.insert(rng.sample().expect("Should hold value"));
        set.insert(rng.sample().expect("Should hold value"));
        set.insert(rng.sample().expect("Should hold value"));
        assert_eq!(set.len(), 4);
    }
}
