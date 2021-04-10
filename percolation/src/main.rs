use rand::Rng;
use std::env;

mod config;

fn get_config() -> config::Config {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => match config::read_config(&args[1]) {
            Ok(config) => config,
            Err(err) => panic!("Could not read the config: {}", err),
        },
        _ => panic!("Usage: percolation <config_file>"),
    }
}

fn print_lattice(lattice: &Vec<u8>, size: usize) {
    for i in 0..size {
        for j in 0..size {
            print!("{:?} ", lattice[i * size + j]);
        }
        println!();
    }
}

fn burn_dfs(lattice: &mut Vec<u8>, size: usize) -> bool {
    fn dfs(lattice: &mut Vec<u8>, i: usize, size: usize) -> bool {
        // corners
        let top_left = 0;
        let top_right = size - 1;
        let bottom_right = size * size - 1;
        let bottom_left = size * size - size;
        // if this is the last row, we found the spanning cluster
        if (i >= bottom_left) && (i <= bottom_right) {
            return true;
        }
        // get neighbors
        let neighbour_indices = match i {
            i if i == top_left => vec![size, 1],
            i if i == top_right => vec![top_right + size, top_right - 1],
            i if i >= top_left && i <= top_right => vec![i + size, i - 1, i + 1], // on the upper side
            i if i % size == 0 => vec![i + size, i + 1, i - size], // on the left side
            i if i / size == size - 1 => vec![i + size, i - 1, i - size], // on the right side
            _ => vec![i + size, i - 1, i + 1, i - size],           // in the middle
        };
        neighbour_indices.into_iter().any(|neighbour| {
            lattice[neighbour] == 1 && {
                lattice[neighbour] = 2;
                dfs(lattice, neighbour, size)
            }
        })
    }

    (0..size).into_iter().any(|i| {
        lattice[i] == 1 && {
            lattice[i] = 2;
            dfs(lattice, i, size)
        }
    })
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
            let mut lattice: Vec<u8> = (0..lattice_size * lattice_size)
                .map(|_| if rng.gen::<f32>() < p { 1 } else { 0 })
                .collect();
            p += probability_step;
            let _is_burned = burn_dfs(&mut lattice, lattice_size);
        }
    }
}
