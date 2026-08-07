use crate::{Action, History, Strategy};

#[derive(Clone, Default)]
pub struct Pavlov;
impl Strategy for Pavlov {
    fn name(&self) -> &str {
        "Pavlov"
    }
    fn next_move(&mut self, history: &History) -> Action {
        match history.last() {
            Some(&(my, opp)) => {
                if my == opp {
                    Action::Cooperate
                } else {
                    Action::Defect
                }
            }
            None => Action::Cooperate,
        }
    }
    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
