mod token;
mod shortcuts;

pub use token::{Token, TokenKind, Position, Span};
pub use shortcuts::ShortcutMap;

use crate::errors::{Result, YourMomError};

pub struct Lexer<'a> {
    input: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
    shortcuts: &'a ShortcutMap,
    pub filename: String,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &str, shortcuts: &'a ShortcutMap) -> Self {
        Self::with_filename(source, shortcuts, "<input>")
    }

    pub fn with_filename(source: &str, shortcuts: &'a ShortcutMap, filename: &str) -> Self {
        Self {
            input: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
            shortcuts,
            filename: filename.to_string(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            loop {
                let before = self.pos;
                self.skip_whitespace();
                self.skip_comments();
                if self.pos == before { break; }
            }

            if self.is_at_end() {
                break;
            }

            tokens.push(self.next_token()?);
        }

        let eof_pos = self.current_position();
        tokens.push(Token::new(TokenKind::Eof, Span { start: eof_pos, end: eof_pos }));
        Ok(tokens)
    }

    fn current_position(&self) -> Position {
        Position { line: self.line, column: self.column, offset: self.pos }
    }

    fn next_token(&mut self) -> Result<Token> {
        let start = self.current_position();
        let kind = self.next_token_kind()?;
        let end = self.current_position();
        Ok(Token::new(kind, Span { start, end }))
    }

    fn next_token_kind(&mut self) -> Result<TokenKind> {
        let c = self.peek();

        if c == '"' {
            return self.read_string();
        }
        if c.is_ascii_digit() {
            return self.read_number();
        }
        if c.is_alphabetic() || c == '_' {
            return self.read_identifier();
        }

        let c = self.advance();
        match c {
            '+' => Ok(TokenKind::Plus),
            '-' => Ok(TokenKind::Minus),
            '*' => Ok(TokenKind::Star),
            '/' => Ok(TokenKind::Slash),
            '%' => Ok(TokenKind::Percent),
            '|' => Ok(TokenKind::Pipe),
            ',' => Ok(TokenKind::Comma),
            ';' => Ok(TokenKind::Semicolon),
            ':' => Ok(TokenKind::Colon),
            '(' => Ok(TokenKind::LParen),
            ')' => Ok(TokenKind::RParen),
            '{' => Ok(TokenKind::LBrace),
            '}' => Ok(TokenKind::RBrace),
            '[' => Ok(TokenKind::LBracket),
            ']' => Ok(TokenKind::RBracket),
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(TokenKind::EqEq)
                } else {
                    Ok(TokenKind::Eq)
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(TokenKind::NotEq)
                } else {
                    Err(YourMomError::LexerError {
                        line: self.line,
                        column: self.column,
                        message: format!("unexpected character '!'"),
                    })
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(TokenKind::LtEq)
                } else {
                    Ok(TokenKind::Lt)
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(TokenKind::GtEq)
                } else {
                    Ok(TokenKind::Gt)
                }
            }
            other => Err(YourMomError::LexerError {
                line: self.line,
                column: self.column,
                message: format!("unexpected character '{}'", other),
            }),
        }
    }

    fn read_string(&mut self) -> Result<TokenKind> {
        self.advance(); // consume opening "
        let mut s = String::new();
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\\' {
                self.advance();
                match self.advance() {
                    'n' => s.push('\n'),
                    't' => s.push('\t'),
                    '"' => s.push('"'),
                    '\\' => s.push('\\'),
                    c => { s.push('\\'); s.push(c); }
                }
            } else {
                s.push(self.advance());
            }
        }
        if self.is_at_end() {
            return Err(YourMomError::LexerError {
                line: self.line,
                column: self.column,
                message: "unterminated string literal — yo mama left a string open".to_string(),
            });
        }
        self.advance(); // consume closing "
        Ok(TokenKind::StringLit(s))
    }

    fn read_number(&mut self) -> Result<TokenKind> {
        let mut s = String::new();
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            s.push(self.advance());
        }
        let n: i64 = s.parse().map_err(|_| YourMomError::LexerError {
            line: self.line,
            column: self.column,
            message: format!("number '{}' overflowed — yo mama so fat she overflowed i64", s),
        })?;
        Ok(TokenKind::Number(n))
    }

    fn read_identifier(&mut self) -> Result<TokenKind> {
        let mut s = String::new();
        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            s.push(self.advance());
        }

        let s = self.shortcuts.expand(&s);

        let token = match s.as_str() {
            "yo"                    => TokenKind::Yo,
            "yo_mama_so_fat"        => TokenKind::YoMamaSoFat,
            "yo_mama_so_dumb"       => TokenKind::YoMamaSoDumb,
            "yo_mama_so_ugly"       => TokenKind::YoMamaSoUgly,
            "mama_main"             => TokenKind::MamaMain,
            "mama_said"             => TokenKind::MamaSaid,
            "mama_lied"             => TokenKind::MamaLied,
            "mama_keeps_saying"     => TokenKind::MamaKeepsSaying,
            "did_your_mom"          => TokenKind::DidYourMom,
            "doing_your_mom"        => TokenKind::DoingYourMom,
            "yo_daddy"              => TokenKind::YoDaddy,
            "yo_mama_in_two_places" => TokenKind::YoMamaInTwoPlaces,
            "int_all"               => TokenKind::IntAll,
            "real_all"              => TokenKind::RealAll,
            "rational_all"          => TokenKind::RationalAll,
            "irrational_all"        => TokenKind::IrrationalAll,
            "imaginary_all"         => TokenKind::ImaginaryAll,
            "heisenberg"            => TokenKind::Heisenberg,
            "quaternion_all"        => TokenKind::QuaternionAll,
            "transfinite_all"       => TokenKind::TransfiniteAll,
            "quantum_sort"          => TokenKind::QuantumSort,
            "entangled_with"        => TokenKind::EntangledWith,
            "mama_said_maybe"       => TokenKind::MamaSaidMaybe,
            "mama_forgot"           => TokenKind::MamaForgot,
            "yo_daddy_issues"       => TokenKind::YoDaddyIssues,
            "mama_caught"           => TokenKind::MamaCaught,
            "yo_mama_so_loud"       => TokenKind::YoMamaSoLoud,
            "yo_mama_so_paranoid"   => TokenKind::YoMamaSoParanoid,
            "yo_mama_so_lazy"       => TokenKind::YoMamaSoLazy,
            "yo_mama_so_meta"       => TokenKind::YoMamaSoMeta,
            _                       => TokenKind::Identifier(s),
        };

        Ok(token)
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn skip_comments(&mut self) {
        if self.peek() == '/' && self.peek_ahead(1) == '/' {
            while !self.is_at_end() && self.peek() != '\n' {
                self.advance();
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.pos]
        }
    }

    fn peek_ahead(&self, offset: usize) -> char {
        let idx = self.pos + offset;
        if idx >= self.input.len() {
            '\0'
        } else {
            self.input[idx]
        }
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        c
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.input.len()
    }
}
