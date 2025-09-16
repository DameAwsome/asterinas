use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use once_cell::sync::OnceCell;
use ostd::{arch::{read_tsc, timer::TIMER_FREQ, tsc_freq}, timer};

use crate::{clocksource::{ClockSource, Instant}, VDSO_DATA_HIGH_RES_UPDATE_FN};
use once_cell::sync::OnceCell as _OnceCell;

pub static CLOCK: OnceCell<Arc<ClockSource>> = OnceCell::new();
static TSC_UPDATE_COUNTER: AtomicU64 = AtomicU64::new(1);

const MAX_DELAY_SECS: u64 = 100;

pub(super) fn init() {
	CLOCK.set(Arc::new(ClockSource::new(tsc_freq(), MAX_DELAY_SECS, Arc::new(read_tsc)))).ok();
	calibrate();
	init_timer();
}

fn calibrate() {
	let clock = CLOCK.get().unwrap();
	let cycles = clock.read_cycles();
	clock.calibrate(cycles);
	// Record the start time at calibration moment for read_start_time()
	let _ = crate::START_TIME.set(crate::read());
}

pub(super) fn read_instant() -> Instant {
	CLOCK.get().unwrap().read_instant()
}

fn update_clocksource() {
	let clock = CLOCK.get().unwrap();
	clock.update();
	if let Some(update_fn) = VDSO_DATA_HIGH_RES_UPDATE_FN.get() {
		let (last_instant, last_cycles) = clock.last_record();
		update_fn(last_instant, last_cycles);
	}
}

fn init_timer() {
	let max_delay_secs = CLOCK.get().unwrap().max_delay_secs() >> 1;
	let delay_counts = TIMER_FREQ * max_delay_secs;
	let update = move || {
		let counter = TSC_UPDATE_COUNTER.fetch_add(1, Ordering::Relaxed);
		if counter % delay_counts == 0 { update_clocksource(); }
	};
	timer::register_callback(update);
}