use rand::Rng;

use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

use console::Emoji;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};

mod config;

static SPARKLE: Emoji<'_, '_> = Emoji("✨", ":-)");
static ROCKET: Emoji<'_, '_> = Emoji("🚀", "");
static GEAR: Emoji<'_, '_> = Emoji("⚙️", "");
static INFO: Emoji<'_, '_> = Emoji("ℹ️", "");

fn get_config() -> config::Config {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => match config::read_config(&args[1]) {
            Ok(config) => {
                println!(
                    "\n{}  Successfully loaded the config file! {}",
                    INFO, SPARKLE
                );
                println!(
                    "{}  L = {}, T = {}, min_p = {}, max_p = {}, p_step = {}\n",
                    GEAR,
                    config.lattice_size,
                    config.number_of_trails,
                    config.min_probability,
                    config.max_probability,
                    config.probability_step
                );
                config
            }
            Err(err) => panic!("Could not read the config: {}", err),
        },
        _ => panic!("Usage: percolation <config_file>"),
    }
}

#[allow(dead_code)]
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
    let started = Instant::now();
    let mut rng = rand::thread_rng();
    let config::Config {
        lattice_size,
        number_of_trails,
        min_probability,
        max_probability,
        probability_step,
    } = get_config();
    let output_file_path = format!("output/AveL{}T{}.txt", lattice_size, number_of_trails);
    fs::remove_file(&output_file_path).ok();
    let mut output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file_path)
        .unwrap();

    let mut p = min_probability;
    let count =
        (((max_probability - min_probability) / probability_step) as u64) * number_of_trails as u64;
    let pb = ProgressBar::new(count);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:42.cyan/blue} {msg}")
            .progress_chars("#>-"),
    );
    let mut i = 0;
    while p <= max_probability {
        pb.set_message(&format!("p={:.3}", &p));
        let burned: u32 = (0..number_of_trails)
            .into_iter()
            .map(|j| {
                let mut lattice: Vec<u8> = (0..lattice_size * lattice_size)
                    .map(|_| if rng.gen::<f32>() < p { 1 } else { 0 })
                    .collect();
                pb.set_position((i * number_of_trails + j) as u64);
                burn_dfs(&mut lattice, lattice_size) as u32
            })
            .sum();
        let p_flow = burned as f32 / number_of_trails as f32;
        writeln!(output_file, "{}", format!("{} {}", p, p_flow)).unwrap();
        p += probability_step;
        i += 1;
    }
    pb.finish_and_clear();
    println!(
        "{} Done in {} {}",
        SPARKLE,
        HumanDuration(started.elapsed()),
        ROCKET
    );
}
