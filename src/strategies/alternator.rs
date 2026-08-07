/// Alternator: Alternates between Cooperation and Defection at every turn.
use crate::{Action, Strategy, History};

#[derive(Clone, Default)]
pub struct Alternator;
impl Strategy for Alternator {
    fn name(&self) -> &str { "Alternator" }
    fn next_move(&mut self, history: &History) -> Action {
        if history.len() % 2 == 0 { Action::Cooperate } else { Action::Defect }
    }
    fn clone_box(&self) -> Box<dyn Strategy> { Box::new(self.clone()) }
}
