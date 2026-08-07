/// Detective: Starts with a fixed sequence [C, D, C, C]. 
/// If the opponent never defects during this phase, it defects forever to exploit them.
/// If the opponent defects, it switches to Tit For Tat.
use crate::{Action, Strategy, History};

#[derive(Clone, Default)]
pub struct Detective;
impl Strategy for Detective {
    fn name(&self) -> &str { "Detective" }
    fn next_move(&mut self, history: &History) -> Action {
        let turn = history.len();
        let opening = [Action::Cooperate, Action::Defect, Action::Cooperate, Action::Cooperate];
        
        if turn < opening.len() {
            return opening[turn];
        }
        
        let opponent_defected = history.iter().any(|(_, opp_a)| *opp_a == Action::Defect);
        if !opponent_defected {
            Action::Defect
        } else {
            history.last().map(|(_, opp_a)| *opp_a).unwrap_or(Action::Cooperate)
        }
    }
    fn clone_box(&self) -> Box<dyn Strategy> { Box::new(self.clone()) }
}
