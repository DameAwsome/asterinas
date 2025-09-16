fn main() {
    #[cfg(feature = "user_init")]
    aster_time::__init_for_user();
    let _ = aster_time::default_clocksource();
    let real = aster_time::get_real_time();
    let mono = aster_time::read_monotonic_time();
    println!(
        "real: {:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09}, mono_ns={}",
        real.year, real.month, real.day, real.hour, real.minute, real.second, real.nanos, mono.as_nanos()
    );
}

