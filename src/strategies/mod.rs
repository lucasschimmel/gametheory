use crate::{Action, FunctionalStrategy, History, Strategy};

pub mod always_cooperate;
pub mod always_defect;
pub mod grudger;
pub mod handshake;
pub mod pavlov;
pub mod tit_for_tat;
// pub mod tit_for_two_tats;
// pub mod suspicious_tit_for_tat;
// pub mod joss;
// pub mod tit_for_tat_with_forgiveness;
// pub mod statistical;
// pub mod alternator;
// pub mod detective;
// pub mod gradual;
// pub mod omega_tft;
// pub mod soft_grudger;
pub mod memory_n;
pub mod q_learning;

use self::memory_n::MemoryN;
use self::q_learning::QLearningStrategy;

pub fn get_all_strategies() -> Vec<Box<dyn Strategy>> {
    let mut strategies: Vec<Box<dyn Strategy>> = Vec::new();

    // Core Strategies
    strategies.push(Box::new(always_cooperate::AlwaysCooperate));
    strategies.push(Box::new(always_defect::AlwaysDefect));
    strategies.push(Box::new(tit_for_tat::TitForTat));
    strategies.push(Box::new(pavlov::Pavlov));
    strategies.push(Box::new(grudger::Grudger));

    // Memory-1 (Basic)
    strategies.push(Box::new(MemoryN::memory_1(
        1.0,
        0.0,
        1.0,
        0.0,
        Action::Cooperate,
        Some("TFT (as Memory-1)".to_string()),
    )));
    strategies.push(Box::new(MemoryN::memory_1(
        1.0,
        1.0,
        1.0,
        1.0,
        Action::Cooperate,
        Some("AlwaysCooperate (as Memory-1)".to_string()),
    )));
    strategies.push(Box::new(MemoryN::memory_1(
        0.0,
        0.0,
        0.0,
        0.0,
        Action::Defect,
        Some("AlwaysDefect (as Memory-1)".to_string()),
    )));

    // Stochastique / Recherche
    for p in [0.1, 0.5, 0.9] {
        for q in [0.1, 0.5, 0.9] {
            let name = format!("Stochastic Memory-1 (p={:.1}, q={:.1})", p, q);
            strategies.push(Box::new(MemoryN::memory_1(
                p,
                q,
                p,
                q,
                Action::Cooperate,
                Some(name),
            )));
        }
    }

    // Handshake
    strategies.push(Box::new(handshake::Handshake));

    // Pattern Matcher (Surgical)
    strategies.push(Box::new(FunctionalStrategy {
        name: "Pattern Matcher (W=4)".to_string(),
        next_move_fn: move |history: &History| {
            let window = 4;
            if history.len() < window * 2 {
                return Action::Cooperate;
            }
            let opp_history: Vec<_> = history.iter().map(|(_, opp)| *opp).collect();
            let last_pattern = &opp_history[opp_history.len() - window..];
            let prev_pattern =
                &opp_history[opp_history.len() - window * 2..opp_history.len() - window];
            if last_pattern == prev_pattern {
                let next_pred = last_pattern[0];
                if next_pred == Action::Cooperate {
                    Action::Defect
                } else {
                    Action::Defect
                }
            } else {
                opp_history.last().cloned().unwrap_or(Action::Cooperate)
            }
        },
    }));

    // Reinforcement Learning
    strategies.push(Box::new(QLearningStrategy::new(0.1, 0.9, 0.1)));
    strategies.push(Box::new(QLearningStrategy::new(0.5, 0.5, 0.2)));

    strategies
}
