use crate::{Action, History, Strategy};

#[derive(Clone, Default)]
pub struct AlwaysDefect;
impl Strategy for AlwaysDefect {
    fn name(&self) -> &str {
        "Always Defect"
    }
    fn next_move(&mut self, _: &History) -> Action {
        Action::Defect
    }
    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
