#[cfg(test)]
mod test {
    use insta::assert_snapshot;
    use std::str;

    use crate::{
        lexer::{Lexer, Token},
        source::Source,
    };

    #[test]
    fn lex_string() {
        let lexer = lex(r#"
            compatible = "zmk,behavior-tap-dance";
            label = "LAYER_TAP_DANCE";"#);
        assert_snapshot!(lexer, @r#"
        [
            Token(NEW_LINE, [0..1], "\n")
            Token(SPACE, [1..13], "            ")
            Token(NAME, [13..23], "compatible")
            Token(SPACE, [23..24], " ")
            Token(EQUAL, [24..25], "=")
            Token(SPACE, [25..26], " ")
            Token(STRING, [26..50], "\"zmk,behavior-tap-dance\"")
            Token(SEMICOLON, [50..51], ";")
            Token(NEW_LINE, [51..52], "\n")
            Token(SPACE, [52..64], "            ")
            Token(NAME, [64..69], "label")
            Token(SPACE, [69..70], " ")
            Token(EQUAL, [70..71], "=")
            Token(SPACE, [71..72], " ")
            Token(STRING, [72..89], "\"LAYER_TAP_DANCE\"")
            Token(SEMICOLON, [89..90], ";")
        ]
        "#);

        let tokens = lex(r#"compatible = "zmk,behavior-tap-dance;
            label = "LAYER_TAP_DANCE";"#);
        assert_snapshot!(tokens, @r#"
        [
            Token(NAME, [0..10], "compatible")
            Token(SPACE, [10..11], " ")
            Token(EQUAL, [11..12], "=")
            Token(SPACE, [12..13], " ")
            Token(UNKNOWN, [13..37], "\"zmk,behavior-tap-dance;")
            Token(NEW_LINE, [37..38], "\n")
            Token(SPACE, [38..50], "            ")
            Token(NAME, [50..55], "label")
            Token(SPACE, [55..56], " ")
            Token(EQUAL, [56..57], "=")
            Token(SPACE, [57..58], " ")
            Token(STRING, [58..75], "\"LAYER_TAP_DANCE\"")
            Token(SEMICOLON, [75..76], ";")
        ]
        "#)
    }

    #[test]
    fn lex_node() {
        let lexer = lex(r#"
/ {
    behaviors {
        lower: lower {
        };
    };
};"#);
        assert_snapshot!(lexer, @r#"
        [
            Token(NEW_LINE, [0..1], "\n")
            Token(ROOT, [1..2], "/")
            Token(SPACE, [2..3], " ")
            Token(L_CURLY, [3..4], "{")
            Token(NEW_LINE, [4..5], "\n")
            Token(SPACE, [5..9], "    ")
            Token(NAME, [9..18], "behaviors")
            Token(SPACE, [18..19], " ")
            Token(L_CURLY, [19..20], "{")
            Token(NEW_LINE, [20..21], "\n")
            Token(SPACE, [21..29], "        ")
            Token(NAME, [29..34], "lower")
            Token(COLON, [34..35], ":")
            Token(SPACE, [35..36], " ")
            Token(NAME, [36..41], "lower")
            Token(SPACE, [41..42], " ")
            Token(L_CURLY, [42..43], "{")
            Token(NEW_LINE, [43..44], "\n")
            Token(SPACE, [44..52], "        ")
            Token(R_CURLY, [52..53], "}")
            Token(SEMICOLON, [53..54], ";")
            Token(NEW_LINE, [54..55], "\n")
            Token(SPACE, [55..59], "    ")
            Token(R_CURLY, [59..60], "}")
            Token(SEMICOLON, [60..61], ";")
            Token(NEW_LINE, [61..62], "\n")
            Token(R_CURLY, [62..63], "}")
            Token(SEMICOLON, [63..64], ";")
        ]
        "#)
    }

    #[test]
    fn lex_property() {
        let tokens = lex(r#"/ {
    #binding-cells = <0>;
    tapping-term-ms = <200 0x123cf 0123>;
    bindings = <&mo LAYER_Lower>, <&to LAYER_Lower>;
};"#);
        assert_snapshot!(tokens, @r##"
        [
            Token(ROOT, [0..1], "/")
            Token(SPACE, [1..2], " ")
            Token(L_CURLY, [2..3], "{")
            Token(NEW_LINE, [3..4], "\n")
            Token(SPACE, [4..8], "    ")
            Token(NAME, [8..22], "#binding-cells")
            Token(SPACE, [22..23], " ")
            Token(EQUAL, [23..24], "=")
            Token(SPACE, [24..25], " ")
            Token(L_ANGLE, [25..26], "<")
            Token(INT, [26..27], "0")
            Token(R_ANGLE, [27..28], ">")
            Token(SEMICOLON, [28..29], ";")
            Token(NEW_LINE, [29..30], "\n")
            Token(SPACE, [30..34], "    ")
            Token(NAME, [34..49], "tapping-term-ms")
            Token(SPACE, [49..50], " ")
            Token(EQUAL, [50..51], "=")
            Token(SPACE, [51..52], " ")
            Token(L_ANGLE, [52..53], "<")
            Token(INT, [53..56], "200")
            Token(SPACE, [56..57], " ")
            Token(INT, [57..64], "0x123cf")
            Token(SPACE, [64..65], " ")
            Token(INT, [65..69], "0123")
            Token(R_ANGLE, [69..70], ">")
            Token(SEMICOLON, [70..71], ";")
            Token(NEW_LINE, [71..72], "\n")
            Token(SPACE, [72..76], "    ")
            Token(NAME, [76..84], "bindings")
            Token(SPACE, [84..85], " ")
            Token(EQUAL, [85..86], "=")
            Token(SPACE, [86..87], " ")
            Token(L_ANGLE, [87..88], "<")
            Token(AMP, [88..89], "&")
            Token(NAME, [89..91], "mo")
            Token(SPACE, [91..92], " ")
            Token(NAME, [92..103], "LAYER_Lower")
            Token(R_ANGLE, [103..104], ">")
            Token(COMMA, [104..105], ",")
            Token(SPACE, [105..106], " ")
            Token(L_ANGLE, [106..107], "<")
            Token(AMP, [107..108], "&")
            Token(NAME, [108..110], "to")
            Token(SPACE, [110..111], " ")
            Token(NAME, [111..122], "LAYER_Lower")
            Token(R_ANGLE, [122..123], ">")
            Token(SEMICOLON, [123..124], ";")
            Token(NEW_LINE, [124..125], "\n")
            Token(R_CURLY, [125..126], "}")
            Token(SEMICOLON, [126..127], ";")
        ]
        "##);
    }

    #[test]
    fn lex_comment() {
        let tokens = lex(r#"
/* #define for */
// single comment
"#);
        assert_snapshot!(tokens, @r#"
        [
            Token(NEW_LINE, [0..1], "\n")
            Token(B_COMMENT, [1..18], "/* #define for */")
            Token(NEW_LINE, [18..19], "\n")
            Token(S_COMMENT, [19..36], "// single comment")
            Token(NEW_LINE, [36..37], "\n")
        ]
        "#);
    }

    fn lex(source: &str) -> String {
        let source = Source::new(source);
        let mut lexer = Lexer::new(&source);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token();
            if token.is_eof() {
                break;
            }
            tokens.push(token);
        }
        print_tokens(&tokens, &source)
    }

    fn print_token(token: &Token, source: &Source) -> String {
        format!(
            "Token({}, {}, {:?})",
            &token.kind,
            &token.range,
            &str::from_utf8(&source[token.range]).unwrap()
        )
    }

    fn print_tokens(tokens: &[Token], source: &Source) -> String {
        let mut tokens_body = String::from('\n');
        for token in tokens {
            tokens_body.push_str("    ");
            tokens_body.push_str(&print_token(token, source));
            tokens_body.push('\n');
        }
        format!("[{tokens_body}]",)
    }
}
