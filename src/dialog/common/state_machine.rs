use super::{data::Data, state::State};

use crate::sources::base::Source;

use std::rc::Rc;

pub struct StateMachine<S>
where
    S: Source,
{
    previous_states: Vec<Rc<State>>,
    state: Rc<State>,
    data: Data<S>,
}

impl<S> StateMachine<S>
where
    S: Source,
{
    #[must_use]
    pub fn new<St>(state: St, data: Data<S>) -> Self
    where
        St: Into<Rc<State>>,
    {
        Self {
            previous_states: vec![],
            state: state.into(),
            data,
        }
    }

    #[must_use]
    pub fn current_state(&self) -> &State {
        &self.state
    }

    /// Set next state and save current state as previous
    pub fn set_state(&mut self, state: State) {
        self.previous_states.push(Rc::clone(&self.state));
        self.state = Rc::new(state);
    }

    /// Set previous state and truncate states after this state
    pub fn set_previous_state_and_truncate_next(&mut self, state: State) {
        let mut seq_state_num = None;

        for (seq_num, previous_state) in self.previous_states.iter().enumerate() {
            if previous_state.as_ref() == &state {
                seq_state_num = Some(seq_num);
                break;
            }
        }

        if let Some(seq_state_num) = seq_state_num {
            self.previous_states.truncate(seq_state_num);
            self.state = Rc::new(state);
        }
    }

    pub fn set_previous_state(&mut self) {
        if let Some(state) = self.previous_states.pop() {
            // Set previous state as current state
            self.state = state;
        }
    }

    /// Get data
    #[must_use]
    pub fn data(&mut self) -> &mut Data<S> {
        &mut self.data
    }
}

impl<S> Default for StateMachine<S>
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
