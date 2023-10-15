// this code is translated from the following url (which is HEAD in git at the time of this
// writing):
//
// https://git.postgresql.org/gitweb/?p=postgresql.git;a=blob;f=src/backend/parser/scan.l;h=0708ba65405309250613b53e38e7712fa97b286f;hb=HEAD
//
use super::{Token, TokenMap};
use crate::make_token;

// -- start of tokens --
//
// In rust, {} is used similar to %s in sprintf(), etc from other languages. The item must
// implement the std::fmt::Display trait to be formatted, which the make_token provides an
// implementation for, which is why the definitions can be re-used here with format strings.
//
pub(crate) fn build_tokens<'a>(tokens: &mut TokenMap<'a>) {
    /*
     * -- whitespace tokens --
     */
    make_token!(tokens, Newline, r"\n\r");
    make_token!(tokens, NonNewlineSpace, r" \f\v\t");
    make_token!(tokens, Space, r"[{}{}]", Newline, NonNewlineSpace);
    make_token!(tokens, NonNewline, r"[^{}]", Newline);
    make_token!(tokens, Comment, r"--{}*", NonNewline);
    make_token!(tokens, Whitespace, r"[{}]+|{}", Space, Comment);

    // according to the source, SQL requires at least one newline in the whitespace separating string
    // literals.
    make_token!(
        tokens,
        SpecialWhitespace,
        r"{}+|{}{}",
        Space,
        Comment,
        Newline
    );
    make_token!(
        tokens,
        NonNewlineWhitespace,
        r"[{}]|{}",
        NonNewlineSpace,
        Comment
    );
    make_token!(
        tokens,
        WhitespaceWithNewline,
        r"[{}]*[{}](?:{})*",
        NonNewlineSpace,
        Newline,
        SpecialWhitespace
    );

    /*
     * -- quoting tokens --
     */
    make_token!(tokens, Quote, "'");
    make_token!(tokens, QuoteContinue, "{}{}", WhitespaceWithNewline, Quote);
    // this is a special token used to determine when quoting runs into a comment and thus fails to
    // parse
    make_token!(tokens, QuoteContinueFail, "(?:{})*-?", Whitespace);

    /*
     * -- bit strings --
     */

    // bytes
    make_token!(tokens, XBStart, "[bB]{}", Quote);
    make_token!(tokens, XBInside, "[^']*");
    make_token!(tokens, XBStop, "{}", Quote);

    // hex
    make_token!(tokens, XHStart, "[xX]{}", Quote);
    make_token!(tokens, XHInside, "[^']*");

    // national (?) characters
    make_token!(tokens, XNStart, "[nN]{}", Quote);

    // Quoted string with escaping
    make_token!(tokens, XEStart, "[eE]{}", Quote);
    make_token!(tokens, XEInside, "[^\\']+");
    make_token!(tokens, XEEscape, "[\\][^0-7]");
    make_token!(tokens, XEOctEscape, "[\\][0-7]{1,3}");
    make_token!(tokens, XEHexEscape, "[\\]x[0-9a-fA-F]{1,2}");
    make_token!(
        tokens,
        XEUnicode,
        r"[\\](?:u[0-9a-fA-F]{4}|U[0-9a-fA-F]{8})"
    );
    make_token!(
        tokens,
        XEUnicodeFail,
        r"[\\](?:u[0-9a-fA-F]{0,3}|U[0-9a-fA-F]{0,7})"
    );

    // Extended Quote
    make_token!(tokens, XQStart, "[{}]", Quote);
    make_token!(tokens, XQDouble, "[{}]{{2}}", Quote);
    make_token!(tokens, XQInside, "[^']+");

    // Dollar quoting
    make_token!(tokens, DolQStart, r#"A-Za-z\200-\377_"#);
    make_token!(tokens, DolQContinue, r#"A-Za-z\200-\377_0-9"#);
    make_token!(
        tokens,
        DolQDelimiter,
        r"\$(?:{}{}*)?\$",
        DolQStart,
        DolQContinue
    );
    make_token!(tokens, DolQFailed, r"\$(?:{}{}*", DolQStart, DolQContinue);
    make_token!(tokens, DolQInside, r"[^$]+");

    // Double quoting

    make_token!(tokens, DoubleQuote, "\"");
    make_token!(tokens, XDStart, "{}", DoubleQuote);
    make_token!(tokens, XDStop, "{}", DoubleQuote);
    make_token!(tokens, XDDouble, "{}{}", DoubleQuote, DoubleQuote);
    make_token!(tokens, XDInside, "[^\"]+");

    // Rules for Unicode Escapes

    // Quoted Identifier with Unicode Escapes
    make_token!(tokens, XUIStart, "[uU]&{}", DoubleQuote);
    // Quoted String with Unicode Escapes
    make_token!(tokens, XUSStart, "[uU]&{}", Quote);
    // Error Rule
    make_token!(tokens, XUFailed, "[uU]&");

    /*
     * single character tokens (that aren't whitespace)
     *
     * these are presented out of order compared to the canonical lexer, but we build the token
     * ordering independently of these macros. The reason these are in this place is so that they can
     * be re-used safely in following tokens.
     */

    // this is "self" in the lexer; rust can't use Self as a struct name for (hopefully) obvious
    // reasons.
    make_token!(tokens, SCTokens, r#",()\[\].;\:\+\-\*\/\%\^\<\>\="#);
    make_token!(tokens, OpChars, r#"\~\!\@\#\^\&\|\`\?\+\-\*\/\%\<\>\="#);
    make_token!(tokens, Operator, "[{}]+", OpChars);

    /*
     * -- C style comments --
     *
     * Please see src/backend/parser/scan.l in the postgresql source for additional flavor on the
     * behavior of these tokens.
     */

    make_token!(tokens, XCStart, r#"\/\*[{}]*"#, OpChars);
    make_token!(tokens, XCStop, r#"\*+\/"#);
    make_token!(tokens, XCInside, r#"[^*/]+"#);

    make_token!(tokens, IdentStart, r#"A-Za-z\200-\377_"#);
    make_token!(tokens, IdentCont, r#"A-Za-z\200-\377_0-9$"#);
    make_token!(tokens, Identifier, "[{}][{}]*", IdentStart, IdentCont);

    /*
     * -- special-case operators --
     */

    // hacks to make my character-class constructor work right; not present in original lexer.
    // these might go away later.
    make_token!(tokens, Colon, ":");
    make_token!(tokens, Dot, ".");
    make_token!(tokens, Greater, ">");
    make_token!(tokens, Less, "<");
    make_token!(tokens, Equals, "=");
    make_token!(tokens, Not, "!");

    // These *are* present in the original lexer.
    make_token!(tokens, Typecast, "{}{}", Colon, Colon);
    make_token!(tokens, DotDot, "{}{}", Dot, Dot);
    make_token!(tokens, ColonEquals, "{}{}", Colon, Equals);

    /*
     * -- operator-like tokens --
     */

    make_token!(tokens, EqualsGreater, "{}{}", Equals, Greater);
    make_token!(tokens, LessEquals, "{}{}", Less, Equals);
    make_token!(tokens, GreaterEquals, "{}{}", Greater, Equals);
    make_token!(tokens, LessGreater, "{}{}", Less, Greater);
    make_token!(tokens, NotEquals, "{}{}", Not, Equals);

    /*
     * Numbers
     */

    // stub tokens; it's important to remember these are character classes in the macro.
    make_token!(tokens, DecDigit, "0-9");
    make_token!(tokens, HexDigit, "0-9A-Fa-f");
    make_token!(tokens, OctDigit, "0-7");
    make_token!(tokens, BinDigit, "01");

    make_token!(tokens, DecInteger, "[{}](?:_?[{}])*", DecDigit, DecDigit);
    make_token!(tokens, HexInteger, "0[xX](?:_?[{}])+", HexDigit);
    make_token!(tokens, OctInteger, "0[oO](?:_?[{}])+", OctDigit);
    make_token!(tokens, BinInteger, "0[bB](?:_?[{}])+", BinDigit);

    make_token!(tokens, HexFail, "0[xX]_?");
    make_token!(tokens, OctFail, "0[oO]_?");
    make_token!(tokens, BinFail, "0[bB]_?");

    make_token!(
        tokens,
        Numeric,
        r#"(?:(?:{}\.{})|(?:\.{}))"#,
        DecInteger,
        DecInteger,
        DecInteger
    );
    make_token!(tokens, NumericFail, r#"[{}]+\.\."#, DecDigit);

    make_token!(
        tokens,
        Real,
        "(?:{}|{})[eE][-+]?{}",
        DecInteger,
        Numeric,
        DecInteger
    );
    make_token!(tokens, RealFail, "(?:{}|{})[eE][-+]", DecInteger, Numeric);

    // Not quite sure what the purpose of these tokens are yet, but I'm guessing they indicate
    // match failures.
    make_token!(tokens, DecIntegerJunk, "{}[{}]", DecInteger, IdentStart);
    make_token!(tokens, HexIntegerJunk, "{}[{}]", HexInteger, IdentStart);
    make_token!(tokens, OctIntegerJunk, "{}[{}]", OctInteger, IdentStart);
    make_token!(tokens, BinIntegerJunk, "{}[{}]", BinInteger, IdentStart);
    make_token!(tokens, NumericJunk, "{}[{}]", Numeric, IdentStart);
    make_token!(tokens, RealJunk, "{}[{}]", Real, IdentStart);

    make_token!(tokens, Param, r#"\${}"#, DecInteger);
    make_token!(tokens, ParamJunk, r#"\${}[{}]"#, DecInteger, IdentStart);
}
