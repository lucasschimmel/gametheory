use crate::{Action, History, Strategy};
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone)]
pub struct QLearningStrategy {
    pub name: String,
    pub q_table: HashMap<(Action, Action), [f64; 2]>, // (my_last, opp_last) -> [Q(C), Q(D)]
    pub last_state: Option<(Action, Action)>,
    pub last_action: Option<Action>,
    pub alpha: f64,   // Learning rate
    pub gamma: f64,   // Discount factor
    pub epsilon: f64, // Exploration rate
}

impl QLearningStrategy {
    pub fn new(alpha: f64, gamma: f64, epsilon: f64) -> Self {
        Self {
            name: format!("Q-Learning (a={:.1}, e={:.1})", alpha, epsilon),
            q_table: HashMap::new(),
            last_state: None,
            last_action: None,
            alpha,
            gamma,
            epsilon,
        }
    }

    fn action_to_idx(action: Action) -> usize {
        match action {
            Action::Cooperate => 0,
            Action::Defect => 1,
        }
    }
}

impl Strategy for QLearningStrategy {
    fn name(&self) -> &str {
        &self.name
    }

    fn next_move(&mut self, history: &History) -> Action {
        let mut rng = rand::rng();

        // Update Q-table from last move
        if let (Some(state), Some(action)) = (self.last_state, self.last_action) {
            if let Some(&(my_actual, opp_actual)) = history.last() {
                // Determine reward (payoff)
                let reward = match (my_actual, opp_actual) {
                    (Action::Cooperate, Action::Cooperate) => 3.0,
                    (Action::Cooperate, Action::Defect) => 0.0,
                    (Action::Defect, Action::Cooperate) => 5.0,
                    (Action::Defect, Action::Defect) => 1.0,
                };

                let next_state = (my_actual, opp_actual);
                let next_q_values = self.q_table.entry(next_state).or_insert([0.0, 0.0]);
                let max_next_q = next_q_values
                    .iter()
                    .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

                let current_q_values = self.q_table.entry(state).or_insert([0.0, 0.0]);
                let action_idx = Self::action_to_idx(action);

                // Q-Learning update rule
                current_q_values[action_idx] +=
                    self.alpha * (reward + self.gamma * max_next_q - current_q_values[action_idx]);
            }
        }

        // Choose next action
        let current_state = history
            .last()
            .cloned()
            .unwrap_or((Action::Cooperate, Action::Cooperate)); // Default state
        let q_values = self.q_table.entry(current_state).or_insert([0.0, 0.0]);

        let action = if rng.random_bool(self.epsilon) {
            // Explore
            if rng.random_bool(0.5) {
                Action::Cooperate
            } else {
                Action::Defect
            }
        } else {
            // Exploit
            if q_values[0] >= q_values[1] {
                Action::Cooperate
            } else {
                Action::Defect
            }
        };

        self.last_state = Some(current_state);
        self.last_action = Some(action);
        action
    }

    fn reset(&mut self) {
        self.last_state = None;
        self.last_action = None;
        // We keep the Q-table between matches if we want it to learn across matches,
        // but for tournament consistency, maybe we should reset it?
        // Actually, for Axelrod, strategies are usually reset between matches.
        self.q_table.clear();
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
