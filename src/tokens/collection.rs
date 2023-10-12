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
//
// In rust, {} is used similar to %s in sprintf(), etc from other languages. The item must
// implement the std::fmt::Display trait to be formatted, which the make_token provides an
// implementation for, which is why the definitions can be re-used here with format strings.
//

// whitespace tokens

// A couple of constant strings to make composing the character classes a little easier.

make_token!(Newline, r"\n\r");
make_token!(NonNewlineSpace, r" \f\v\t");
make_token!(Space, r"[{}|{}]", Newline, NonNewlineSpace);
make_token!(NonNewline, r"[^{}]", Newline);
make_token!(Comment, r"--{}*", NonNewline);
make_token!(Whitespace, r"[{}]+|{}", Space, Comment);

// according to the source, SQL requires at least one newline in the whitespace separating string
// literals.
make_token!(SpecialWhitespace, r"[{}]+|{}{}", Space, Comment, Newline);
