use rand::Rng;
use std::env;
use std::process;

mod config;

fn help() {
    println!("Usage: percolation <config_file>");
}

fn get_config() -> config::Config {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => match config::read_config(&args[1]) {
            Ok(config) => config,
            Err(err) => panic!("Could not read the config: {}", err),
        },
        _ => {
            help();
            process::exit(1)
        }
    }
}

fn main() {
    let config::Config {
        lattice_size,
        number_of_trails,
        min_probability,
        max_probability,
        probability_step,
    } = get_config();
    let mut rng = rand::thread_rng();

    for _ in 0..number_of_trails {
        let mut p = min_probability;
        while p <= max_probability {
            let _lattice: Vec<u8> = (0..lattice_size)
                .map(|_| if rng.gen::<f32>() < p { 1 } else { 0 })
                .collect();
            p += probability_step;
        }
    }
}

