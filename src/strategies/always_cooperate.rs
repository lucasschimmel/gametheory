use crate::{Action, History, Strategy};

#[derive(Clone, Default)]
pub struct AlwaysCooperate;
impl Strategy for AlwaysCooperate {
    fn name(&self) -> &str {
        "Always Cooperate"
    }
    fn next_move(&mut self, _: &History) -> Action {
        Action::Cooperate
    }
    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
