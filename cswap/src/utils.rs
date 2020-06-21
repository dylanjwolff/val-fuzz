use bit_vec::BitVec;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::error::ErrorKind;
use nom::error_position;
use nom::multi::many1;
use nom::{bytes::complete::tag, IResult};
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use std::cmp::max;
use std::cmp::min;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

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

pub struct Timer {
    done: Arc<AtomicBool>,
}

impl Timer {
    pub fn new_started(time: Duration) -> Self {
        let t = Timer {
            done: Arc::new(AtomicBool::new(false)),
        };
        t.start(time);
        t
    }

    pub fn new() -> Self {
        Timer {
            done: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn clone(&self) -> Self {
        Timer {
            done: Arc::clone(&self.done),
        }
    }

    pub fn start(&self, time: Duration) {
        let t_b = Arc::clone(&self.done);
        thread::spawn(move || {
            thread::sleep(time);
            t_b.store(true, Ordering::Relaxed)
        });
    }

    pub fn is_done(&self) -> bool {
        self.done.load(Ordering::Relaxed)
    }

    pub fn reset(&self) {
        self.done.store(false, Ordering::Relaxed);
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
    mask_size: usize,
}

impl RandUniqPermGen {
    pub fn new_masked_with_retries(
        seed: u64,
        numbits: usize,
        maxiter: u32,
        mask_size: usize,
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
            mask_size: mask_size,
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
            mask_size: numbits,
        }
    }

    pub fn new_definite(numbits: usize, maxiter: u32) -> Self {
        Self::new_definite_seeded(0, numbits, maxiter)
    }

    fn num_seen(&self) -> usize {
        self.seen_masked.values().fold(0, |a, set| set.len() + a)
    }

    pub fn mask(&mut self) -> Option<BitVec> {
        if self.mask_size != self.numbits {
            assert!(self.use_retries, "must use retries with mask");
        }

        assert!(self.mask_size <= self.numbits, "mask larger than bits");
        if self.use_max && self.max <= self.num_seen() as u32 {
            return None;
        }

        let mut attempt = 0;
        while self.use_max || (self.use_retries && attempt < self.retries) {
            // if the mask is filling more than half full, we will empty from full
            // Otherwise fill from empty
            let desired = !(self.mask_size >= self.numbits / 2);
            let mut mask = BitVec::from_elem(self.numbits, !desired);
            let mut num_bits_in_desired_state = 0;
            while num_bits_in_desired_state != min(self.mask_size, self.numbits - self.mask_size) {
                let i = self.rng.gen_range(0, self.numbits);
                if mask[i] != desired {
                    mask.set(i, desired);
                    num_bits_in_desired_state = num_bits_in_desired_state + 1;
                }
            }
            if (Self::get_or_insert(&mut self.seen_masked, &mask).len() as f64).log2()
                < self.mask_size as f64
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
        while !self.use_retries || (self.use_retries && attempt < self.retries) {
            self.rng.fill(&mut self.buf[..]);
            let mut bv = BitVec::from_bytes(&self.buf[..]);
            bv.truncate(self.mask_size);
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

fn eof_str<'a>(s: &'a str) -> IResult<&'a str, &'a str> {
    if s.len() == 0 {
        Ok(("", ""))
    } else {
        Err(nom::Err::Error(error_position!(s, ErrorKind::Eof)))
    }
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
        let timer = Timer::new();
        timer.start(Duration::from_millis(200));
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
