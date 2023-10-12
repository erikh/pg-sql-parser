#![allow(dead_code)]
// this code is translated from the following url (which is HEAD in git at the time of this
// writing):
//
// https://git.postgresql.org/gitweb/?p=postgresql.git;a=blob;f=src/backend/parser/scan.l;h=0708ba65405309250613b53e38e7712fa97b286f;hb=HEAD
//
use super::Token; // this must be used for the `make_token` macro to function here (I tried rolling
                  // it into the macro and I made the compiler mad)
use crate::make_token;

// -- start of tokens --

// whitespace tokens
make_token!(Newline, r"[\n\r]");
make_token!(NonNewlineSpace, r"[ \f\v\t]");
make_token!(Space, r"[{}{}]", Newline, NonNewlineSpace);
make_token!(NonNewline, r"[^{}]", Newline);
make_token!(Comment, r"--[^{}]*", Newline);
make_token!(Whitespace, r"[{}]+|{}", Space, Comment);

// according to the source, SQL requires at least one newline in the whitespace separating string
// literals.
make_token!(SpecialWhitespace, r"[{}]+|{}{}", Space, Comment, Newline);
