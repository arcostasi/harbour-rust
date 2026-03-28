#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    const fn start() -> Self {
        Self {
            offset: 0,
            line: 1,
            column: 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn slice<'src>(&self, source: &'src str) -> &'src str {
        &source[self.start.offset..self.end.offset]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Procedure,
    Function,
    Local,
    Return,
    If,
    Else,
    EndIf,
    Do,
    While,
    EndDo,
    For,
    Next,
    Static,
    Public,
    Private,
    Nil,
    True,
    False,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    Integer,
    Float,
    String,
    Keyword(Keyword),
    Newline,
    Question,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Dollar,
    Hash,
    At,
    Ampersand,
    Pipe,
    Less,
    Greater,
    Assign,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn text<'src>(&self, source: &'src str) -> &'src str {
        self.span.slice(source)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErrorKind {
    InvalidCharacter(char),
    UnterminatedString,
    UnterminatedBlockComment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexedSource {
    pub tokens: Vec<Token>,
    pub errors: Vec<LexError>,
}

pub fn lex(source: &str) -> LexedSource {
    Lexer::new(source).lex()
}

struct Lexer<'src> {
    source: &'src str,
    position: Position,
    tokens: Vec<Token>,
    errors: Vec<LexError>,
}

impl<'src> Lexer<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source,
            position: Position::start(),
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn lex(mut self) -> LexedSource {
        while let Some(ch) = self.peek_char() {
            match ch {
                ' ' | '\t' => {
                    self.bump_char();
                }
                '\r' | '\n' => self.lex_newline(),
                '?' => self.push_single_char(TokenKind::Question),
                '(' => self.push_single_char(TokenKind::LeftParen),
                ')' => self.push_single_char(TokenKind::RightParen),
                '[' => self.push_single_char(TokenKind::LeftBracket),
                ']' => self.push_single_char(TokenKind::RightBracket),
                '{' => self.push_single_char(TokenKind::LeftBrace),
                '}' => self.push_single_char(TokenKind::RightBrace),
                ',' => self.push_single_char(TokenKind::Comma),
                ';' => self.push_single_char(TokenKind::Semicolon),
                ':' => self.push_single_char(TokenKind::Colon),
                '.' => self.push_single_char(TokenKind::Dot),
                '+' => self.push_single_char(TokenKind::Plus),
                '-' => self.push_single_char(TokenKind::Minus),
                '*' => self.push_single_char(TokenKind::Star),
                '/' => self.push_single_char(TokenKind::Slash),
                '%' => self.push_single_char(TokenKind::Percent),
                '^' => self.push_single_char(TokenKind::Caret),
                '$' => self.push_single_char(TokenKind::Dollar),
                '#' => self.push_single_char(TokenKind::Hash),
                '@' => self.push_single_char(TokenKind::At),
                '&' => self.push_single_char(TokenKind::Ampersand),
                '|' => self.push_single_char(TokenKind::Pipe),
                '<' => self.push_single_char(TokenKind::Less),
                '>' => self.push_single_char(TokenKind::Greater),
                '=' => self.push_single_char(TokenKind::Assign),
                ch if is_identifier_start(ch) => self.lex_identifier(),
                ch if ch.is_ascii_digit() => self.lex_number(),
                '"' | '\'' | '`' => self.lex_string(ch),
                ch => self.lex_invalid_character(ch),
            }
        }

        let eof = self.position;
        self.tokens.push(Token {
            kind: TokenKind::Eof,
            span: Span {
                start: eof,
                end: eof,
            },
        });

        LexedSource {
            tokens: self.tokens,
            errors: self.errors,
        }
    }

    fn lex_newline(&mut self) {
        let start = self.position;

        if self.peek_char() == Some('\r') {
            self.bump_char();
            if self.peek_char() == Some('\n') {
                self.bump_char();
            }
        } else {
            self.bump_char();
        }

        self.tokens.push(Token {
            kind: TokenKind::Newline,
            span: Span {
                start,
                end: self.position,
            },
        });
    }

    fn lex_identifier(&mut self) {
        let start = self.position;
        self.bump_char();

        while let Some(ch) = self.peek_char() {
            if !is_identifier_continue(ch) {
                break;
            }
            self.bump_char();
        }

        self.tokens.push(Token {
            kind: TokenKind::Identifier,
            span: Span {
                start,
                end: self.position,
            },
        });
    }

    fn lex_number(&mut self) {
        let start = self.position;
        let mut saw_dot = false;

        while let Some(ch) = self.peek_char() {
            match ch {
                '0'..='9' => {
                    self.bump_char();
                }
                '.' if !saw_dot => {
                    saw_dot = true;
                    self.bump_char();
                }
                _ => break,
            }
        }

        self.tokens.push(Token {
            kind: if saw_dot {
                TokenKind::Float
            } else {
                TokenKind::Integer
            },
            span: Span {
                start,
                end: self.position,
            },
        });
    }

    fn lex_string(&mut self, delimiter: char) {
        let start = self.position;
        let end_delimiter = if delimiter == '`' { '\'' } else { delimiter };
        self.bump_char();

        while let Some(ch) = self.peek_char() {
            if ch == end_delimiter {
                self.bump_char();
                self.tokens.push(Token {
                    kind: TokenKind::String,
                    span: Span {
                        start,
                        end: self.position,
                    },
                });
                return;
            }

            if ch == '\r' || ch == '\n' {
                break;
            }

            self.bump_char();
        }

        self.errors.push(LexError {
            kind: LexErrorKind::UnterminatedString,
            span: Span {
                start,
                end: self.position,
            },
            message: "unterminated string literal".to_owned(),
        });
    }

    fn lex_invalid_character(&mut self, ch: char) {
        let start = self.position;
        self.bump_char();

        self.errors.push(LexError {
            kind: LexErrorKind::InvalidCharacter(ch),
            span: Span {
                start,
                end: self.position,
            },
            message: format!("invalid character `{ch}`"),
        });
    }

    fn push_single_char(&mut self, kind: TokenKind) {
        let start = self.position;
        self.bump_char();
        self.tokens.push(Token {
            kind,
            span: Span {
                start,
                end: self.position,
            },
        });
    }

    fn peek_char(&self) -> Option<char> {
        self.source[self.position.offset..].chars().next()
    }

    fn bump_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.position.offset += ch.len_utf8();

        if ch == '\n' {
            self.position.line += 1;
            self.position.column = 1;
        } else {
            self.position.column += 1;
        }

        Some(ch)
    }
}

fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::{Position, TokenKind, lex};

    #[test]
    fn empty_input_yields_eof() {
        let lexed = lex("");
        assert!(lexed.errors.is_empty());
        assert_eq!(lexed.tokens.len(), 1);
        assert_eq!(lexed.tokens[0].kind, TokenKind::Eof);
        assert_eq!(
            lexed.tokens[0].span.start,
            Position {
                offset: 0,
                line: 1,
                column: 1
            }
        );
    }

    #[test]
    fn newline_tracks_source_position() {
        let lexed = lex("\r\nx");
        assert!(lexed.errors.is_empty());
        assert_eq!(lexed.tokens[0].kind, TokenKind::Newline);
        assert_eq!(lexed.tokens[1].kind, TokenKind::Identifier);
        assert_eq!(lexed.tokens[1].span.start.line, 2);
        assert_eq!(lexed.tokens[1].span.start.column, 1);
        assert_eq!(lexed.tokens[1].span.start.offset, 2);
    }

    #[test]
    fn identifier_preserves_text_span() {
        let source = "Main";
        let lexed = lex(source);
        assert!(lexed.errors.is_empty());
        assert_eq!(lexed.tokens[0].kind, TokenKind::Identifier);
        assert_eq!(lexed.tokens[0].text(source), "Main");
    }
}
