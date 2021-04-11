use rand::Rng;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

use console::Emoji;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};

mod config;

static SPARKLE: Emoji<'_, '_> = Emoji("✨", ":)");
static ROCKET: Emoji<'_, '_> = Emoji("🚀", ":o");
static GEAR: Emoji<'_, '_> = Emoji("⚙️", ":)");

fn get_config() -> config::Config {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => match config::read_config(&args[1]) {
            Ok(config) => {
                println!(
                    "\n{} Successfully loaded the config file! {}",
                    SPARKLE, SPARKLE
                );
                let prob_format = match config.mode {
                    config::Mode::Ave => format!(
                        "{{{}, {}, {}, ..., {}}}",
                        config.min_probability,
                        config.min_probability + config.probability_step,
                        config.min_probability + config.probability_step * 2.0,
                        config.max_probability
                    ),
                    config::Mode::Dist => {
                        format!(
                            "{}  mode: {:?}\n{}  L = {}, T = {}, p in {:?}\n",
                            GEAR,
                            config.mode,
                            GEAR,
                            config.lattice_size,
                            config.number_of_trails,
                            config.probabilities
                        )
                    }
                };

                println!(
                    "{}  mode: {:?}\n{}  L = {}, T = {}, p in {}\n",
                    GEAR,
                    config.mode,
                    GEAR,
                    config.lattice_size,
                    config.number_of_trails,
                    prob_format,
                );
                config
            }
            Err(err) => panic!("Could not read the config: {}", err),
        },
        _ => panic!("Usage: percolation <config_file>"),
    }
}

#[allow(dead_code)]
fn print_lattice(lattice: &Vec<usize>, size: usize) {
    for i in 0..size {
        for j in 0..size {
            print!("{:?} ", lattice[i * size + j]);
        }
        println!();
    }
}

fn burn_dfs(lattice: &mut Vec<usize>, size: usize) -> bool {
    // corners
    let top_left = 0;
    let top_right = size - 1;
    let bottom_right = size * size - 1;
    let bottom_left = size * size - size;

    (0..size).into_iter().any(|i| {
        let mut stack: Vec<usize> = Vec::with_capacity(size * size / 2 as usize);
        stack.push(i);
        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            if lattice[current] == 1 {
                lattice[current] = 2;
                match current {
                    // if this is the last row, we found the spanning cluster
                    i if (i >= bottom_left) && (i <= bottom_right) => return true,
                    // top-left corner
                    i if i == top_left => stack.extend(vec![1, size]),
                    // top-right corner
                    i if i == top_right => stack.extend(vec![top_right - 1, top_right + size]),
                    // on the upper side
                    i if i > top_left && i < top_right => {
                        stack.extend(vec![i - 1, i + 1, i + size])
                    }
                    // on the left side
                    i if i % size == 0 => stack.extend(vec![i - size, i + 1, i + size]),
                    // on the right side
                    i if i / size == size - 1 => stack.extend(vec![i - size, i - 1, i + size]),
                    // in the middle
                    i => stack.extend(vec![i - size, i - 1, i + 1, i + size]),
                };
            }
        }
        false
    })
}

fn hoshen_kopelman(lattice: &mut Vec<usize>, size: usize) -> Vec<i32> {
    // corners
    let top_left = 0;
    let top_right = size - 1;
    let mut k: usize = 2;

    let mut m: Vec<i32> = vec![0; (size * size / 2 + 3) as usize];
    for e in lattice.iter_mut() {
        if *e == 1 {
            *e = k;
            m[k] = 1;
            break;
        }
    }
    if m.is_empty() {
        return m;
    }

    for i in 0..size * size {
        if lattice[i] == 1 {
            let mut neighbors: Vec<usize> = match i {
                i if i == top_right => vec![top_right - 1], // top right corner
                i if i > top_left && i < top_right => vec![i - 1], // on the upper side
                i if i % size == 0 => vec![i - size],       // on the left side
                _ => vec![i - 1, i - size],                 // in the middle
            };
            neighbors = neighbors.into_iter().filter(|i| lattice[*i] != 0).collect();
            match neighbors.len() {
                0 => {
                    k += 1;
                    lattice[i] = k;
                    m[k] = 1;
                }
                1 => {
                    let mut k0 = lattice[neighbors.pop().unwrap()];
                    while m[k0] < 0 {
                        k0 = (-1 * m[k0]) as usize;
                    }
                    m[k0] += 1;
                    lattice[i] = k0;
                }
                2 => {
                    let mut k1 = lattice[neighbors.pop().unwrap()];
                    let mut k2 = lattice[neighbors.pop().unwrap()];

                    while m[k1] < 0 {
                        k1 = (-1 * m[k1]) as usize;
                    }

                    while m[k2] < 0 {
                        k2 = (-1 * m[k2]) as usize;
                    }
                    if k1 != k2 {
                        lattice[i] = k1;
                        m[k1] += m[k2] + 1;
                        m[k2] = -1 * (k1 as i32);
                    } else {
                        lattice[i] = k1;
                        m[k1] += 1;
                    }
                }
                _ => panic!("This will never happen"),
            }
        }
    }
    m
}

fn reset_lattice(lattice: &mut Vec<usize>) {
    for e in lattice.iter_mut() {
        if *e == 2 {
            *e = 1;
        }
    }
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
        mode,
        probabilities,
    } = get_config();
    match mode {
        config::Mode::Ave => {
            let output_file_path = format!("output/Ave_L{}T{}.txt", lattice_size, number_of_trails);
            fs::remove_file(&output_file_path).ok();
            let mut output_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(output_file_path)
                .unwrap();
            let mut p = min_probability;
            let count = (((max_probability - min_probability) / probability_step) as u64)
                * number_of_trails as u64;
            let pb = ProgressBar::new(count);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:42.cyan/blue} {msg}")
                    .progress_chars("#>-"),
            );
            let mut i = 0;
            while p <= max_probability {
                let mut burned = 0;
                let mut sum_s_max = 0;
                for j in 0..number_of_trails {
                    pb.set_message(&format!("p={:.3} [{}/{}]", &p, j, number_of_trails));
                    let mut lattice: Vec<usize> = (0..lattice_size * lattice_size)
                        .map(|_| if rng.gen::<f32>() < p { 1 } else { 0 })
                        .collect();
                    burned += burn_dfs(&mut lattice, lattice_size) as u32;
                    reset_lattice(&mut lattice);
                    let m = hoshen_kopelman(&mut lattice, lattice_size);
                    sum_s_max += (&m).iter().max().unwrap();
                    pb.set_position((i * number_of_trails + j) as u64);
                }
                let p_flow = burned as f32 / number_of_trails as f32;
                let avg_s_max = sum_s_max as f32 / number_of_trails as f32;
                writeln!(output_file, "{}", format!("{} {} {}", p, p_flow, avg_s_max)).unwrap();
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
        config::Mode::Dist => {
            let pb = ProgressBar::new((probabilities.len() as u32 * number_of_trails) as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:42.cyan/blue} {msg}")
                    .progress_chars("#>-"),
            );
            for (i, p) in probabilities.iter().enumerate() {
                let output_file_path = format!(
                    "output/Dist_p{}L{}T{}.txt",
                    p, lattice_size, number_of_trails
                );
                fs::remove_file(&output_file_path).ok();
                let mut output_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(output_file_path)
                    .unwrap();
                let mut dist: HashMap<i32, i32> = HashMap::new();
                for j in 0..number_of_trails {
                    pb.set_message(&format!("p={} [{}/{}]", &p, j, number_of_trails));
                    pb.set_position(((i as u32) * number_of_trails + j) as u64);
                    let mut lattice: Vec<usize> = (0..lattice_size * lattice_size)
                        .map(|_| if rng.gen::<f32>() < *p { 1 } else { 0 })
                        .collect();
                    for k in hoshen_kopelman(&mut lattice, lattice_size).iter() {
                        if *k > 0 {
                            let entry = dist.entry(*k).or_insert(0);
                            *entry += 1;
                        }
                    }
                }
                for (s, n) in dist {
                    writeln!(output_file, "{}", format!("{} {}", s, n)).unwrap();
                }
            }
            pb.finish_and_clear();
            println!(
                "{} Done in {} {}",
                SPARKLE,
                HumanDuration(started.elapsed()),
                ROCKET
            );
        }
    }
}
