use super::{data::Data, state::State};

#[derive(Default)]
pub struct StateMachine {
    state: State,
    data: Data,
}

impl StateMachine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current_state(&self) -> &State {
        &self.state
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn data(&mut self) -> &mut Data {
        &mut self.data
    }
}
