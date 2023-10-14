#![allow(dead_code)]
use crate::tokens::Token;
use anyhow::{anyhow, Result};

pub(crate) type FollowingStates<'a> = Vec<(&'a dyn Token, State<'a>)>;

#[derive(Clone)]
pub(crate) struct State<'a> {
    name: &'a str,
    following: FollowingStates<'a>,
    end: bool,
}

impl<'a> State<'a> {
    pub fn new(name: &'a str, following: FollowingStates<'a>) -> Self {
        Self {
            name,
            following,
            end: false,
        }
    }

    pub fn mark_final(&mut self) {
        self.end = true;
    }

    pub fn is_final(&self) -> bool {
        self.end
    }

    pub fn scan(&self, input: &str, offset: usize) -> Result<(&dyn Token, State)> {
        if input.is_empty() {
            return Err(anyhow!("Premature end of input"));
        }

        let mut m = String::new();
        let mut s: Option<State> = None;
        let mut t: Option<&dyn Token> = None;

        for (token, state) in &self.following {
            if let Some(capture) = token.captured(input) {
                if capture.len() > m.len() {
                    m = capture;
                    s = Some(state.clone());
                    t = Some(*token);
                }
            }
        }

        if m.is_empty() {
            return Err(anyhow!("Syntax error in `{}` at offset {}", input, offset));
        }

        if let (Some(token), Some(state)) = (t, s) {
            return Ok((token, state));
        }

        return Err(anyhow!("Internal Error in Scanner"));
    }
}
