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

    pub(crate) fn is_single_line_comment(&self) -> bool {
        matches!(self.kind, TokenKind::S_COMMENT)
    }

    pub(crate) fn is_block_comment(&self) -> bool {
        matches!(self.kind, TokenKind::B_COMMENT)
    }

    pub(crate) fn is_newline(&self) -> bool {
        matches!(self.kind, TokenKind::NEW_LINE)
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

pub(crate) struct BufferedLexer<'src> {
    cached_next_tokens: VecDeque<Token>,
    lexer: Lexer<'src>,
    last_token_range: SourceRange,
    tokens: Vec<Token>,
}

impl<'src> BufferedLexer<'src> {
    pub(crate) fn new(lexer: Lexer<'src>) -> Self {
        Self {
            cached_next_tokens: VecDeque::new(),
            lexer,
            last_token_range: SourceRange::null(),
            tokens: Vec::new(),
        }
    }
}

impl BufferedLexer<'_> {
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

    pub(crate) fn finish(self) -> Vec<Token> {
        self.tokens
    }

    pub(crate) fn current_token_range(&mut self) -> SourceRange {
        self.populate_cache(1);
        self.cached_next_tokens.front().unwrap().range
    }

    fn next_non_trivia_token(&mut self) -> Token {
        loop {
            let token = self.lexer.next_token();
            self.tokens.push(token.clone());
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
