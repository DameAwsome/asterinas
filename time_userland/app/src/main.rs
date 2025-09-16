fn main() {
    let _ = component::init_all(component::parse_metadata!());
    for _ in 0..10 {
        // simulate timer tick to update clocksource
        ostd::timer::simulate_tick();
    }
    let t = aster_time::get_real_time();
    println!("real: {:04}-{:02}-{:02} {:02}:{:02}:{:02}", t.year, t.month, t.day, t.hour, t.minute, t.second);
    let m = aster_time::read_monotonic_time();
    println!("mono secs: {}", m.as_secs());
}

