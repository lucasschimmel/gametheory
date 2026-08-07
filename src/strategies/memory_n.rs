use crate::{Action, History, Strategy};
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MemoryN {
    pub name: String,
    pub memory: usize,
    pub table: HashMap<Vec<(Action, Action)>, Action>,
    pub initial_move: Action,
}

impl MemoryN {
    pub fn new(
        memory: usize,
        initial_move: Action,
        table: HashMap<Vec<(Action, Action)>, Action>,
    ) -> Self {
        Self {
            name: format!("Memory-{}", memory),
            memory,
            table,
            initial_move,
        }
    }

    pub fn memory_1(
        p_cc: f64,
        p_cd: f64,
        p_dc: f64,
        p_dd: f64,
        initial_move: Action,
        name: Option<String>,
    ) -> Self {
        let mut table = HashMap::new();
        let mut rng = rand::rng();

        table.insert(
            vec![(Action::Cooperate, Action::Cooperate)],
            if rng.random_bool(p_cc) {
                Action::Cooperate
            } else {
                Action::Defect
            },
        );
        table.insert(
            vec![(Action::Cooperate, Action::Defect)],
            if rng.random_bool(p_cd) {
                Action::Cooperate
            } else {
                Action::Defect
            },
        );
        table.insert(
            vec![(Action::Defect, Action::Cooperate)],
            if rng.random_bool(p_dc) {
                Action::Cooperate
            } else {
                Action::Defect
            },
        );
        table.insert(
            vec![(Action::Defect, Action::Defect)],
            if rng.random_bool(p_dd) {
                Action::Cooperate
            } else {
                Action::Defect
            },
        );

        Self {
            name: name.unwrap_or_else(|| {
                format!(
                    "Memory-1(cc={:.1}, cd={:.1}, dc={:.1}, dd={:.1})",
                    p_cc, p_cd, p_dc, p_dd
                )
            }),
            memory: 1,
            table,
            initial_move,
        }
    }
}

impl Strategy for MemoryN {
    fn name(&self) -> &str {
        &self.name
    }

    fn next_move(&mut self, history: &History) -> Action {
        if history.is_empty() {
            return self.initial_move;
        }

        let len = history.len();
        let start = if len >= self.memory {
            len - self.memory
        } else {
            0
        };
        let state = history[start..].to_vec();

        *self.table.get(&state).unwrap_or(&Action::Cooperate)
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
