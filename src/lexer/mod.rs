pub(crate) mod collection;
pub(crate) mod state;

#[macro_export]
macro_rules! build_state {
    ($collection:ident, $(($token:ident, $state:expr)),*) => {{
        use super::state::State;

        State::new(vec![$((Some(lookup_token!($collection, $token)), $state),)*])
    }};
}

#[macro_export]
macro_rules! build_final_state {
    () => {{
        let mut state = State::new(vec![]);
        state.mark_final();
        state
    }};
}

#[macro_export]
macro_rules! build_initial_states {
    ($($state:expr),*) => {{
        let mut state = State::new(vec![$((None, $state),)*]);
        state.mark_final();
        state
    }};
}
