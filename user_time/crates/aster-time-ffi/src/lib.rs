#[no_mangle]
pub extern "C" fn time_init_all() {
	let _ = component::init_all(component::parse_metadata!());
}

#[no_mangle]
pub extern "C" fn time_read_real(year: &mut u16, month: &mut u8, day: &mut u8, hour: &mut u8, minute: &mut u8, second: &mut u8) {
	let t = aster_time::get_real_time();
	*year = t.year; *month = t.month; *day = t.day; *hour = t.hour; *minute = t.minute; *second = t.second;
}

#[no_mangle]
pub extern "C" fn time_read_monotonic_secs() -> u64 { aster_time::read_monotonic_time().as_secs() }

#[no_mangle]
pub extern "C" fn ostd_simulate_tick() { unsafe { extern "C" { fn ostd_simulate_tick(); } ostd_simulate_tick(); } }