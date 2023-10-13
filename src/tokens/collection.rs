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

/*
 * -- whitespace tokens --
 */
make_token!(Newline, r"\n\r");
make_token!(NonNewlineSpace, r" \f\v\t");
make_token!(Space, r"[{}{}]", Newline, NonNewlineSpace);
make_token!(NonNewline, r"[^{}]", Newline);
make_token!(Comment, r"--{}*", NonNewline);
make_token!(Whitespace, r"[{}]+|{}", Space, Comment);

// according to the source, SQL requires at least one newline in the whitespace separating string
// literals.
make_token!(SpecialWhitespace, r"{}+|{}{}", Space, Comment, Newline);
make_token!(NonNewlineWhitespace, r"[{}]|{}", NonNewlineSpace, Comment);
make_token!(
    WhitespaceWithNewline,
    r"[{}]*[{}](?:{})*",
    NonNewlineSpace,
    Newline,
    SpecialWhitespace
);

/*
 * -- quoting tokens --
 */
make_token!(Quote, "'");
make_token!(QuoteContinue, "{}{}", WhitespaceWithNewline, Quote);
// this is a special token used to determine when quoting runs into a comment and thus fails to
// parse
make_token!(QuoteContinueFail, "(?:{})*-?", Whitespace);

/*
 * -- bit strings --
 */

// bytes
make_token!(XBStart, "[bB]{}", Quote);
make_token!(XBInside, "[^']*");

// hex
make_token!(XHStart, "[xX]{}", Quote);
make_token!(XHInside, "[^']*");

// national (?) characters
make_token!(XNStart, "[nN]{}", Quote);

// Quoted string with escaping
make_token!(XEStart, "[eE]{}", Quote);
make_token!(XEInside, "[^\\']+");
make_token!(XEEscape, "[\\][^0-7]");
make_token!(XEOctEscape, "[\\][0-7]{1,3}");
make_token!(XEHexEscape, "[\\]x[0-9a-fA-F]{1,2}");
make_token!(XEUnicode, r"[\\](?:u[0-9a-fA-F]{4}|U[0-9a-fA-F]{8})");
make_token!(
    XEUnicodeFail,
    r"[\\](?:u[0-9a-fA-F]{0,3}|U[0-9a-fA-F]{0,7})"
);

// Extended Quote
make_token!(XQStart, "[{}]", Quote);
make_token!(XQDouble, "[{}]{{2}}", Quote);
make_token!(XQInside, "[^']+");

// Dollar quoting
make_token!(DolQStart, r"[A-Za-z\200-\377_]");
make_token!(DolQContinue, r"[A-Za-z\200-\377_0-9]");
make_token!(DolQDelimiter, r"\$(?:{}{}*)?\$", DolQStart, DolQContinue);
make_token!(DolQFailed, r"\$(?:{}{}*", DolQStart, DolQContinue);
make_token!(DolQInside, r"[^$]+");

// Double quoting

make_token!(DoubleQuote, "\"");
make_token!(XDStart, "{}", DoubleQuote);
make_token!(XDStop, "{}", DoubleQuote);
make_token!(XDDouble, "{}{}", DoubleQuote, DoubleQuote);
make_token!(XDInside, "[^\"]+");

// Rules for Unicode Escapes

// Quoted Identifier with Unicode Escapes
make_token!(XUIStart, "[uU]&{}", DoubleQuote);
// Quoted String with Unicode Escapes
make_token!(XUSStart, "[uU]&{}", Quote);
// Error Rule
make_token!(XUFailed, "[uU]&");
