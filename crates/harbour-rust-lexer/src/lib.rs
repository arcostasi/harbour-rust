use std::fmt;

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
    PlusEq,
    Increment,
    Minus,
    MinusEq,
    Decrement,
    Star,
    StarEq,
    Slash,
    SlashEq,
    Percent,
    PercentEq,
    Caret,
    CaretEq,
    Power,
    PowerEq,
    Dollar,
    Hash,
    At,
    Ampersand,
    Pipe,
    Bang,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    ExactEqual,
    NotEqual,
    InAssign,
    Alias,
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

impl LexError {
    pub fn line(&self) -> usize {
        self.span.start.line
    }

    pub fn column(&self) -> usize {
        self.span.start.column
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at line {}, column {}",
            self.message,
            self.line(),
            self.column()
        )
    }
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
                '.' => self.lex_dot_prefixed_token(),
                '?' => self.push_single_char(TokenKind::Question),
                '(' => self.push_single_char(TokenKind::LeftParen),
                ')' => self.push_single_char(TokenKind::RightParen),
                '[' => self.push_single_char(TokenKind::LeftBracket),
                ']' => self.push_single_char(TokenKind::RightBracket),
                '{' => self.push_single_char(TokenKind::LeftBrace),
                '}' => self.push_single_char(TokenKind::RightBrace),
                ',' => self.push_single_char(TokenKind::Comma),
                ';' => self.push_single_char(TokenKind::Semicolon),
                ':' => self.lex_colon_or_assign(),
                '+' => self.lex_plus_family(),
                '-' => self.lex_minus_family(),
                '*' => self.lex_star_or_comment(),
                '/' => self.lex_slash_family(),
                '%' => self.lex_percent_family(),
                '^' => self.lex_caret_family(),
                '$' => self.push_single_char(TokenKind::Dollar),
                '#' => self.push_single_char(TokenKind::Hash),
                '@' => self.push_single_char(TokenKind::At),
                '&' => self.lex_ampersand_or_comment(),
                '|' => self.push_single_char(TokenKind::Pipe),
                '!' => self.lex_bang_family(),
                '<' => self.lex_less_family(),
                '>' => self.lex_greater_family(),
                '=' => self.lex_equal_family(),
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

        let text = &self.source[start.offset..self.position.offset];
        self.tokens.push(Token {
            kind: match_keyword(text).map_or(TokenKind::Identifier, TokenKind::Keyword),
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

    fn lex_line_comment(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch == '\r' || ch == '\n' {
                break;
            }
            self.bump_char();
        }
    }

    fn lex_block_comment(&mut self, start: Position) {
        loop {
            match self.peek_char() {
                Some('*') if self.peek_second_char() == Some('/') => {
                    self.bump_char();
                    self.bump_char();
                    return;
                }
                Some(_) => {
                    self.bump_char();
                }
                None => {
                    self.errors.push(LexError {
                        kind: LexErrorKind::UnterminatedBlockComment,
                        span: Span {
                            start,
                            end: self.position,
                        },
                        message: "unterminated block comment".to_owned(),
                    });
                    return;
                }
            }
        }
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

    fn push_consumed(&mut self, kind: TokenKind, start: Position) {
        self.tokens.push(Token {
            kind,
            span: Span {
                start,
                end: self.position,
            },
        });
    }

    fn lex_dot_prefixed_token(&mut self) {
        let start = self.position;
        self.bump_char();

        let Some(ch) = self.peek_char() else {
            self.push_consumed(TokenKind::Dot, start);
            return;
        };

        if !ch.is_ascii_alphabetic() {
            self.push_consumed(TokenKind::Dot, start);
            return;
        }

        while let Some(ch) = self.peek_char() {
            if !ch.is_ascii_alphabetic() {
                break;
            }
            self.bump_char();
        }

        if self.peek_char() == Some('.') {
            self.bump_char();
            let text = &self.source[start.offset..self.position.offset];
            if let Some(kind) = match_dot_keyword(text) {
                self.push_consumed(TokenKind::Keyword(kind), start);
                return;
            }
        }

        while self.position.offset > start.offset + 1 {
            self.rewind_one_char();
        }
        self.push_consumed(TokenKind::Dot, start);
    }

    fn lex_colon_or_assign(&mut self) {
        let start = self.position;
        self.bump_char();

        if self.peek_char() == Some('=') {
            self.bump_char();
            self.push_consumed(TokenKind::InAssign, start);
            return;
        }

        self.push_consumed(TokenKind::Colon, start);
    }

    fn lex_plus_family(&mut self) {
        let start = self.position;
        self.bump_char();
        let kind = match self.peek_char() {
            Some('+') => {
                self.bump_char();
                TokenKind::Increment
            }
            Some('=') => {
                self.bump_char();
                TokenKind::PlusEq
            }
            _ => TokenKind::Plus,
        };
        self.push_consumed(kind, start);
    }

    fn lex_minus_family(&mut self) {
        let start = self.position;
        self.bump_char();
        let kind = match self.peek_char() {
            Some('-') => {
                self.bump_char();
                TokenKind::Decrement
            }
            Some('=') => {
                self.bump_char();
                TokenKind::MinusEq
            }
            Some('>') => {
                self.bump_char();
                TokenKind::Alias
            }
            _ => TokenKind::Minus,
        };
        self.push_consumed(kind, start);
    }

    fn lex_star_family(&mut self) {
        let start = self.position;
        self.bump_char();
        let kind = match self.peek_char() {
            Some('*') => {
                self.bump_char();
                if self.peek_char() == Some('=') {
                    self.bump_char();
                    TokenKind::PowerEq
                } else {
                    TokenKind::Power
                }
            }
            Some('=') => {
                self.bump_char();
                TokenKind::StarEq
            }
            _ => TokenKind::Star,
        };
        self.push_consumed(kind, start);
    }

    fn lex_star_or_comment(&mut self) {
        if self.is_at_line_start() {
            self.lex_line_comment();
            return;
        }

        self.lex_star_family();
    }

    fn lex_slash_family(&mut self) {
        let start = self.position;
        self.bump_char();
        let kind = match self.peek_char() {
            Some('/') => {
                self.bump_char();
                self.lex_line_comment();
                return;
            }
            Some('*') => {
                self.bump_char();
                self.lex_block_comment(start);
                return;
            }
            Some('=') => {
                self.bump_char();
                TokenKind::SlashEq
            }
            _ => TokenKind::Slash,
        };
        self.push_consumed(kind, start);
    }

    fn lex_ampersand_or_comment(&mut self) {
        let start = self.position;
        self.bump_char();

        if self.peek_char() == Some('&') {
            self.bump_char();
            self.lex_line_comment();
            return;
        }

        self.push_consumed(TokenKind::Ampersand, start);
    }

    fn lex_percent_family(&mut self) {
        let start = self.position;
        self.bump_char();
        let kind = match self.peek_char() {
            Some('=') => {
                self.bump_char();
                TokenKind::PercentEq
            }
            _ => TokenKind::Percent,
        };
        self.push_consumed(kind, start);
    }

    fn lex_caret_family(&mut self) {
        let start = self.position;
        self.bump_char();
        let kind = match self.peek_char() {
            Some('=') => {
                self.bump_char();
                TokenKind::CaretEq
            }
            _ => TokenKind::Caret,
        };
        self.push_consumed(kind, start);
    }

    fn lex_less_family(&mut self) {
        let start = self.position;
        self.bump_char();
        let kind = match self.peek_char() {
            Some('=') => {
                self.bump_char();
                TokenKind::LessEqual
            }
            Some('>') => {
                self.bump_char();
                TokenKind::NotEqual
            }
            _ => TokenKind::Less,
        };
        self.push_consumed(kind, start);
    }

    fn lex_bang_family(&mut self) {
        let start = self.position;
        self.bump_char();
        let kind = match self.peek_char() {
            Some('=') => {
                self.bump_char();
                TokenKind::NotEqual
            }
            _ => TokenKind::Bang,
        };
        self.push_consumed(kind, start);
    }

    fn lex_greater_family(&mut self) {
        let start = self.position;
        self.bump_char();
        let kind = match self.peek_char() {
            Some('=') => {
                self.bump_char();
                TokenKind::GreaterEqual
            }
            _ => TokenKind::Greater,
        };
        self.push_consumed(kind, start);
    }

    fn lex_equal_family(&mut self) {
        let start = self.position;
        self.bump_char();
        let kind = match self.peek_char() {
            Some('=') => {
                self.bump_char();
                TokenKind::ExactEqual
            }
            _ => TokenKind::Equal,
        };
        self.push_consumed(kind, start);
    }

    fn peek_char(&self) -> Option<char> {
        self.source[self.position.offset..].chars().next()
    }

    fn peek_second_char(&self) -> Option<char> {
        let mut chars = self.source[self.position.offset..].chars();
        chars.next()?;
        chars.next()
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

    fn rewind_one_char(&mut self) {
        let ch = self.source[..self.position.offset]
            .chars()
            .next_back()
            .expect("rewind requested on empty prefix");
        self.position.offset -= ch.len_utf8();
        self.position.column -= 1;
    }

    fn is_at_line_start(&self) -> bool {
        let prefix = &self.source[..self.position.offset];
        let line_start = prefix.rfind(['\n', '\r']).map_or(0, |index| index + 1);
        prefix[line_start..]
            .chars()
            .all(|ch| ch == ' ' || ch == '\t')
    }
}

fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

fn match_keyword(text: &str) -> Option<Keyword> {
    if text.eq_ignore_ascii_case("PROCEDURE") {
        Some(Keyword::Procedure)
    } else if text.eq_ignore_ascii_case("FUNCTION") {
        Some(Keyword::Function)
    } else if text.eq_ignore_ascii_case("LOCAL") {
        Some(Keyword::Local)
    } else if text.eq_ignore_ascii_case("RETURN") {
        Some(Keyword::Return)
    } else if text.eq_ignore_ascii_case("IF") {
        Some(Keyword::If)
    } else if text.eq_ignore_ascii_case("ELSE") {
        Some(Keyword::Else)
    } else if text.eq_ignore_ascii_case("ENDIF") {
        Some(Keyword::EndIf)
    } else if text.eq_ignore_ascii_case("DO") {
        Some(Keyword::Do)
    } else if text.eq_ignore_ascii_case("WHILE") {
        Some(Keyword::While)
    } else if text.eq_ignore_ascii_case("ENDDO") {
        Some(Keyword::EndDo)
    } else if text.eq_ignore_ascii_case("FOR") {
        Some(Keyword::For)
    } else if text.eq_ignore_ascii_case("NEXT") {
        Some(Keyword::Next)
    } else if text.eq_ignore_ascii_case("STATIC") {
        Some(Keyword::Static)
    } else if text.eq_ignore_ascii_case("PUBLIC") {
        Some(Keyword::Public)
    } else if text.eq_ignore_ascii_case("PRIVATE") {
        Some(Keyword::Private)
    } else if text.eq_ignore_ascii_case("NIL") {
        Some(Keyword::Nil)
    } else if text.eq_ignore_ascii_case("AND") {
        Some(Keyword::And)
    } else if text.eq_ignore_ascii_case("OR") {
        Some(Keyword::Or)
    } else if text.eq_ignore_ascii_case("NOT") {
        Some(Keyword::Not)
    } else {
        None
    }
}

fn match_dot_keyword(text: &str) -> Option<Keyword> {
    if text.eq_ignore_ascii_case(".T.") || text.eq_ignore_ascii_case(".Y.") {
        Some(Keyword::True)
    } else if text.eq_ignore_ascii_case(".F.") || text.eq_ignore_ascii_case(".N.") {
        Some(Keyword::False)
    } else if text.eq_ignore_ascii_case(".AND.") {
        Some(Keyword::And)
    } else if text.eq_ignore_ascii_case(".OR.") {
        Some(Keyword::Or)
    } else if text.eq_ignore_ascii_case(".NOT.") {
        Some(Keyword::Not)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Keyword, Position, TokenKind, lex};

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

    #[test]
    fn keywords_are_case_insensitive() {
        let lexed = lex("procedure Main return nil");
        assert!(lexed.errors.is_empty());
        assert_eq!(lexed.tokens[0].kind, TokenKind::Keyword(Keyword::Procedure));
        assert_eq!(lexed.tokens[1].kind, TokenKind::Identifier);
        assert_eq!(lexed.tokens[2].kind, TokenKind::Keyword(Keyword::Return));
        assert_eq!(lexed.tokens[3].kind, TokenKind::Keyword(Keyword::Nil));
    }

    #[test]
    fn lexes_multi_character_operators() {
        let source = "x := y ++ <= >= <> == != -> += -= *= /= %= ^= ** **=";
        let kinds: Vec<_> = lex(source)
            .tokens
            .into_iter()
            .map(|token| token.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Identifier,
                TokenKind::InAssign,
                TokenKind::Identifier,
                TokenKind::Increment,
                TokenKind::LessEqual,
                TokenKind::GreaterEqual,
                TokenKind::NotEqual,
                TokenKind::ExactEqual,
                TokenKind::NotEqual,
                TokenKind::Alias,
                TokenKind::PlusEq,
                TokenKind::MinusEq,
                TokenKind::StarEq,
                TokenKind::SlashEq,
                TokenKind::PercentEq,
                TokenKind::CaretEq,
                TokenKind::Power,
                TokenKind::PowerEq,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_dot_delimited_logicals() {
        let kinds: Vec<_> = lex(".T. .F. .AND. .OR. .NOT.")
            .tokens
            .into_iter()
            .map(|token| token.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Keyword(Keyword::True),
                TokenKind::Keyword(Keyword::False),
                TokenKind::Keyword(Keyword::And),
                TokenKind::Keyword(Keyword::Or),
                TokenKind::Keyword(Keyword::Not),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_integer_float_and_strings() {
        let source = "42 10.5 \"hello\" `clipper'";
        let lexed = lex(source);
        assert!(lexed.errors.is_empty());
        assert_eq!(lexed.tokens[0].kind, TokenKind::Integer);
        assert_eq!(lexed.tokens[0].text(source), "42");
        assert_eq!(lexed.tokens[1].kind, TokenKind::Float);
        assert_eq!(lexed.tokens[1].text(source), "10.5");
        assert_eq!(lexed.tokens[2].kind, TokenKind::String);
        assert_eq!(lexed.tokens[2].text(source), "\"hello\"");
        assert_eq!(lexed.tokens[3].kind, TokenKind::String);
        assert_eq!(lexed.tokens[3].text(source), "`clipper'");
    }

    #[test]
    fn skips_line_and_block_comments_but_keeps_newlines() {
        let source = "// hello\r\n&& world\r\n/* block */\r\n   * note\r\nx";
        let kinds: Vec<_> = lex(source)
            .tokens
            .into_iter()
            .map(|token| token.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Newline,
                TokenKind::Identifier,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn invalid_character_reports_line_and_column() {
        let lexed = lex("PROCEDURE Main()\n@\n~");
        assert_eq!(lexed.errors.len(), 1);
        assert_eq!(
            lexed.errors[0].kind,
            super::LexErrorKind::InvalidCharacter('~')
        );
        assert_eq!(lexed.errors[0].line(), 3);
        assert_eq!(lexed.errors[0].column(), 1);
        assert_eq!(
            lexed.errors[0].to_string(),
            "invalid character `~` at line 3, column 1"
        );
    }

    #[test]
    fn unterminated_string_reports_origin() {
        let lexed = lex("\"hello");
        assert_eq!(lexed.errors.len(), 1);
        assert_eq!(
            lexed.errors[0].kind,
            super::LexErrorKind::UnterminatedString
        );
        assert_eq!(lexed.errors[0].line(), 1);
        assert_eq!(lexed.errors[0].column(), 1);
    }

    #[test]
    fn unterminated_block_comment_reports_origin() {
        let lexed = lex("/* comment");
        assert_eq!(lexed.errors.len(), 1);
        assert_eq!(
            lexed.errors[0].kind,
            super::LexErrorKind::UnterminatedBlockComment
        );
        assert_eq!(lexed.errors[0].line(), 1);
        assert_eq!(lexed.errors[0].column(), 1);
        assert_eq!(
            lexed.errors[0].to_string(),
            "unterminated block comment at line 1, column 1"
        );
    }
}
