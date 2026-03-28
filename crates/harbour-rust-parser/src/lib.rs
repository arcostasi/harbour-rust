use harbour_rust_ast::{
    AssignmentExpression, CallExpression, Expression, ExpressionStatement, FloatLiteral,
    Identifier, IntegerLiteral, Item, LogicalLiteral, NilLiteral, PrintStatement, Program,
    ReturnStatement, Routine, RoutineKind, Statement, StringLiteral,
};
use harbour_rust_lexer::{Keyword, LexErrorKind, Span, Token, TokenKind, lex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseOutput {
    pub program: Program,
    pub errors: Vec<ParseError>,
}

pub fn parse(source: &str) -> ParseOutput {
    let lexed = lex(source);
    let mut errors: Vec<ParseError> = lexed
        .errors
        .iter()
        .map(|error| ParseError {
            message: match error.kind {
                LexErrorKind::InvalidCharacter(_) => error.message.clone(),
                LexErrorKind::UnterminatedString => error.message.clone(),
                LexErrorKind::UnterminatedBlockComment => error.message.clone(),
            },
            span: error.span,
        })
        .collect();

    let mut parser = Parser {
        source,
        tokens: lexed.tokens,
        cursor: 0,
        errors: Vec::new(),
    };
    let program = parser.parse_program();
    errors.extend(parser.errors);

    ParseOutput { program, errors }
}

struct Parser<'src> {
    source: &'src str,
    tokens: Vec<Token>,
    cursor: usize,
    errors: Vec<ParseError>,
}

impl<'src> Parser<'src> {
    fn parse_program(&mut self) -> Program {
        let mut items = Vec::new();
        self.skip_separators();

        while !self.at(TokenKind::Eof) {
            match self.parse_routine() {
                Some(routine) => items.push(Item::Routine(routine)),
                None => self.synchronize_to_next_routine(),
            }
            self.skip_separators();
        }

        Program { items }
    }

    fn parse_routine(&mut self) -> Option<Routine> {
        let start = self.current().span.start;
        let kind = if self.match_keyword(Keyword::Procedure) {
            RoutineKind::Procedure
        } else if self.match_keyword(Keyword::Function) {
            RoutineKind::Function
        } else {
            let token = self.current();
            self.errors.push(ParseError {
                message: "expected PROCEDURE or FUNCTION".to_owned(),
                span: token.span,
            });
            return None;
        };

        let name = self.parse_identifier()?;
        self.expect(TokenKind::LeftParen, "expected `(` after routine name")?;
        let params = self.parse_parameter_list()?;
        let end_paren = self.expect(TokenKind::RightParen, "expected `)` after parameter list")?;
        self.skip_separators();

        let mut body = Vec::new();
        while !self.at(TokenKind::Eof)
            && !self.at_keyword(Keyword::Procedure)
            && !self.at_keyword(Keyword::Function)
        {
            if self.at(TokenKind::Newline) || self.at(TokenKind::Semicolon) {
                self.skip_separators();
                continue;
            }

            match self.parse_statement() {
                Some(statement) => body.push(statement),
                None => self.synchronize_statement(),
            }
            self.skip_separators();
        }

        let end = body
            .last()
            .map_or(end_paren.span.end, |statement| statement.span().end);
        Some(Routine {
            kind,
            name,
            params,
            body,
            span: Span { start, end },
        })
    }

    fn parse_parameter_list(&mut self) -> Option<Vec<Identifier>> {
        let mut params = Vec::new();
        if self.at(TokenKind::RightParen) {
            return Some(params);
        }

        loop {
            params.push(self.parse_identifier()?);
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        Some(params)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if self.match_keyword(Keyword::Return) {
            return Some(self.parse_return_statement());
        }

        if self.match_token(TokenKind::Question) {
            return self.parse_print_statement();
        }

        let expression = self.parse_expression()?;
        let span = expression.span();
        Some(Statement::Expression(ExpressionStatement {
            expression,
            span,
        }))
    }

    fn parse_return_statement(&mut self) -> Statement {
        let start = self.previous().span.start;
        if self.at(TokenKind::Newline)
            || self.at(TokenKind::Semicolon)
            || self.at(TokenKind::Eof)
            || self.at_keyword(Keyword::Procedure)
            || self.at_keyword(Keyword::Function)
        {
            return Statement::Return(ReturnStatement {
                value: None,
                span: Span {
                    start,
                    end: self.previous().span.end,
                },
            });
        }

        let value = self.parse_expression();
        let end = value
            .as_ref()
            .map_or(self.previous().span.end, |expression| expression.span().end);

        Statement::Return(ReturnStatement {
            value,
            span: Span { start, end },
        })
    }

    fn parse_print_statement(&mut self) -> Option<Statement> {
        let start = self.previous().span.start;
        let mut arguments = Vec::new();

        while !self.at(TokenKind::Newline)
            && !self.at(TokenKind::Semicolon)
            && !self.at(TokenKind::Eof)
            && !self.at_keyword(Keyword::Procedure)
            && !self.at_keyword(Keyword::Function)
        {
            arguments.push(self.parse_expression()?);
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        let end = arguments
            .last()
            .map_or(self.previous().span.end, |expression| expression.span().end);
        Some(Statement::Print(PrintStatement {
            arguments,
            span: Span { start, end },
        }))
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Option<Expression> {
        let left = self.parse_call()?;
        if !self.match_token(TokenKind::InAssign) {
            return Some(left);
        }

        let value = self.parse_assignment()?;
        let span = Span {
            start: left.span().start,
            end: value.span().end,
        };

        Some(Expression::Assignment(AssignmentExpression {
            target: Box::new(left),
            value: Box::new(value),
            span,
        }))
    }

    fn parse_call(&mut self) -> Option<Expression> {
        let mut expression = self.parse_primary()?;

        loop {
            if !self.match_token(TokenKind::LeftParen) {
                break;
            }

            let mut arguments = Vec::new();
            if !self.at(TokenKind::RightParen) {
                loop {
                    arguments.push(self.parse_expression()?);
                    if !self.match_token(TokenKind::Comma) {
                        break;
                    }
                }
            }

            let right_paren = self.expect(TokenKind::RightParen, "expected `)` after arguments")?;
            let span = Span {
                start: expression.span().start,
                end: right_paren.span.end,
            };
            expression = Expression::Call(CallExpression {
                callee: Box::new(expression),
                arguments,
                span,
            });
        }

        Some(expression)
    }

    fn parse_primary(&mut self) -> Option<Expression> {
        let token = *self.current();
        match token.kind {
            TokenKind::Identifier => {
                self.bump();
                Some(Expression::Identifier(Identifier {
                    text: token.text(self.source).to_owned(),
                    span: token.span,
                }))
            }
            TokenKind::Keyword(Keyword::Nil) => {
                self.bump();
                Some(Expression::Nil(NilLiteral { span: token.span }))
            }
            TokenKind::Keyword(Keyword::True) => {
                self.bump();
                Some(Expression::Logical(LogicalLiteral {
                    value: true,
                    span: token.span,
                }))
            }
            TokenKind::Keyword(Keyword::False) => {
                self.bump();
                Some(Expression::Logical(LogicalLiteral {
                    value: false,
                    span: token.span,
                }))
            }
            TokenKind::Integer => {
                self.bump();
                Some(Expression::Integer(IntegerLiteral {
                    lexeme: token.text(self.source).to_owned(),
                    span: token.span,
                }))
            }
            TokenKind::Float => {
                self.bump();
                Some(Expression::Float(FloatLiteral {
                    lexeme: token.text(self.source).to_owned(),
                    span: token.span,
                }))
            }
            TokenKind::String => {
                self.bump();
                Some(Expression::String(StringLiteral {
                    lexeme: token.text(self.source).to_owned(),
                    span: token.span,
                }))
            }
            _ => {
                self.errors.push(ParseError {
                    message: "expected expression".to_owned(),
                    span: token.span,
                });
                None
            }
        }
    }

    fn parse_identifier(&mut self) -> Option<Identifier> {
        let token = *self.current();
        if !matches!(token.kind, TokenKind::Identifier) {
            self.errors.push(ParseError {
                message: "expected identifier".to_owned(),
                span: token.span,
            });
            return None;
        }

        self.bump();
        Some(Identifier {
            text: token.text(self.source).to_owned(),
            span: token.span,
        })
    }

    fn expect(&mut self, kind: TokenKind, message: &str) -> Option<Token> {
        if self.at(kind) {
            let token = *self.current();
            self.bump();
            return Some(token);
        }

        self.errors.push(ParseError {
            message: message.to_owned(),
            span: self.current().span,
        });
        None
    }

    fn synchronize_statement(&mut self) {
        while !self.at(TokenKind::Eof)
            && !self.at(TokenKind::Newline)
            && !self.at(TokenKind::Semicolon)
            && !self.at_keyword(Keyword::Procedure)
            && !self.at_keyword(Keyword::Function)
        {
            self.bump();
        }
    }

    fn synchronize_to_next_routine(&mut self) {
        while !self.at(TokenKind::Eof)
            && !self.at_keyword(Keyword::Procedure)
            && !self.at_keyword(Keyword::Function)
        {
            self.bump();
        }
    }

    fn skip_separators(&mut self) {
        while self.at(TokenKind::Newline) || self.at(TokenKind::Semicolon) {
            self.bump();
        }
    }

    fn match_keyword(&mut self, keyword: Keyword) -> bool {
        if self.at_keyword(keyword) {
            self.bump();
            return true;
        }
        false
    }

    fn at_keyword(&self, keyword: Keyword) -> bool {
        matches!(self.current().kind, TokenKind::Keyword(found) if found == keyword)
    }

    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.bump();
            return true;
        }
        false
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.current().kind == kind
    }

    fn bump(&mut self) {
        if self.cursor < self.tokens.len().saturating_sub(1) {
            self.cursor += 1;
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.cursor]
    }

    fn previous(&self) -> &Token {
        let index = self.cursor.saturating_sub(1);
        &self.tokens[index]
    }
}

#[cfg(test)]
mod tests {
    use harbour_rust_ast::{Expression, Item, RoutineKind, Statement};

    use crate::parse;

    #[test]
    fn parses_hello_procedure() {
        let source = r#"
PROCEDURE Main()

   ? "Hello, world!"

   RETURN
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        assert_eq!(parsed.program.items.len(), 1);

        let Item::Routine(routine) = &parsed.program.items[0];
        assert_eq!(routine.kind, RoutineKind::Procedure);
        assert_eq!(routine.name.text, "Main");
        assert!(routine.params.is_empty());
        assert_eq!(routine.body.len(), 2);
        assert!(matches!(&routine.body[0], Statement::Print(_)));
        assert!(matches!(&routine.body[1], Statement::Return(_)));
    }

    #[test]
    fn parses_function_with_return_value() {
        let source = r#"
FUNCTION Answer()
   RETURN 42
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);

        let Item::Routine(routine) = &parsed.program.items[0];
        assert_eq!(routine.kind, RoutineKind::Function);
        match &routine.body[0] {
            Statement::Return(statement) => {
                assert!(matches!(statement.value, Some(Expression::Integer(_))));
            }
            statement => panic!("expected return, found {statement:?}"),
        }
    }

    #[test]
    fn parses_print_and_expression_statements() {
        let source = r#"
PROCEDURE Main()
   ? "From Main()"
   Two( 1 )
   RETURN
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);

        let Item::Routine(routine) = &parsed.program.items[0];
        assert!(matches!(&routine.body[0], Statement::Print(_)));
        match &routine.body[1] {
            Statement::Expression(statement) => {
                assert!(matches!(statement.expression, Expression::Call(_)));
            }
            statement => panic!("expected expression statement, found {statement:?}"),
        }
    }

    #[test]
    fn reports_missing_routine_name() {
        let parsed = parse("PROCEDURE ()");
        assert_eq!(parsed.errors.len(), 1);
        assert_eq!(parsed.errors[0].message, "expected identifier");
    }
}
