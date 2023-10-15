use std::collections::BTreeMap;

pub(crate) mod collection;

pub(crate) type TokenMap<'a> = BTreeMap<&'a str, Token<'a>>;

#[derive(Clone)]
pub(crate) struct Token<'a> {
    name: &'a str,
    pattern: String,
    regex: regex::Regex,
}

impl<'a> Token<'a> {
    pub fn initial(name: &'a str, pattern: String) -> Self {
        Self {
            name,
            pattern: pattern.clone(),
            regex: regex::Regex::new(&format!("^([{}])", pattern)).unwrap(),
        }
    }

    pub fn new(name: &'a str, pattern: String) -> Self {
        Self {
            name,
            pattern: pattern.clone(),
            regex: regex::Regex::new(&format!("^({})", pattern)).unwrap(),
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn captured(&self, input: &str) -> Option<String> {
        if let Some(cap) = self.regex.captures(input) {
            return Some(cap[1].to_string());
        }

        None
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.pattern())
    }
}

#[macro_export]
macro_rules! make_token {
    ($collection:ident, $name:ident, $match:literal, $($format:ident),*) => {
        $collection.insert("$name", Token::new("$name", format!($match, $($crate::lookup_token!($collection, $format),)*)));
    };

    ($collection:ident, $name:ident, $match:literal) => {
        $collection.insert("$name", Token::initial("$name", String::from($match)));
    };
}

#[macro_export]
macro_rules! lookup_token {
    ($collection:ident, $name:ident) => {
        $collection["$name"].clone()
    };
}
