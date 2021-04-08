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

// Detects whether a spanning cluster exists in a given lattice
fn burning_method(mut lattice: Vec<u32>, size: usize) -> bool {
    // Label all occupied cells in the top line with the marker t = 2
    let mut last_burned_indices: Vec<usize> = Vec::with_capacity(size);
    for i in 0..size {
        if lattice[i] == 1 {
            last_burned_indices.push(i);
            lattice[i] = 2;
        }
    }
    // corners
    let top_left = 0;
    let top_right = size - 1;
    let bottom_right = size * size - 1;
    let bottom_left = bottom_right - size;
    // iteration step
    let mut t = 2;
    loop {
        let mut burning_indices: Vec<usize> = Vec::with_capacity(size);
        for i in last_burned_indices {
            // if this is the last row, we found the spanning cluster
            if (i >= bottom_left) && (i <= bottom_right) {
                return true;
            }
            // get neighbors
            let neighbour_indices = match i {
                i if i == top_left => vec![1, size],
                i if i == top_right => vec![top_right - 1, top_right + size],
                i if i >= top_left && i <= top_right => vec![i - 1, i + size, i + 1],
                i if i % size == 0 => vec![i - size, i + 1, i + size],
                i if i / size == size - 1 => vec![i - size, i - 1, i + size],
                _ => vec![i - 1, i + 1, i - size, i + size],
            };
            // burn neighbors
            for index in neighbour_indices {
                if lattice[index] == 1 {
                    lattice[index] = t + 1;
                    burning_indices.push(index);
                }
            }
        }
        if burning_indices.is_empty() {
            return false;
        }
        last_burned_indices = burning_indices;
        t += 1;
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
            let lattice: Vec<u32> = (0..lattice_size * lattice_size)
                .map(|_| if rng.gen::<f32>() < p { 1 } else { 0 })
                .collect();
            p += probability_step;
            burning_method(lattice, lattice_size);
        }
    }
}
