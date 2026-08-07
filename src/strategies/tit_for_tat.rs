/// Tit For Tat: The classic reciprocal strategy. Starts with Cooperation, then mimics the opponent's last move.
use crate::{Action, History, Strategy};

#[derive(Clone, Default)]
pub struct TitForTat;
impl Strategy for TitForTat {
    fn name(&self) -> &str {
        "Tit For Tat"
    }
    fn next_move(&mut self, history: &History) -> Action {
        history
            .last()
            .map(|(_, opp_a)| *opp_a)
            .unwrap_or(Action::Cooperate)
    }
    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
