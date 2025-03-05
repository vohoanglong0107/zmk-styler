use core::fmt;
use std::collections::VecDeque;

use crate::source::{Source, SourceIndex, SourceRange};

pub(crate) struct Lexer<'src> {
    source: &'src Source<'src>,
    current_position: SourceIndex,
}
impl<'src> Lexer<'src> {
    pub(crate) fn new(source: &'src Source<'src>) -> Self {
        Self {
            source,
            current_position: SourceIndex::default(),
        }
    }

    pub(crate) fn next_token(&mut self) -> Token {
        if self.is_eof() {
            return Token::eof();
        }
        match self.current_byte() {
            b'&' => self.consume_single_byte(TokenKind::AMP),
            b'@' => self.consume_single_byte(TokenKind::AT),
            b':' => self.consume_single_byte(TokenKind::COLON),
            b';' => self.consume_single_byte(TokenKind::SEMICOLON),
            b',' => self.consume_single_byte(TokenKind::COMMA),
            b'=' => self.consume_single_byte(TokenKind::EQUAL),
            b'<' => self.consume_single_byte(TokenKind::L_ANGLE),
            b'>' => self.consume_single_byte(TokenKind::R_ANGLE),
            b'{' => self.consume_single_byte(TokenKind::L_CURLY),
            b'}' => self.consume_single_byte(TokenKind::R_CURLY),
            b'(' => self.consume_single_byte(TokenKind::L_PAREN),
            b')' => self.consume_single_byte(TokenKind::R_PAREN),
            b'/' => self.consume_slash(),
            b'"' => self.consume_string(),
            b'0'..=b'9' => self.consume_integer(),
            // Technically we have to have a new token for property name,
            // but we have to either have lex context or handle two different
            // types of name at client
            b'a'..=b'z' | b'A'..=b'Z' | b'#' => self.consume_name(),
            b' ' | b'\t' => self.consume_whitespace(),
            b'\r' | b'\n' => self.consume_new_line(),
            _ => self.consume_single_byte(TokenKind::UNKNOWN),
        }
    }

    fn advance(&mut self) {
        self.current_position = self.current_position.increment();
    }

    fn current_byte(&self) -> u8 {
        self.source[self.current_position]
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.current_position.increment()).cloned()
    }

    fn consume_single_byte(&mut self, kind: TokenKind) -> Token {
        let start = self.current_position;
        self.advance();
        Token {
            kind,
            range: self.range(start),
        }
    }

    fn consume_slash(&mut self) -> Token {
        assert_eq!(self.current_byte(), b'/');
        match (self.current_byte(), self.peek()) {
            (b'/', Some(b'/')) => self.consume_single_line_comment(),
            (b'/', Some(b'*')) => self.consume_block_comment(),
            _ => self.consume_root_node(),
        }
    }

    fn consume_single_line_comment(&mut self) -> Token {
        assert_eq!(self.current_byte(), b'/');
        assert_eq!(self.peek(), Some(b'/'));
        let start = self.current_position;
        self.advance();
        loop {
            self.advance();
            if self.is_eof() || self.current_byte() == b'\n' {
                if self.current_byte() == b'\n' {
                    self.advance();
                }
                break Token {
                    kind: TokenKind::S_COMMENT,
                    range: self.range(start),
                };
            }
        }
    }

    fn consume_block_comment(&mut self) -> Token {
        assert_eq!(self.current_byte(), b'/');
        assert_eq!(self.peek(), Some(b'*'));
        let start = self.current_position;
        self.advance();
        loop {
            self.advance();

            if self.is_eof() {
                break Token {
                    kind: TokenKind::UNKNOWN,
                    range: self.range(start),
                };
            }
            if self.current_byte() == b'*' && self.peek().is_some_and(|t| t == b'/') {
                // consume '*'
                self.advance();
                // consume '/'
                self.advance();
                break Token {
                    kind: TokenKind::B_COMMENT,
                    range: self.range(start),
                };
            }
        }
    }

    fn consume_root_node(&mut self) -> Token {
        assert_eq!(self.current_byte(), b'/');
        let start = self.current_position;
        self.advance();
        Token {
            kind: TokenKind::ROOT,
            range: self.range(start),
        }
    }

    fn consume_string(&mut self) -> Token {
        assert_eq!(self.current_byte(), b'"');
        let start = self.current_position;
        self.advance();
        while !self.is_eof() && self.current_byte() != b'"' && self.current_byte() != b'\n' {
            self.advance();
        }
        if self.is_eof() || self.current_byte() == b'\n' {
            // Don't consume the new line, as new line is not part of a string
            Token {
                kind: TokenKind::UNKNOWN,
                range: self.range(start),
            }
        } else {
            self.advance();
            Token {
                kind: TokenKind::STRING,
                range: self.range(start),
            }
        }
    }

    fn consume_integer(&mut self) -> Token {
        let start = self.current_position;
        if self.current_byte() == b'0' && self.peek().is_some_and(|b| b == b'x' || b == b'X') {
            // consume '0'
            self.advance();
            // consume 'x' or 'X'
            self.advance();
            while !self.is_eof() && self.current_byte().is_ascii_hexdigit() {
                self.advance();
            }
            Token {
                kind: TokenKind::INT,
                range: self.range(start),
            }
        } else if self.current_byte() == b'0' {
            while !self.is_eof() && matches!(self.current_byte(), b'0'..=b'7') {
                self.advance();
            }
            Token {
                kind: TokenKind::INT,
                range: self.range(start),
            }
        } else {
            while !self.is_eof() && self.current_byte().is_ascii_digit() {
                self.advance();
            }
            Token {
                kind: TokenKind::INT,
                range: self.range(start),
            }
        }
    }

    // https://devicetree-specification.readthedocs.io/en/latest/chapter2-devicetree-basics.html#node-name-requirements
    fn consume_name(&mut self) -> Token {
        let start = self.current_position;
        while !self.is_eof()
            // since we don't have separate token for node and property names,
            // node names might contains '#' or '?'
            && matches!(self.current_byte(), b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b',' | b'.' | b'_' | b'+' | b'-' | b'#' | b'?')
        {
            self.advance();
        }
        Token {
            kind: TokenKind::NAME,
            range: self.range(start),
        }
    }

    fn consume_whitespace(&mut self) -> Token {
        let start = self.current_position;
        while !self.is_eof() && matches!(self.current_byte(), b' ' | b'\t') {
            self.advance();
        }
        Token {
            kind: TokenKind::SPACE,
            range: self.range(start),
        }
    }

    fn consume_new_line(&mut self) -> Token {
        let start = self.current_position;
        match (self.current_byte(), self.peek()) {
            (b'\n', _) => {
                self.advance();
                Token {
                    kind: TokenKind::NEW_LINE,
                    range: self.range(start),
                }
            }
            (b'\r', Some(b'\n')) => {
                self.advance();
                self.advance();
                Token {
                    kind: TokenKind::NEW_LINE,
                    range: self.range(start),
                }
            }
            _ => {
                self.advance();
                Token {
                    kind: TokenKind::UNKNOWN,
                    range: self.range(start),
                }
            }
        }
    }

    fn is_eof(&self) -> bool {
        self.source.is_eof(self.current_position)
    }

    fn range(&self, start: SourceIndex) -> SourceRange {
        SourceRange::new(start, self.current_position)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) range: SourceRange,
}

impl Token {
    pub(crate) fn is_trivia(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::NEW_LINE | TokenKind::SPACE | TokenKind::B_COMMENT | TokenKind::S_COMMENT
        )
    }

    pub(crate) fn is_eof(&self) -> bool {
        matches!(self.kind, TokenKind::EOF)
    }

    fn eof() -> Self {
        Self {
            kind: TokenKind::EOF,
            range: SourceRange::null(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    /// End of file
    EOF,
    /// Unknown token
    UNKNOWN,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub(crate) struct TokenSource<'src> {
    cached_next_tokens: VecDeque<Token>,
    lexer: Lexer<'src>,
    last_token_range: SourceRange,
}

impl<'src> TokenSource<'src> {
    pub(crate) fn new(lexer: Lexer<'src>) -> Self {
        Self {
            cached_next_tokens: VecDeque::new(),
            lexer,
            last_token_range: SourceRange::null(),
        }
    }
}

impl TokenSource<'_> {
    pub(crate) fn nth(&mut self, pos: usize) -> Token {
        self.populate_cache(pos + 1);
        self.cached_next_tokens.get(pos).unwrap().clone()
    }

    pub(crate) fn advance(&mut self) -> Token {
        let token = if self.cached_next_tokens.is_empty() {
            self.next_non_trivia_token()
        } else {
            self.cached_next_tokens.pop_front().unwrap()
        };
        self.last_token_range = token.range;
        token
    }

    pub(crate) fn current_token_start(&mut self) -> SourceIndex {
        self.current_token_range().start()
    }

    pub(crate) fn last_token_end(&mut self) -> SourceIndex {
        self.last_token_range.end()
    }

    fn current_token_range(&mut self) -> SourceRange {
        self.populate_cache(1);
        self.cached_next_tokens.front().unwrap().range
    }

    fn next_non_trivia_token(&mut self) -> Token {
        loop {
            let token = self.lexer.next_token();
            if !token.is_trivia() {
                break token;
            }
        }
    }

    fn populate_cache(&mut self, size: usize) {
        while self.cached_next_tokens.len() < size {
            let token = self.next_non_trivia_token();
            self.cached_next_tokens.push_back(token);
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;
    use std::str;

    use super::*;

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
            Token(S_COMMENT, [19..37], "// single comment\n")
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
