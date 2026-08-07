use crate::Action;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct ExperimentConfig {
    pub name: String,
    pub iterations: usize,
    pub action_noise: f64,
    pub perception_noise: f64,
    pub discount_factor: f64,
    pub payoffs: (i32, i32, i32, i32),
    pub seed: Option<u64>,
    pub generations: usize,
    pub reproduction_rate: f64,
    pub grid_size: usize,
}

impl ExperimentConfig {
    pub fn new() -> Self {
        Self {
            name: "Default Experiment".to_string(),
            iterations: 200,
            action_noise: 0.0,
            perception_noise: 0.0,
            discount_factor: 0.0,
            payoffs: (5, 3, 1, 0),
            seed: None,
            generations: 50,
            reproduction_rate: 0.2,
            grid_size: 20,
        }
    }
}

pub struct ExperimentResult {
    pub config: ExperimentConfig,
    pub final_distribution: HashMap<String, usize>,
    pub cooperation_history: Vec<f64>,
    pub entropy_history: Vec<f64>,
    pub population_history: Vec<HashMap<String, usize>>,
}

pub fn calculate_entropy(distribution: &HashMap<String, usize>) -> f64 {
    let total: usize = distribution.values().sum();
    if total == 0 {
        return 0.0;
    }
    let mut entropy = 0.0;
    for &count in distribution.values() {
        let p = count as f64 / total as f64;
        if p > 0.0 {
            entropy -= p * p.ln();
        }
    }
    entropy
}

pub fn calculate_cooperation_rate(history: &[(Action, Action)]) -> f64 {
    if history.is_empty() {
        return 0.0;
    }
    let total_moves = history.len() * 2;
    let mut c_moves = 0;
    for (a1, a2) in history {
        if *a1 == Action::Cooperate {
            c_moves += 1;
        }
        if *a2 == Action::Cooperate {
            c_moves += 1;
        }
    }
    c_moves as f64 / total_moves as f64
}
