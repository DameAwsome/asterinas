use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};

use ostd::{arch::{read_tsc, timer::TIMER_FREQ, tsc_freq}, timer};
use spin::Once;

use crate::{clocksource::{ClockSource, Instant}, START_TIME, VDSO_DATA_HIGH_RES_UPDATE_FN};

pub static CLOCK: Once<Arc<ClockSource>> = Once::new();

const MAX_DELAY_SECS: u64 = 100;

pub(super) fn init() { init_clock(); calibrate(); init_timer(); }

fn init_clock() { CLOCK.call_once(|| Arc::new(ClockSource::new(tsc_freq(), MAX_DELAY_SECS, Arc::new(read_tsc)))); }

fn calibrate() { let clock = CLOCK.get().unwrap(); let cycles = clock.read_cycles(); clock.calibrate(cycles); START_TIME.call_once(crate::read); }

pub(super) fn read_instant() -> Instant { CLOCK.get().unwrap().read_instant() }

fn update_clocksource() {
	let clock = CLOCK.get().unwrap();
	clock.update();
	if let Some(update_fn) = VDSO_DATA_HIGH_RES_UPDATE_FN.get() {
		let (last_instant, last_cycles) = clock.last_record();
		update_fn(last_instant, last_cycles);
	}
}

static TSC_UPDATE_COUNTER: AtomicU64 = AtomicU64::new(1);

fn init_timer() {
	let max_delay_secs = CLOCK.get().unwrap().max_delay_secs() >> 1;
	let delay_counts = TIMER_FREQ * max_delay_secs;
	let update = move || { let counter = TSC_UPDATE_COUNTER.fetch_add(1, Ordering::Relaxed); if counter % delay_counts == 0 { update_clocksource(); } };
	timer::register_callback(update);
}