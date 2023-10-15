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
            // Comments
            XCStart,
            build_state!(
                tokens,
                (
                    XCInside,
                    build_state!(tokens, (XCStop, build_final_state!()))
                ),
                // to avoid matching operators by accident
                (OpChars, build_skip_state!())
            )
        ),
        (
            // bitstrings
            XBStart,
            build_state!(
                tokens,
                (
                    XBInside,
                    build_state!(tokens, (XBStop, build_final_state!()))
                )
            )
        ),
        (
            // hex strings
            XHStart,
            build_state!(
                tokens,
                (
                    XHInside,
                    build_state!(tokens, (XHStop, build_final_state!()))
                )
            )
        ),
        (
            // national strings
            XNStart,
            build_state!(
                tokens,
                (
                    XNInside,
                    build_state!(tokens, (XNStop, build_final_state!()))
                )
            )
        ),
        (
            // extended quoted strings
            // FIXME this probably doesn't work right, I need to give the lexer a way to support
            // epsilon moves
            XEStart,
            build_state!(
                tokens,
                (
                    XEEscape,
                    build_state!(tokens, (XEStop, build_final_state!()))
                ),
                (
                    XEOctEscape,
                    build_state!(tokens, (XEStop, build_final_state!()))
                ),
                (
                    XEHexEscape,
                    build_state!(tokens, (XEStop, build_final_state!()))
                ),
                (
                    XEUnicode,
                    build_state!(tokens, (XEStop, build_final_state!()))
                ),
                (
                    XEInside,
                    build_state!(tokens, (XEStop, build_final_state!()))
                )
            )
        ),
        (
            // quoted strings
            XQStart,
            build_state!(
                tokens,
                (
                    XQInside,
                    build_state!(tokens, (XQStop, build_final_state!()))
                ),
                (
                    XQDouble,
                    build_state!(tokens, (XQStop, build_final_state!()))
                )
            )
        )
    ))
}
