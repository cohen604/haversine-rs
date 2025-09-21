use std::{fmt::Display, fs, io::Write};

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand, ValueEnum};
use rand::{
    Rng, SeedableRng,
    distr::{Distribution, Uniform},
    rngs::StdRng,
};

use crate::haversine::reference_haversine;

mod haversine;

// Using the same as in the computer enhance code to get similar results
const EARTH_RADIUS: f64 = 6372.8;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        #[arg(value_enum)]
        method: GenerationMethod,
        #[arg(long)]
        seed: u64,
        #[arg(long)]
        size: u64,
    },
}

#[derive(Clone, ValueEnum)]
enum GenerationMethod {
    Cluster,
    Uniform,
}

#[derive(Debug)]
struct Pair {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"x0\":{}, \"y0\":{}, \"x1\":{}, \"y1\":{}}}",
            self.x0, self.y0, self.x1, self.y1
        )
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { seed, size, method } => {
            generate_coordinate_pairs(seed, size, method).unwrap()
        }
    };
}

fn generate_random_quadrants(rng: &mut StdRng) -> Result<(Uniform<f64>, Uniform<f64>)> {
    let max_diff = 20;

    let x_low = rng.random_range(-180..160);
    let x_high = rng.random_range(x_low + 1..=x_low + max_diff);

    let y_low = rng.random_range(-90..70);
    let y_high = rng.random_range(y_low + 1..=y_low + max_diff);

    Ok((
        Uniform::new(x_low as f64, x_high as f64)?,
        Uniform::new(y_low as f64, y_high as f64)?,
    ))
}

fn generate_coordinate_pairs(seed: u64, size: u64, method: GenerationMethod) -> Result<()> {
    let mut rng = StdRng::seed_from_u64(seed);
    let x_rng = Uniform::new(-180.0, 180.0)?;
    let y_rng = Uniform::new(-90.0, 90.0)?;

    let mut file = fs::File::create_new("test.json")?;
    file.write_all(b"{\"pairs\": [\n")?;

    let mut i = 0;
    let mut sum: f64 = 0.0;
    let sum_coef = 1.0 / size as f64;

    match method {
        GenerationMethod::Cluster => {
            let batch = 5000;
            let (mut x_rng, mut y_rng) = generate_random_quadrants(&mut rng)?;
            while i < size {
                if i % batch == 0 {
                    (x_rng, y_rng) = generate_random_quadrants(&mut rng)?;
                }

                let pair = generate_single_pair(x_rng, y_rng, &mut rng)?;
                let haversine =
                    reference_haversine(pair.x0, pair.y0, pair.x1, pair.y1, EARTH_RADIUS)?;
                let sep = if i == size - 1 { "\n" } else { ",\n" };
                file.write_all(format!("{pair}{sep}").as_bytes());

                sum += sum_coef * haversine;
                i += 1;
            }
        }
        GenerationMethod::Uniform => {
            while i < size {
                let pair = generate_single_pair(x_rng, y_rng, &mut rng)?;
                let haversine =
                    reference_haversine(pair.x0, pair.y0, pair.x1, pair.y1, EARTH_RADIUS)?;
                let sep = if i == size - 1 { "\n" } else { ",\n" };
                file.write_all(format!("{pair}{sep}").as_bytes());

                sum += sum_coef * haversine;
                i += 1;
            }
        }
    }
    file.write_all(b"]}")?;

    println!("Seed: {}", seed);
    println!("Pair count: {}", size);
    println!("Expected Sum: {}", sum);

    Ok(())
}

fn generate_single_pair(
    x_rng: Uniform<f64>,
    y_rng: Uniform<f64>,
    rng: &mut StdRng,
) -> Result<Pair> {
    let x0 = x_rng.sample(rng);
    let x1 = x_rng.sample(rng);
    let y0 = y_rng.sample(rng);
    let y1 = y_rng.sample(rng);

    Ok(Pair { x0, y0, x1, y1 })
}
