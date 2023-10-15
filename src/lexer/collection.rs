use super::state::State;
use crate::{
    build_final_state, build_initial_states, build_skip_state, build_state, lookup_token,
    tokens::{collection::build_tokens, TokenMap},
};

pub(crate) fn build_state_machine<'a>() -> State<'a> {
    let mut tokens = TokenMap::default();
    build_tokens(&mut tokens);
    build_initial_states!(build_state!(
        tokens,
        (Whitespace, build_skip_state!()),
        (
            XBStart,
            build_state!(
                tokens,
                (
                    XBInside,
                    build_state!(tokens, (XBEnd, build_final_state!()))
                )
            )
        )
    ))
}
