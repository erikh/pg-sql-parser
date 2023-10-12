pub(crate) mod collection;

// all tokens implement Token, by way of the `make_token` macro below.
pub(crate) trait Token {
    fn captured(&mut self, input: &str) -> bool;
    fn contained(&self) -> String;
    fn pattern(&self) -> &str;
}

// creates a token, takes a regex pattern (string literal). The first argument is the name of the
// token (must be translatable to a rust type syntactically), the second argument is a literal
// string used in formatting the regex, and the rest of the arguments are components that match
// that formatted string. See std::format in the rust stdlib for more information, as well as the
// comments in collection.rs in this directory.
//
// The second form is for literals which do not rely on other tokens. This allows us to bootstrap
// primitive tokens. Note, these are all converted to character classes when they are constructed,
// so you must "or" them to combine them together into one character class when using them with the
// more complex form. This is to prevent feeding the regex engine nested character classes, which
// are invalid.
//
// There's probably a good argument for better use of generics than this macro, but I think this
// reads better (and should be roughly the same speed).
#[macro_export]
macro_rules! make_token {
    ($name:ident, $match:literal, $($format:ident),*) => {
        $crate::make_token_struct!($name);

        impl $name {
            pub fn new() -> $name {
                let pattern = format!($match, $($format::new(),)*);

                Self {
                    pattern: pattern.clone(),
                    regex: regex::Regex::new(&format!("^({})", pattern)).unwrap(),
                    captured: String::new(),
                }
            }
        }
    };

    ($name:ident, $match:literal) => {
        $crate::make_token_struct!($name);

        impl $name {
            pub fn new() -> $name {
                let pattern = String::from($match);

                Self {
                    pattern: pattern.clone(),
                    regex: regex::Regex::new(&format!("^([{}])", pattern)).unwrap(),
                    captured: String::new(),
                }
            }
        }
    };
}

// this macro just makes it easier to construct the other macro without repetition.
#[macro_export]
macro_rules! make_token_struct {
    ($name:ident) => {
        pub(crate) struct $name {
            pattern: String,
            regex: regex::Regex,
            captured: String,
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.pattern())
            }
        }

        impl Token for $name {
            fn captured(&mut self, input: &str) -> bool {
                if let Some(cap) = self.regex.captures(input) {
                    self.captured = cap[1].to_string();
                    return true;
                }

                false
            }

            fn contained(&self) -> String {
                self.captured.clone()
            }

            fn pattern(&self) -> &str {
                self.pattern.as_str()
            }
        }
    };
}
