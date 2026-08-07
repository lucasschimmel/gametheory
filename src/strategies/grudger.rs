use crate::{Action, History, Strategy};

#[derive(Clone, Default)]
pub struct Grudger;
impl Strategy for Grudger {
    fn name(&self) -> &str {
        "Grudger"
    }
    fn next_move(&mut self, history: &History) -> Action {
        if history.iter().any(|(_, opp)| *opp == Action::Defect) {
            Action::Defect
        } else {
            Action::Cooperate
        }
    }
    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
