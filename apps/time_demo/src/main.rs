// SPDX-License-Identifier: MPL-2.0

fn main() {
    env_logger::init();
    user_time::init();

    let start = user_time::get_real_time();
    println!(
        "start: {:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09}",
        start.year, start.month, start.day, start.hour, start.minute, start.second, start.nanos
    );

    let mono0 = user_time::read_monotonic_time();
    std::thread::sleep(std::time::Duration::from_millis(150));
    let mono1 = user_time::read_monotonic_time();
    println!(
        "monotonic delta: {} ms",
        (mono1 - mono0).as_millis()
    );

    let instant = user_time::read_instant();
    println!("instant: {}s {}ns", instant.secs(), instant.nanos());

    let trace = user_time::take_trace();
    println!("trace entries: {}", trace.len());
    for line in trace {
        println!("trace: {}", line);
    }
}

