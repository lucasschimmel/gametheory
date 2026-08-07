use crate::{Action, History, Strategy};

#[derive(Clone, Default)]
pub struct Handshake;
impl Strategy for Handshake {
    fn name(&self) -> &str {
        "Handshake"
    }
    fn next_move(&mut self, history: &History) -> Action {
        let turn = history.len();
        match turn {
            0 => Action::Cooperate,
            1 => Action::Defect,
            _ => {
                if history.len() >= 2
                    && history[0].1 == Action::Cooperate
                    && history[1].1 == Action::Defect
                {
                    history
                        .last()
                        .map(|(_, opp)| *opp)
                        .unwrap_or(Action::Cooperate)
                } else {
                    Action::Defect
                }
            }
        }
    }
    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
