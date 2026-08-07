use clap::Parser;
use game_theory::{
    Game, SpatialTournament, Strategy, Tournament,
    experiment::{ExperimentConfig, calculate_entropy},
    strategies,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 200)]
    iterations: usize,

    #[arg(long, default_value_t = 0.0)]
    action_noise: f64,

    #[arg(long, default_value_t = 0.0)]
    perception_noise: f64,

    #[arg(long, default_value_t = 0.0)]
    discount_factor: f64,

    #[arg(short, long, default_value_t = 1)]
    repetitions: usize,

    #[arg(long)]
    evolution: bool,

    #[arg(long, default_value_t = 50)]
    generations: usize,

    #[arg(long, default_value_t = 0.2)]
    reproduction_rate: f64,

    #[arg(long)]
    spatial: bool,

    #[arg(long, default_value_t = 20)]
    grid_size: usize,

    #[arg(long)]
    seed: Option<u64>,

    #[arg(long)]
    json_output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let config = ExperimentConfig {
        name: "IPD Experiment".to_string(),
        iterations: args.iterations,
        action_noise: args.action_noise,
        perception_noise: args.perception_noise,
        discount_factor: args.discount_factor,
        payoffs: (5, 3, 1, 0),
        seed: args.seed,
        generations: args.generations,
        reproduction_rate: args.reproduction_rate,
        grid_size: args.grid_size,
    };

    println!("Starting Research-Grade Axelrod Tournament...");
    println!("Config: {}", serde_json::to_string_pretty(&config).unwrap());

    let game = Game {
        iterations: config.iterations,
        action_noise: config.action_noise,
        perception_noise: config.perception_noise,
        discount_factor: config.discount_factor,
        payoffs: config.payoffs,
        seed: config.seed,
    };

    let strategies = strategies::get_all_strategies();

    if args.spatial {
        run_spatial_experiment(&config, strategies, game);
    } else if args.evolution {
        run_evolutionary_experiment(&config, strategies, game, args.json_output);
    } else {
        run_standard_experiment(strategies, game);
    }
}

fn run_spatial_experiment(
    config: &ExperimentConfig,
    strategies: Vec<Box<dyn Strategy>>,
    game: Game,
) {
    println!("\nRunning Spatial Experiment...");
    let mut spatial_tournament =
        SpatialTournament::new(config.grid_size, config.grid_size, strategies, game);

    for generation in 0..config.generations {
        spatial_tournament.step();
        if generation % 10 == 0 {
            let counts = spatial_tournament.get_population_counts();
            let entropy = calculate_entropy(&counts);
            println!("Gen {}: Entropy = {:.4}", generation, entropy);
        }
    }

    let final_counts = spatial_tournament.get_population_counts();
    display_population_results(&final_counts);
}

fn run_evolutionary_experiment(
    config: &ExperimentConfig,
    strategies: Vec<Box<dyn Strategy>>,
    game: Game,
    json_path: Option<String>,
) {
    println!("\nRunning Evolutionary Experiment...");
    let mut tournament = Tournament::new(strategies, game);
    let (_, history) = tournament.run_evolution(config.generations, config.reproduction_rate);

    println!("\nEvolution Metrics:");
    println!("{:<10} | {:<10} | {:<10}", "Gen", "Entropy", "Diversity");
    for (generation, counts) in history.iter().enumerate() {
        if generation % 10 == 0 || generation == history.len() - 1 {
            let entropy = calculate_entropy(&counts);
            let diversity = counts.len();
            println!("{:<10} | {:<10.4} | {:<10}", generation, entropy, diversity);
        }
    }

    if let Some(path) = json_path {
        let json = serde_json::to_string_pretty(&history).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    let final_counts = history.last().unwrap();
    display_population_results(final_counts);
}

fn run_standard_experiment(strategies: Vec<Box<dyn Strategy>>, game: Game) {
    let tournament = Tournament::new(strategies, game);
    let results = tournament.run_round_robin();

    let mut sorted_results: Vec<_> = results.iter().collect();
    sorted_results.sort_by(|a, b| b.1.cmp(a.1));

    println!("\nFinal Scores:");
    for (name, score) in sorted_results.iter().take(20) {
        println!("{:<40} | {}", name, score);
    }
}

fn display_population_results(counts: &HashMap<String, usize>) {
    let mut sorted_counts: Vec<_> = counts.iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(a.1));

    println!("\nFinal Population Distribution:");
    for (name, count) in sorted_counts.iter().take(20) {
        println!("{:<40} | {} individuals", name, count);
    }
}
