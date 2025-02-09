use core::fmt;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{anychar, line_ending, one_of, space1},
    combinator::{map, peek, recognize},
    multi::{many0, many1, many_till},
    sequence::{pair, preceded},
    IResult, Parser,
};

pub(crate) fn lex_tokens(input: &str) -> Vec<Token> {
    let mut parser = many0(alt((
        tag("&").map(Token::amp),
        tag("@").map(Token::at),
        tag(":").map(Token::colon),
        tag(";").map(Token::semicolon),
        tag(",").map(Token::comma),
        tag("=").map(Token::equal),
        tag("<").map(Token::l_angle),
        tag(">").map(Token::r_angle),
        tag("{").map(Token::l_curly),
        tag("}").map(Token::r_curly),
        tag("(").map(Token::l_paren),
        tag(")").map(Token::r_paren),
        single_line_comment,
        block_comment,
        string,
        integer,
        // valid integer can be a valid name, so name must follow integer
        name,
        tag("/").map(Token::root),
        space1.map(Token::space),
        line_ending.map(Token::new_line),
        // catch all into an Unknown token
        anychar.map(Token::unknown),
    )));
    let (unparsed, tokens) = parser(input).expect("Lexer shouldn't fail");
    assert_eq!(unparsed, "");
    tokens
}

fn single_line_comment(input: &str) -> IResult<&str, Token> {
    map(
        preceded(tag("//"), many_till(anychar, line_ending)),
        |(comment, _)| Token::s_comment(comment.into_iter().collect::<String>()),
    )(input)
}

fn block_comment(input: &str) -> IResult<&str, Token> {
    map(
        preceded(tag("/*"), many_till(anychar, tag("*/"))),
        |(comment, _)| Token::b_comment(comment.into_iter().collect::<String>()),
    )(input)
}

fn string(input: &str) -> IResult<&str, Token> {
    let mut parser = map(
        pair(
            tag("\""),
            many_till(anychar, alt((tag("\""), peek(line_ending)))),
        ),
        |(l_quote, (content, ending))| {
            let content: String = content.into_iter().collect();
            if ending == "\"" {
                Token::string(content)
            } else {
                Token::unknown(format!("{l_quote}{content}"))
            }
        },
    );
    parser(input)
}

// https://learn.microsoft.com/en-us/cpp/c-language/c-integer-constants?view=msvc-170
fn integer(input: &str) -> IResult<&str, Token> {
    let hex_parser = recognize(pair(
        alt((tag("0x"), tag("0X"))),
        many1(one_of("0123456789abcdefABCDEF")),
    ));
    let oct_parser = recognize(pair(tag("0"), many0(one_of("01234567"))));
    let dec_parser = recognize(pair(
        one_of("1234567"),
        many0(one_of("0123456789abcdefABCDEF")),
    ));
    let mut parser = map(alt((hex_parser, oct_parser, dec_parser)), |val: &str| {
        Token {
            kind: TokenKind::INT,
            text: val.to_owned(),
        }
    });
    parser(input)
}

fn name(input: &str) -> IResult<&str, Token> {
    map(
        take_while1(|c: char| c.is_alphanumeric() || ",._+?#-".contains(c)),
        |name: &str| Token {
            kind: TokenKind::NAME,
            text: name.to_string(),
        },
    )(input)
}

#[derive(Clone, Debug)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) text: String,
}

impl Token {
    fn amp(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::AMP,
            text: s.to_string(),
        }
    }

    fn at(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::AT,
            text: s.to_string(),
        }
    }

    fn colon(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::COLON,
            text: s.to_string(),
        }
    }

    fn semicolon(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::SEMICOLON,
            text: s.to_string(),
        }
    }

    fn comma(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::COMMA,
            text: s.to_string(),
        }
    }

    fn equal(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::EQUAL,
            text: s.to_string(),
        }
    }

    fn l_angle(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::L_ANGLE,
            text: s.to_string(),
        }
    }

    fn r_angle(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::R_ANGLE,
            text: s.to_string(),
        }
    }

    fn l_curly(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::L_CURLY,
            text: s.to_string(),
        }
    }
    fn r_curly(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::R_CURLY,
            text: s.to_string(),
        }
    }

    fn l_paren(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::L_PAREN,
            text: s.to_string(),
        }
    }

    fn r_paren(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::R_PAREN,
            text: s.to_string(),
        }
    }

    fn new_line(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::NEW_LINE,
            text: s.to_string(),
        }
    }

    fn space(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::SPACE,
            text: s.to_string(),
        }
    }

    fn root(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::ROOT,
            text: s.to_string(),
        }
    }

    fn string(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::STRING,
            text: s.to_string(),
        }
    }

    fn b_comment(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::B_COMMENT,
            text: s.to_string(),
        }
    }

    fn s_comment(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::S_COMMENT,
            text: s.to_string(),
        }
    }

    fn unknown(s: impl ToString) -> Self {
        Self {
            kind: TokenKind::UNKNOWN,
            text: s.to_string(),
        }
    }

    pub(crate) fn is_trivia(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::NEW_LINE | TokenKind::SPACE | TokenKind::B_COMMENT | TokenKind::S_COMMENT
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum TokenKind {
    /// Ampersand `@`
    AMP,
    /// At sign `@`
    AT,
    /// Colon `:`
    COLON,
    /// Semicolon `;`
    SEMICOLON,
    /// Comma `,`
    COMMA,
    /// Equal `=`
    EQUAL,
    /// Left angle bracket `<`
    L_ANGLE,
    /// Right angle bracket `>`
    R_ANGLE,
    /// Left curly bracket `{`
    L_CURLY,
    /// Right curly bracket `}`
    R_CURLY,
    /// Left parenthesis `(`
    L_PAREN,
    /// Right parenthesis `)`
    R_PAREN,
    /// New line `\n` | `\r\n`
    NEW_LINE,
    /// White space `\t` | ` `
    SPACE,
    /// An indentifier, can be a node name, label or property name
    NAME,
    /// Root node `/`
    ROOT,
    /// Int
    INT,
    /// String
    STRING,
    /// Block comment
    B_COMMENT,
    /// Single line comment
    S_COMMENT,
    /// Unknown token
    UNKNOWN,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn lex_string() {
        let lexer = lex_tokens(
            r#"
            compatible = "zmk,behavior-tap-dance";
            label = "LAYER_TAP_DANCE";"#,
        );
        assert_debug_snapshot!(lexer, @r#"
        [
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: SPACE,
                text: "            ",
            },
            Token {
                kind: NAME,
                text: "compatible",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: EQUAL,
                text: "=",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: STRING,
                text: "zmk,behavior-tap-dance",
            },
            Token {
                kind: SEMICOLON,
                text: ";",
            },
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: SPACE,
                text: "            ",
            },
            Token {
                kind: NAME,
                text: "label",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: EQUAL,
                text: "=",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: STRING,
                text: "LAYER_TAP_DANCE",
            },
            Token {
                kind: SEMICOLON,
                text: ";",
            },
        ]
        "#);

        let tokens = lex_tokens(
            r#"compatible = "zmk,behavior-tap-dance;
            label = "LAYER_TAP_DANCE";"#,
        );
        assert_debug_snapshot!(tokens, @r#"
        [
            Token {
                kind: NAME,
                text: "compatible",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: EQUAL,
                text: "=",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: UNKNOWN,
                text: "\"zmk,behavior-tap-dance;",
            },
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: SPACE,
                text: "            ",
            },
            Token {
                kind: NAME,
                text: "label",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: EQUAL,
                text: "=",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: STRING,
                text: "LAYER_TAP_DANCE",
            },
            Token {
                kind: SEMICOLON,
                text: ";",
            },
        ]
        "#)
    }

    #[test]
    fn lex_node() {
        let lexer = lex_tokens(
            r#"
/ {
    behaviors {
        lower: lower {
        };
    };
};"#,
        );
        assert_debug_snapshot!(lexer, @r#"
        [
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: ROOT,
                text: "/",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: L_CURLY,
                text: "{",
            },
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: SPACE,
                text: "    ",
            },
            Token {
                kind: NAME,
                text: "behaviors",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: L_CURLY,
                text: "{",
            },
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: SPACE,
                text: "        ",
            },
            Token {
                kind: NAME,
                text: "lower",
            },
            Token {
                kind: COLON,
                text: ":",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: NAME,
                text: "lower",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: L_CURLY,
                text: "{",
            },
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: SPACE,
                text: "        ",
            },
            Token {
                kind: R_CURLY,
                text: "}",
            },
            Token {
                kind: SEMICOLON,
                text: ";",
            },
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: SPACE,
                text: "    ",
            },
            Token {
                kind: R_CURLY,
                text: "}",
            },
            Token {
                kind: SEMICOLON,
                text: ";",
            },
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: R_CURLY,
                text: "}",
            },
            Token {
                kind: SEMICOLON,
                text: ";",
            },
        ]
        "#)
    }

    #[test]
    fn lex_property() {
        let tokens = lex_tokens(
            r#"/ {
    #binding-cells = <0>;
    tapping-term-ms = <200 0x123cf 0123>;
    bindings = <&mo LAYER_Lower>, <&to LAYER_Lower>;
};"#,
        );
        assert_debug_snapshot!(tokens, @r##"
        [
            Token {
                kind: ROOT,
                text: "/",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: L_CURLY,
                text: "{",
            },
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: SPACE,
                text: "    ",
            },
            Token {
                kind: NAME,
                text: "#binding-cells",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: EQUAL,
                text: "=",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: L_ANGLE,
                text: "<",
            },
            Token {
                kind: INT,
                text: "0",
            },
            Token {
                kind: R_ANGLE,
                text: ">",
            },
            Token {
                kind: SEMICOLON,
                text: ";",
            },
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: SPACE,
                text: "    ",
            },
            Token {
                kind: NAME,
                text: "tapping-term-ms",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: EQUAL,
                text: "=",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: L_ANGLE,
                text: "<",
            },
            Token {
                kind: INT,
                text: "200",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: INT,
                text: "0x123cf",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: INT,
                text: "0123",
            },
            Token {
                kind: R_ANGLE,
                text: ">",
            },
            Token {
                kind: SEMICOLON,
                text: ";",
            },
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: SPACE,
                text: "    ",
            },
            Token {
                kind: NAME,
                text: "bindings",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: EQUAL,
                text: "=",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: L_ANGLE,
                text: "<",
            },
            Token {
                kind: AMP,
                text: "&",
            },
            Token {
                kind: NAME,
                text: "mo",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: NAME,
                text: "LAYER_Lower",
            },
            Token {
                kind: R_ANGLE,
                text: ">",
            },
            Token {
                kind: COMMA,
                text: ",",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: L_ANGLE,
                text: "<",
            },
            Token {
                kind: AMP,
                text: "&",
            },
            Token {
                kind: NAME,
                text: "to",
            },
            Token {
                kind: SPACE,
                text: " ",
            },
            Token {
                kind: NAME,
                text: "LAYER_Lower",
            },
            Token {
                kind: R_ANGLE,
                text: ">",
            },
            Token {
                kind: SEMICOLON,
                text: ";",
            },
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: R_CURLY,
                text: "}",
            },
            Token {
                kind: SEMICOLON,
                text: ";",
            },
        ]
        "##);
    }

    #[test]
    fn lex_comment() {
        let tokens = lex_tokens(
            r#"
/* #define for */
// single comment
"#,
        );
        assert_debug_snapshot!(tokens, @r#"
        [
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: B_COMMENT,
                text: " #define for ",
            },
            Token {
                kind: NEW_LINE,
                text: "\n",
            },
            Token {
                kind: S_COMMENT,
                text: " single comment",
            },
        ]
        "#);
    }
}
