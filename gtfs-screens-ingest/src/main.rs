use std::{env, time::Instant};

mod sfbart;

fn main() {
    let start = Instant::now();

    let argv: Vec<String> = env::args().collect();

    if argv.len() < 2 {
        panic!(
            "Expected 1 arg (destination path) but got {} args instead.",
            argv.len()
        )
    }

    let path = std::path::Path::new(&argv[1]);

    if !path.exists() {
        panic!(
            "Provided path {} does not exist on the system.",
            &path.to_str().unwrap()
        );
    }

    sfbart::ingest(&path);

    println!(
        "\x1b[34mIngest job took \x1b[33m{:?}\x1b[34m to complete.\x1b[0m",
        start.elapsed()
    );
}
