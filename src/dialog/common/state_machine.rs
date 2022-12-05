use crate::sources::base::Source;

use super::{data::Data, state::State};

pub struct StateMachine<'a, S>
where
    S: Source,
{
    previous_states: Vec<State>,
    state: State,
    data: Data<'a, S>,
}

impl<'a, S> StateMachine<'a, S>
where
    S: Source,
{
    #[must_use]
    pub fn new(state: State, data: Data<'a, S>) -> Self {
        Self {
            previous_states: Vec::new(),
            state,
            data,
        }
    }

    /// Get current state
    #[must_use]
    pub fn current_state(&self) -> &State {
        &self.state
    }

    /// Set next state and save current state as previous
    pub fn set_state(&mut self, state: State) {
        self.previous_states.push(self.state.clone());
        self.state = state;
    }

    /// Go to previous state
    pub fn set_previous_state(&mut self) {
        // Get last previous state
        if let Some(state) = self.previous_states.pop() {
            // Set previous state as current state
            self.state = state;
        }
    }

    /// Get data
    #[must_use]
    pub fn data(&mut self) -> &mut Data<'a, S> {
        &mut self.data
    }
}

impl<S> Default for StateMachine<'_, S>
where
    S: Source,
{
    fn default() -> Self {
        Self {
            previous_states: Vec::default(),
            state: State::default(),
            data: Data::default(),
        }
    }
}
