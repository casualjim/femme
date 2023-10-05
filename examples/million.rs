use kv_log_macro as log;
use std::time::Instant;

fn main() {
    cj_femme::with_level(cj_femme::LevelFilter::Trace);

    let start = Instant::now();
    for n in 0..1_000_000 {
        log::info!("logging no. {}", n);
    }

    eprintln!("time elapsed: {:?}", Instant::now().duration_since(start));
}
