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
    seen: BTreeSet<BitVec>,
    retries: u16,
    max: u32,
    use_max: bool,
    use_retries: bool,
}

impl RandUniqPermGen {
    pub fn new_definite_seeded(seed: u64, numbits: usize, maxiter: u32) -> Self {
        let buf = BitVec::from_elem(numbits, false).to_bytes();
        let seen = BTreeSet::new();
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
            seen: seen,
            retries: 0,
            max: true_max,
            use_max: true,
            use_retries: false,
        }
    }

    pub fn new_definite(numbits: usize, maxiter: u32) -> Self {
        Self::new_definite_seeded(0, numbits, maxiter)
    }

    pub fn get_count(&self) -> u32 {
        self.seen.len() as u32
    }

    #[allow(unused)]
    pub fn sample(&mut self) -> Option<BitVec> {
        if self.max <= self.seen.len() as u32 {
            return None;
        }

        let mut is_new = false;
        let mut attempt = 0;
        while true || (self.use_retries && attempt < self.retries) {
            self.rng.fill(&mut self.buf[..]);
            let mut bv = BitVec::from_bytes(&self.buf[..]);
            bv.truncate(self.numbits);
            is_new = self.seen.insert(bv.clone());
            if is_new {
                return Some(bv);
            }
            attempt = attempt + 1;
        }

        None
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

type DFormatParseError<'a> = nom::Err<((&'a str, std::vec::Vec<&'a str>), nom::error::ErrorKind)>;

pub fn dyn_fmt<'a>(s: &'a str, mut vs: Vec<&'a str>) -> Result<String, DFormatParseError<'a>> {
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

    #[test]
    fn dfmt() {
        assert_eq!(
            dyn_fmt("{} asdf {}!", vec!["sub", "stitute"]).unwrap(),
            "sub asdf stitute!"
        );
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
