fn main() {
	let _ = component::init_all(component::parse_metadata!());

	// Simulate some ticks to advance the clocksource
	for _ in 0..10 { ostd::timer::simulate_tick(); }

	let t1 = aster_time::get_real_time();
	println!("get_real_time: {:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09}", t1.year, t1.month, t1.day, t1.hour, t1.minute, t1.second, t1.nanos);

	let t2 = aster_time::read();
	println!("read: {:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09}", t2.year, t2.month, t2.day, t2.hour, t2.minute, t2.second, t2.nanos);

	// START_TIME is set during calibration in tsc::init()
	let ts = aster_time::read_start_time();
	println!("read_start_time: {:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09}", ts.year, ts.month, ts.day, ts.hour, ts.minute, ts.second, ts.nanos);

	let mono = aster_time::read_monotonic_time();
	println!("read_monotonic_time: {}s {}ns", mono.as_secs(), mono.subsec_nanos());

	let clock = aster_time::default_clocksource();
	let cycles = clock.read_cycles();
	println!("default_clocksource.read_cycles: {}", cycles);
	let last = clock.last_record();
	println!("default_clocksource.last_record: instant=({}s, {}ns) cycles={}", last.0.secs(), last.0.nanos(), last.1);
}