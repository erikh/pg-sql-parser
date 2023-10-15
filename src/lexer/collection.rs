use super::state::State;
use crate::{
    build_final_state, build_initial_states, build_state, lookup_token,
    tokens::{collection::build_tokens, TokenMap},
};

pub(crate) fn build_state_machine<'a>() -> State<'a> {
    let mut tokens = TokenMap::default();
    build_tokens(&mut tokens);
    let end = build_final_state!();
    build_initial_states!(build_state!(tokens, (XBStart, end)))
}
