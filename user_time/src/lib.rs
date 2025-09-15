// SPDX-License-Identifier: MPL-2.0

//! Standalone user-mode time library adapted from kernel `aster-time`.
//! This version removes direct `ostd` dependencies and replaces them
//! with user-mode friendly stubs so it can build and run as a normal
//! Rust library using `std`.

#![deny(unsafe_code)]

use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::time::Duration as StdDuration;

mod coeff;
use coeff::Coeff;
use chrono::{Datelike, Timelike};

pub const NANOS_PER_SECOND: u32 = 1_000_000_000;

// --------- Instant and ClockSource (adapted) ----------

#[derive(Debug, Default, Copy, Clone)]
pub struct Instant {
    secs: u64,
    nanos: u32,
}

impl Instant {
    pub const fn zero() -> Self {
        Self { secs: 0, nanos: 0 }
    }

    pub fn new(secs: u64, nanos: u32) -> Self {
        Self { secs, nanos }
    }

    pub fn secs(&self) -> u64 {
        self.secs
    }

    pub fn nanos(&self) -> u32 {
        self.nanos
    }
}

impl From<StdDuration> for Instant {
    fn from(value: StdDuration) -> Self {
        Self {
            secs: value.as_secs(),
            nanos: value.subsec_nanos(),
        }
    }
}

impl std::ops::Add<StdDuration> for Instant {
    type Output = Instant;
    fn add(self, other: StdDuration) -> Self::Output {
        let mut secs = self.secs + other.as_secs();
        let mut nanos = self.nanos + other.subsec_nanos();
        if nanos >= NANOS_PER_SECOND {
            secs += 1;
            nanos -= NANOS_PER_SECOND;
        }
        Instant::new(secs, nanos)
    }
}

#[derive(Debug, Copy, Clone)]
struct ClockSourceBase {
    freq: u64,
    max_delay_secs: u64,
}

impl ClockSourceBase {
    fn new(freq: u64, max_delay_secs: u64) -> Self {
        let max_delay_secs = std::cmp::max(2, max_delay_secs);
        ClockSourceBase {
            freq,
            max_delay_secs,
        }
    }
}

pub struct ClockSource {
    read_cycles: Arc<dyn Fn() -> u64 + Sync + Send>,
    base: ClockSourceBase,
    coeff: Coeff,
    last_record: RwLock<(Instant, u64)>,
}

impl ClockSource {
    pub fn new(freq: u64, max_delay_secs: u64, read_cycles: Arc<dyn Fn() -> u64 + Sync + Send>) -> Self {
        let base = ClockSourceBase::new(freq, max_delay_secs);
        debug_assert!(max_delay_secs < 600);
        let coeff = Coeff::new(NANOS_PER_SECOND as u64, freq, max_delay_secs * freq);
        Self { read_cycles, base, coeff, last_record: RwLock::new((Instant::zero(), 0)) }
    }

    fn calculate_instant(&self) -> (Instant, u64) {
        let (instant_cycles, last_instant, last_cycles) = {
            let (last_instant, last_cycles) = *self.last_record.read().unwrap();
            ((self.read_cycles)(), last_instant, last_cycles)
        };

        let delta_nanos = {
            let delta_cycles = instant_cycles - last_cycles;
            self.cycles_to_nanos_lossy(delta_cycles)
        };
        let duration = StdDuration::from_nanos(delta_nanos);
        (last_instant + duration, instant_cycles)
    }

    fn cycles_to_nanos_lossy(&self, cycles: u64) -> u64 {
        let max_cycles = self.base.max_delay_secs * self.base.freq;
        if cycles <= max_cycles {
            self.coeff * cycles
        } else {
            log::warn!(
                "ClockSource not reliable: cycles {} exceed max {} (s)",
                cycles, max_cycles
            );
            self.coeff * max_cycles
        }
    }

    fn update_last_record(&self, record: (Instant, u64)) {
        *self.last_record.write().unwrap() = record;
    }

    pub fn read_cycles(&self) -> u64 { (self.read_cycles)() }
    pub fn last_record(&self) -> (Instant, u64) { *self.last_record.read().unwrap() }
    pub fn max_delay_secs(&self) -> u64 { self.base.max_delay_secs }
    pub fn coeff(&self) -> &Coeff { &self.coeff }
    pub fn freq(&self) -> u64 { self.base.freq }
    pub(crate) fn calibrate(&self, instant_cycles: u64) { self.update_last_record((Instant::zero(), instant_cycles)); }
    pub(crate) fn update(&self) { let (i, c) = self.calculate_instant(); self.update_last_record((i, c)); }
    pub(crate) fn read_instant(&self) -> Instant { self.calculate_instant().0 }
}

// --------- Stubbed ostd-facing types/APIs ----------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SystemTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub nanos: u64,
}

impl SystemTime {
    pub const fn zero() -> Self {
        Self { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0, nanos: 0 }
    }
}

// A simple user-mode RTC stub that reads from std::time::SystemTime
trait RtcDriver: Send + Sync {
    fn read_rtc(&self) -> SystemTime;
}

struct StdRtc;
impl RtcDriver for StdRtc {
    fn read_rtc(&self) -> SystemTime {
        let now = std::time::SystemTime::now();
        let datetime: chrono::DateTime<chrono::Utc> = now.into();
        let (is_ad, year) = datetime.year_ce();
        debug_assert!(is_ad);
        SystemTime {
            year: year as u16,
            month: datetime.month() as u8,
            day: datetime.day() as u8,
            hour: datetime.hour() as u8,
            minute: datetime.minute() as u8,
            second: datetime.second() as u8,
            nanos: datetime.timestamp_subsec_nanos() as u64,
        }
    }
}

static RTC_DRIVER: OnceLock<Arc<dyn RtcDriver>> = OnceLock::new();
static VDSO_DATA_HIGH_RES_UPDATE_FN: OnceLock<Arc<dyn Fn(Instant, u64) + Sync + Send>> = OnceLock::new();
static READ_TIME: Mutex<SystemTime> = Mutex::new(SystemTime::zero());
static START_TIME: OnceLock<SystemTime> = OnceLock::new();
static CLOCK: OnceLock<Arc<ClockSource>> = OnceLock::new();

const MAX_DELAY_SECS: u64 = 100;

fn read_tsc_stub() -> u64 {
    // Use a monotonic counter derived from std::time::Instant
    static EPOCH: OnceLock<std::time::Instant> = OnceLock::new();
    let base = EPOCH.get_or_init(std::time::Instant::now);
    base.elapsed().as_nanos() as u64
}

fn tsc_freq_stub() -> u64 {
    // We treat the stub cycles as nanoseconds, so 1e9 cycles/sec.
    1_000_000_000
}

fn init_clock() {
    CLOCK.get_or_init(|| {
        Arc::new(ClockSource::new(tsc_freq_stub(), MAX_DELAY_SECS, Arc::new(read_tsc_stub)))
    });
}

fn calibrate() {
    let clock = CLOCK.get().unwrap();
    let cycles = clock.read_cycles();
    clock.calibrate(cycles);
    START_TIME.get_or_init(read);
}

fn init_timer() {
    // In user mode, spawn a background thread to periodically update the clocksource
    let max_delay_secs = CLOCK.get().unwrap().max_delay_secs() >> 1;
    let update_interval = std::time::Duration::from_secs(std::cmp::max(1, max_delay_secs));
    std::thread::spawn(move || loop {
        std::thread::sleep(update_interval);
        update_clocksource();
    });
}

fn update_clocksource() {
    let clock = CLOCK.get().unwrap();
    clock.update();
    if let Some(update_fn) = VDSO_DATA_HIGH_RES_UPDATE_FN.get() {
        let (last_instant, last_cycles) = clock.last_record();
        update_fn(last_instant, last_cycles);
    }
}

fn update_time() {
    let mut lock = READ_TIME.lock().unwrap();
    *lock = RTC_DRIVER.get_or_init(|| Arc::new(StdRtc)).read_rtc();
}

pub fn init() {
    RTC_DRIVER.get_or_init(|| Arc::new(StdRtc));
    init_clock();
    calibrate();
    init_timer();
}

pub fn get_real_time() -> SystemTime { trace_record("get_real_time"); read() }
pub fn read() -> SystemTime { trace_record("read_time"); update_time(); READ_TIME.lock().unwrap().clone() }
pub fn read_start_time() -> SystemTime { *START_TIME.get().expect("init() not called") }
pub fn read_monotonic_time() -> StdDuration { let instant = read_instant(); StdDuration::new(instant.secs(), instant.nanos()) }
pub fn default_clocksource() -> Arc<ClockSource> { CLOCK.get().expect("init() not called").clone() }
pub fn read_instant() -> Instant { CLOCK.get().expect("init() not called").read_instant() }

// Allow external code to register a vdso-like update callback
pub fn register_vdso_update_callback(cb: Arc<dyn Fn(Instant, u64) + Sync + Send>) {
    let _ = VDSO_DATA_HIGH_RES_UPDATE_FN.set(cb);
}

// --------- Simple trace facility for DFSan-friendly logging ----------
static TRACE: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn trace_record(event: &str) {
    let vec = TRACE.get_or_init(|| Mutex::new(Vec::new()));
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    vec.lock().unwrap().push(format!("{}:{}", ts, event));
}

pub fn take_trace() -> Vec<String> {
    TRACE
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap()
        .drain(..)
        .collect()
}

