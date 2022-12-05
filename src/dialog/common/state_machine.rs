use super::{data::Data, state::State};

use crate::sources::base::Source;

use std::rc::Rc;

pub struct StateMachine<'a, S>
where
    S: Source,
{
    previous_states: Vec<Rc<State>>,
    state: Rc<State>,
    data: Data<'a, S>,
}

impl<'a, S> StateMachine<'a, S>
where
    S: Source,
{
    #[must_use]
    pub fn new<St>(state: St, data: Data<'a, S>) -> Self
    where
        St: Into<Rc<State>>,
    {
        Self {
            previous_states: Vec::new(),
            state: state.into(),
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
        self.previous_states.push(Rc::clone(&self.state));
        self.state = Rc::new(state);
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
    #[must_use]
    fn default() -> Self {
        Self {
            previous_states: Vec::default(),
            state: Rc::new(State::default()),
            data: Data::default(),
        }
    }
}
