use crate::tokens::Token;
use anyhow::{anyhow, Result};

pub(crate) type FollowingStates<'a> = Vec<(Option<Token<'a>>, State<'a>)>;

#[derive(Clone)]
pub(crate) struct State<'a> {
    following: FollowingStates<'a>,
    skip: bool,
    // NOTE final is a keyword in rust, although I'm not sure what it's for, to be honest. Either
    // way this rename saves me a bunch of trouble.
    end: bool,
}

impl<'a> State<'a> {
    pub fn new(following: FollowingStates<'a>) -> Self {
        Self {
            following,
            skip: false,
            end: false,
        }
    }

    pub fn mark_final(&mut self) {
        self.end = true;
    }

    #[inline]
    pub fn is_final(&self) -> bool {
        self.end
    }

    pub fn mark_skip(&mut self) {
        self.skip = true;
    }

    #[inline]
    pub fn is_skip(&self) -> bool {
        self.end
    }

    pub fn scan(&self, input: &str, offset: usize) -> Result<(Option<Token>, State)> {
        if input.is_empty() && !self.is_skip() {
            return Err(anyhow!("Premature end of input"));
        }

        let mut m = String::new();
        let mut s: Option<State> = None;
        let mut t: Option<Token> = None;

        for (token, state) in &self.following {
            if let Some(token) = token {
                if let Some(capture) = token.captured(input) {
                    if capture.len() > m.len() {
                        m = capture;
                        s = Some(state.clone());
                        t = Some(token.clone());
                    }
                }
            } else {
                s = Some(state.clone());
            }
        }

        if m.is_empty() && !self.is_skip() {
            return Err(anyhow!("Syntax error in `{}` at offset {}", input, offset));
        }

        if let Some(state) = s {
            return Ok((t, state));
        }

        return Err(anyhow!("Internal Error in Scanner"));
    }
}
