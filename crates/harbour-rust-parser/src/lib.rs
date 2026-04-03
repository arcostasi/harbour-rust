use harbour_rust_ast::{
    ArrayLiteral, AssignmentExpression, BinaryExpression, BinaryOperator, CallExpression,
    CodeblockLiteral, ConditionalBranch, DoWhileStatement, Expression, ExpressionStatement,
    FloatLiteral, ForStatement, Identifier, IfStatement, IndexExpression, IntegerLiteral, Item,
    LocalBinding, LocalStatement, LogicalLiteral, MacroExpression, MemvarBinding, MemvarClass,
    MemvarStatement, NilLiteral, PostfixExpression, PostfixOperator, PrintStatement, Program,
    ReturnStatement, Routine, RoutineKind, Statement, StaticBinding, StaticStatement, StorageClass,
    StringLiteral, UnaryExpression, UnaryOperator,
};
use harbour_rust_lexer::{Keyword, LexErrorKind, Span, Token, TokenKind, lex};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl ParseError {
    pub fn line(&self) -> usize {
        self.span.start.line
    }

    pub fn column(&self) -> usize {
        self.span.start.column
    }
}

impl fmt::Display for ParseError {
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
            if self.match_keyword(Keyword::Static) {
                match self.parse_module_static_item() {
                    Some(statement) => items.push(Item::Static(statement)),
                    None => self.synchronize_to_next_item(),
                }
            } else {
                match self.parse_routine() {
                    Some(routine) => items.push(Item::Routine(routine)),
                    None => self.synchronize_to_next_item(),
                }
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
            self.error_current("expected PROCEDURE or FUNCTION");
            return None;
        };

        let name = self.parse_identifier()?;
        self.expect(TokenKind::LeftParen, "expected `(` after routine name")?;
        let params = self.parse_parameter_list()?;
        let end_paren = self.expect(TokenKind::RightParen, "expected `)` after parameter list")?;
        self.skip_separators();

        let body = self.parse_block_until(&[
            Terminator::Keyword(Keyword::Procedure),
            Terminator::Keyword(Keyword::Function),
            Terminator::Eof,
        ]);

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

    fn parse_block_until(&mut self, terminators: &[Terminator]) -> Vec<Statement> {
        let mut body = Vec::new();

        while !self.at(TokenKind::Eof) && !self.matches_any_terminator(terminators) {
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

        body
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if self.match_keyword(Keyword::Local) {
            return self.parse_local_statement();
        }

        if self.match_keyword(Keyword::Static) {
            return self.parse_static_statement();
        }

        if self.match_keyword(Keyword::Private) {
            return self.parse_memvar_statement(MemvarClass::Private);
        }

        if self.match_keyword(Keyword::Public) {
            return self.parse_memvar_statement(MemvarClass::Public);
        }

        if self.match_keyword(Keyword::If) {
            return self.parse_if_statement();
        }

        if self.match_keyword(Keyword::Do) {
            return self.parse_do_while_statement();
        }

        if self.match_keyword(Keyword::For) {
            return self.parse_for_statement();
        }

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

    fn parse_local_statement(&mut self) -> Option<Statement> {
        let start = self.previous().span.start;
        let mut bindings = Vec::new();

        loop {
            let name = self.parse_identifier()?;
            let binding_start = name.span.start;
            let initializer = if self.match_token(TokenKind::InAssign) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            let end = initializer
                .as_ref()
                .map_or(name.span.end, |expression| expression.span().end);
            bindings.push(LocalBinding {
                name,
                initializer,
                span: Span {
                    start: binding_start,
                    end,
                },
            });

            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        let end = bindings
            .last()
            .map_or(self.previous().span.end, |binding| binding.span.end);
        Some(Statement::Local(LocalStatement {
            storage_class: StorageClass::Local,
            bindings,
            span: Span { start, end },
        }))
    }

    fn parse_static_statement(&mut self) -> Option<Statement> {
        let (bindings, span) = self.parse_static_bindings(self.previous().span.start)?;
        Some(Statement::Static(StaticStatement {
            storage_class: StorageClass::Static,
            bindings,
            span,
        }))
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        let start = self.previous().span.start;
        let condition = self.parse_expression()?;
        self.expect_statement_break("expected newline after IF condition");
        let branch_body = self.parse_block_until(&[
            Terminator::Keyword(Keyword::Else),
            Terminator::Keyword(Keyword::EndIf),
            Terminator::Eof,
            Terminator::Keyword(Keyword::Procedure),
            Terminator::Keyword(Keyword::Function),
        ]);
        let branch_end = branch_body
            .last()
            .map_or(condition.span().end, |statement| statement.span().end);
        let branches = vec![ConditionalBranch {
            condition,
            body: branch_body,
            span: Span {
                start,
                end: branch_end,
            },
        }];

        let else_branch = if self.match_keyword(Keyword::Else) {
            self.expect_statement_break("expected newline after ELSE");
            Some(self.parse_block_until(&[
                Terminator::Keyword(Keyword::EndIf),
                Terminator::Eof,
                Terminator::Keyword(Keyword::Procedure),
                Terminator::Keyword(Keyword::Function),
            ]))
        } else {
            None
        };

        let end = self.expect_keyword_or_recover(
            Keyword::EndIf,
            "expected ENDIF after IF block",
            else_branch
                .as_ref()
                .and_then(|branch| branch.last().map(|statement| statement.span().end))
                .unwrap_or(branch_end),
        );
        Some(Statement::If(Box::new(IfStatement {
            branches,
            else_branch,
            span: Span { start, end },
        })))
    }

    fn parse_memvar_statement(&mut self, memvar_class: MemvarClass) -> Option<Statement> {
        let start = self.previous().span.start;
        let bindings = self.parse_memvar_bindings()?;
        let end = bindings
            .last()
            .map_or(self.previous().span.end, |binding| binding.span.end);
        let statement = MemvarStatement {
            memvar_class,
            bindings,
            span: Span { start, end },
        };

        Some(match memvar_class {
            MemvarClass::Private => Statement::Private(statement),
            MemvarClass::Public => Statement::Public(statement),
        })
    }

    fn parse_do_while_statement(&mut self) -> Option<Statement> {
        let start = self.previous().span.start;
        self.expect_keyword(Keyword::While, "expected WHILE after DO")?;
        let condition = self.parse_expression()?;
        self.expect_statement_break("expected newline after DO WHILE condition");
        let body = self.parse_block_until(&[
            Terminator::Keyword(Keyword::EndDo),
            Terminator::Eof,
            Terminator::Keyword(Keyword::Procedure),
            Terminator::Keyword(Keyword::Function),
        ]);
        let end = self.expect_keyword_or_recover(
            Keyword::EndDo,
            "expected ENDDO after DO WHILE block",
            body.last()
                .map_or(condition.span().end, |statement| statement.span().end),
        );

        Some(Statement::DoWhile(Box::new(DoWhileStatement {
            condition,
            body,
            span: Span { start, end },
        })))
    }

    fn parse_for_statement(&mut self) -> Option<Statement> {
        let start = self.previous().span.start;
        let variable = self.parse_identifier()?;
        self.expect(TokenKind::InAssign, "expected `:=` after FOR variable")?;
        let initial_value = self.parse_expression()?;
        self.expect_keyword(Keyword::To, "expected TO in FOR statement")?;
        let limit = self.parse_expression()?;
        let step = if self.match_keyword(Keyword::Step) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect_statement_break("expected newline after FOR header");
        let body = self.parse_block_until(&[
            Terminator::Keyword(Keyword::Next),
            Terminator::Eof,
            Terminator::Keyword(Keyword::Procedure),
            Terminator::Keyword(Keyword::Function),
        ]);
        let end = self.expect_keyword_or_recover(
            Keyword::Next,
            "expected NEXT after FOR block",
            body.last()
                .map_or(limit.span().end, |statement| statement.span().end),
        );

        Some(Statement::For(Box::new(ForStatement {
            variable,
            initial_value,
            limit,
            step,
            body,
            span: Span { start, end },
        })))
    }

    fn parse_return_statement(&mut self) -> Statement {
        let start = self.previous().span.start;
        if self.statement_should_end() {
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

        while !self.statement_should_end() {
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
        let left = self.parse_or()?;
        if self.match_token(TokenKind::InAssign) {
            let value = self.parse_assignment()?;
            let span = Span {
                start: left.span().start,
                end: value.span().end,
            };
            return Some(Expression::Assignment(AssignmentExpression {
                target: Box::new(left),
                value: Box::new(value),
                span,
            }));
        }

        if let Some(operator) = self.match_compound_assignment_operator() {
            let value = self.parse_assignment()?;
            return self.lower_compound_assignment(left, operator, value);
        }

        Some(left)
    }

    fn parse_or(&mut self) -> Option<Expression> {
        let mut expression = self.parse_and()?;
        while self.match_keyword(Keyword::Or) {
            let right = self.parse_and()?;
            let span = Span {
                start: expression.span().start,
                end: right.span().end,
            };
            expression = Expression::Binary(BinaryExpression {
                left: Box::new(expression),
                operator: BinaryOperator::Or,
                right: Box::new(right),
                span,
            });
        }
        Some(expression)
    }

    fn parse_and(&mut self) -> Option<Expression> {
        let mut expression = self.parse_equality()?;
        while self.match_keyword(Keyword::And) {
            let right = self.parse_equality()?;
            let span = Span {
                start: expression.span().start,
                end: right.span().end,
            };
            expression = Expression::Binary(BinaryExpression {
                left: Box::new(expression),
                operator: BinaryOperator::And,
                right: Box::new(right),
                span,
            });
        }
        Some(expression)
    }

    fn parse_equality(&mut self) -> Option<Expression> {
        let mut expression = self.parse_comparison()?;

        loop {
            let operator = if self.match_token(TokenKind::Equal) {
                Some(BinaryOperator::Equal)
            } else if self.match_token(TokenKind::ExactEqual) {
                Some(BinaryOperator::ExactEqual)
            } else if self.match_token(TokenKind::NotEqual) {
                Some(BinaryOperator::NotEqual)
            } else {
                None
            };

            let Some(operator) = operator else {
                break;
            };
            let right = self.parse_comparison()?;
            let span = Span {
                start: expression.span().start,
                end: right.span().end,
            };
            expression = Expression::Binary(BinaryExpression {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
                span,
            });
        }

        Some(expression)
    }

    fn parse_comparison(&mut self) -> Option<Expression> {
        let mut expression = self.parse_term()?;

        loop {
            let operator = if self.match_token(TokenKind::Less) {
                Some(BinaryOperator::Less)
            } else if self.match_token(TokenKind::LessEqual) {
                Some(BinaryOperator::LessEqual)
            } else if self.match_token(TokenKind::Greater) {
                Some(BinaryOperator::Greater)
            } else if self.match_token(TokenKind::GreaterEqual) {
                Some(BinaryOperator::GreaterEqual)
            } else {
                None
            };

            let Some(operator) = operator else {
                break;
            };
            let right = self.parse_term()?;
            let span = Span {
                start: expression.span().start,
                end: right.span().end,
            };
            expression = Expression::Binary(BinaryExpression {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
                span,
            });
        }

        Some(expression)
    }

    fn parse_term(&mut self) -> Option<Expression> {
        let mut expression = self.parse_factor()?;

        loop {
            let operator = if self.match_token(TokenKind::Plus) {
                Some(BinaryOperator::Add)
            } else if self.match_token(TokenKind::Minus) {
                Some(BinaryOperator::Subtract)
            } else {
                None
            };

            let Some(operator) = operator else {
                break;
            };
            let right = self.parse_factor()?;
            let span = Span {
                start: expression.span().start,
                end: right.span().end,
            };
            expression = Expression::Binary(BinaryExpression {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
                span,
            });
        }

        Some(expression)
    }

    fn parse_factor(&mut self) -> Option<Expression> {
        let mut expression = self.parse_unary()?;

        loop {
            let operator = if self.match_token(TokenKind::Star) {
                Some(BinaryOperator::Multiply)
            } else if self.match_token(TokenKind::Slash) {
                Some(BinaryOperator::Divide)
            } else if self.match_token(TokenKind::Percent) {
                Some(BinaryOperator::Modulo)
            } else if self.match_token(TokenKind::Caret) || self.match_token(TokenKind::Power) {
                Some(BinaryOperator::Power)
            } else {
                None
            };

            let Some(operator) = operator else {
                break;
            };
            let right = self.parse_unary()?;
            let span = Span {
                start: expression.span().start,
                end: right.span().end,
            };
            expression = Expression::Binary(BinaryExpression {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
                span,
            });
        }

        Some(expression)
    }

    fn parse_unary(&mut self) -> Option<Expression> {
        let operator = if self.match_token(TokenKind::Plus) {
            Some(UnaryOperator::Plus)
        } else if self.match_token(TokenKind::Minus) {
            Some(UnaryOperator::Minus)
        } else if self.match_keyword(Keyword::Not) {
            Some(UnaryOperator::Not)
        } else {
            None
        };

        if let Some(operator) = operator {
            let start = self.previous().span.start;
            let operand = self.parse_unary()?;
            let span = Span {
                start,
                end: operand.span().end,
            };
            return Some(Expression::Unary(UnaryExpression {
                operator,
                operand: Box::new(operand),
                span,
            }));
        }

        if self.match_token(TokenKind::Ampersand) {
            return self.parse_macro_expression();
        }

        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Option<Expression> {
        let mut expression = self.parse_call()?;

        loop {
            let operator = if self.match_token(TokenKind::Increment) {
                Some(PostfixOperator::Increment)
            } else if self.match_token(TokenKind::Decrement) {
                Some(PostfixOperator::Decrement)
            } else {
                None
            };

            let Some(operator) = operator else {
                break;
            };
            let span = Span {
                start: expression.span().start,
                end: self.previous().span.end,
            };
            expression = Expression::Postfix(PostfixExpression {
                operand: Box::new(expression),
                operator,
                span,
            });
        }

        Some(expression)
    }

    fn parse_call(&mut self) -> Option<Expression> {
        let mut expression = self.parse_primary()?;

        loop {
            if self.match_token(TokenKind::LeftParen) {
                let mut arguments = Vec::new();
                if !self.at(TokenKind::RightParen) {
                    loop {
                        arguments.push(self.parse_expression()?);
                        if !self.match_token(TokenKind::Comma) {
                            break;
                        }
                    }
                }

                let right_paren =
                    self.expect(TokenKind::RightParen, "expected `)` after arguments")?;
                let span = Span {
                    start: expression.span().start,
                    end: right_paren.span.end,
                };
                expression = Expression::Call(CallExpression {
                    callee: Box::new(expression),
                    arguments,
                    span,
                });
                continue;
            }

            if self.match_token(TokenKind::LeftBracket) {
                let mut indices = Vec::new();
                loop {
                    indices.push(self.parse_expression()?);
                    if !self.match_token(TokenKind::Comma) {
                        break;
                    }
                }

                let right_bracket =
                    self.expect(TokenKind::RightBracket, "expected `]` after array index")?;
                let span = Span {
                    start: expression.span().start,
                    end: right_bracket.span.end,
                };
                expression = Expression::Index(IndexExpression {
                    target: Box::new(expression),
                    indices,
                    span,
                });
                continue;
            }

            break;
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
            TokenKind::LeftBrace if self.peek_kind(1) == Some(TokenKind::Pipe) => {
                self.parse_codeblock_literal()
            }
            TokenKind::LeftBrace => self.parse_array_literal(),
            TokenKind::LeftParen => {
                self.bump();
                let expression = self.parse_expression()?;
                self.expect(TokenKind::RightParen, "expected `)` after expression")?;
                Some(expression)
            }
            _ => {
                self.error_current("expected expression");
                None
            }
        }
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        let start = self.current().span.start;
        self.bump();

        let mut elements = Vec::new();
        if !self.at(TokenKind::RightBrace) {
            loop {
                elements.push(self.parse_expression()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
        }

        let right_brace = self.expect(TokenKind::RightBrace, "expected `}` after array literal")?;
        Some(Expression::Array(ArrayLiteral {
            elements,
            span: Span {
                start,
                end: right_brace.span.end,
            },
        }))
    }

    fn parse_codeblock_literal(&mut self) -> Option<Expression> {
        let start = self.current().span.start;
        self.bump();
        self.expect(
            TokenKind::Pipe,
            "expected `|` after `{` in codeblock literal",
        )?;

        let mut params = Vec::new();
        if !self.match_token(TokenKind::Pipe) {
            loop {
                params.push(self.parse_identifier()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::Pipe, "expected `|` after codeblock parameters")?;
        }

        let mut body = Vec::new();
        if !self.at(TokenKind::RightBrace) {
            loop {
                body.push(self.parse_expression()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
        }

        let right_brace = self.expect(
            TokenKind::RightBrace,
            "expected `}` after codeblock literal",
        )?;
        Some(Expression::Codeblock(CodeblockLiteral {
            params,
            body,
            span: Span {
                start,
                end: right_brace.span.end,
            },
        }))
    }

    fn parse_macro_expression(&mut self) -> Option<Expression> {
        let start = self.previous().span.start;
        let value = if self.match_token(TokenKind::LeftParen) {
            let expression = self.parse_expression()?;
            self.expect(TokenKind::RightParen, "expected `)` after macro expression")?;
            expression
        } else {
            match self.current().kind {
                TokenKind::Identifier
                | TokenKind::String
                | TokenKind::LeftBrace
                | TokenKind::LeftParen => self.parse_primary()?,
                _ => {
                    self.error_current("expected identifier or parenthesized expression after `&`");
                    return None;
                }
            }
        };

        let span = Span {
            start,
            end: value.span().end,
        };
        Some(Expression::Macro(MacroExpression {
            value: Box::new(value),
            span,
        }))
    }

    fn lower_compound_assignment(
        &mut self,
        left: Expression,
        operator: BinaryOperator,
        value: Expression,
    ) -> Option<Expression> {
        let Expression::Identifier(identifier) = left else {
            self.errors.push(ParseError {
                message: "expected identifier before compound assignment operator".to_owned(),
                span: self.previous().span,
            });
            return None;
        };

        let target = Expression::Identifier(identifier.clone());
        let binary = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Identifier(identifier.clone())),
            operator,
            right: Box::new(value),
            span: Span {
                start: identifier.span.start,
                end: self.previous().span.end,
            },
        });
        let span = binary.span();

        Some(Expression::Assignment(AssignmentExpression {
            target: Box::new(target),
            value: Box::new(binary),
            span,
        }))
    }

    fn match_compound_assignment_operator(&mut self) -> Option<BinaryOperator> {
        if self.match_token(TokenKind::PlusEq) {
            Some(BinaryOperator::Add)
        } else if self.match_token(TokenKind::MinusEq) {
            Some(BinaryOperator::Subtract)
        } else if self.match_token(TokenKind::StarEq) {
            Some(BinaryOperator::Multiply)
        } else if self.match_token(TokenKind::SlashEq) {
            Some(BinaryOperator::Divide)
        } else if self.match_token(TokenKind::PercentEq) {
            Some(BinaryOperator::Modulo)
        } else if self.match_token(TokenKind::CaretEq) {
            Some(BinaryOperator::Power)
        } else {
            None
        }
    }

    fn parse_identifier(&mut self) -> Option<Identifier> {
        let token = *self.current();
        if !matches!(token.kind, TokenKind::Identifier) {
            self.error_current("expected identifier");
            return None;
        }

        self.bump();
        Some(Identifier {
            text: token.text(self.source).to_owned(),
            span: token.span,
        })
    }

    fn parse_memvar_bindings(&mut self) -> Option<Vec<MemvarBinding>> {
        let mut bindings = Vec::new();

        loop {
            let name = self.parse_identifier()?;
            let binding_start = name.span.start;
            let initializer = if self.match_token(TokenKind::InAssign) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            let end = initializer
                .as_ref()
                .map_or(name.span.end, |expression| expression.span().end);
            bindings.push(MemvarBinding {
                name,
                initializer,
                span: Span {
                    start: binding_start,
                    end,
                },
            });

            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        Some(bindings)
    }

    fn expect(&mut self, kind: TokenKind, message: &str) -> Option<Token> {
        if self.at(kind) {
            let token = *self.current();
            self.bump();
            return Some(token);
        }
        self.error_expected(message);
        None
    }

    fn expect_keyword(&mut self, keyword: Keyword, message: &str) -> Option<Token> {
        if self.at_keyword(keyword) {
            let token = *self.current();
            self.bump();
            return Some(token);
        }
        self.error_expected(message);
        None
    }

    fn expect_statement_break(&mut self, message: &str) {
        if self.at(TokenKind::Newline) || self.at(TokenKind::Semicolon) {
            self.skip_separators();
            return;
        }
        self.error_expected(message);
    }

    fn statement_should_end(&self) -> bool {
        self.at(TokenKind::Newline)
            || self.at(TokenKind::Semicolon)
            || self.at(TokenKind::Eof)
            || self.at_keyword(Keyword::Procedure)
            || self.at_keyword(Keyword::Function)
            || self.at_keyword(Keyword::Else)
            || self.at_keyword(Keyword::EndIf)
            || self.at_keyword(Keyword::EndDo)
            || self.at_keyword(Keyword::Next)
    }

    fn synchronize_statement(&mut self) {
        while !self.at(TokenKind::Eof)
            && !self.at(TokenKind::Newline)
            && !self.at(TokenKind::Semicolon)
            && !self.at_keyword(Keyword::Procedure)
            && !self.at_keyword(Keyword::Function)
            && !self.at_keyword(Keyword::Else)
            && !self.at_keyword(Keyword::EndIf)
            && !self.at_keyword(Keyword::EndDo)
            && !self.at_keyword(Keyword::Next)
        {
            self.bump();
        }
    }

    fn synchronize_to_next_item(&mut self) {
        while !self.at(TokenKind::Eof)
            && !self.at_keyword(Keyword::Static)
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

    fn parse_module_static_item(&mut self) -> Option<StaticStatement> {
        let (bindings, span) = self.parse_static_bindings(self.previous().span.start)?;
        Some(StaticStatement {
            storage_class: StorageClass::Static,
            bindings,
            span,
        })
    }

    fn parse_static_bindings(
        &mut self,
        start: harbour_rust_lexer::Position,
    ) -> Option<(Vec<StaticBinding>, Span)> {
        let mut bindings = Vec::new();

        loop {
            let name = self.parse_identifier()?;
            let binding_start = name.span.start;
            let initializer = if self.match_token(TokenKind::InAssign) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            let end = initializer
                .as_ref()
                .map_or(name.span.end, |expression| expression.span().end);
            bindings.push(StaticBinding {
                name,
                initializer,
                span: Span {
                    start: binding_start,
                    end,
                },
            });

            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }

        let end = bindings
            .last()
            .map_or(self.previous().span.end, |binding| binding.span.end);
        Some((bindings, Span { start, end }))
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

    fn matches_any_terminator(&self, terminators: &[Terminator]) -> bool {
        terminators.iter().any(|terminator| match terminator {
            Terminator::Keyword(keyword) => self.at_keyword(*keyword),
            Terminator::Eof => self.at(TokenKind::Eof),
        })
    }

    fn error_current(&mut self, message: &str) {
        self.errors.push(ParseError {
            message: message.to_owned(),
            span: self.current().span,
        });
    }

    fn error_expected(&mut self, message: &str) {
        let token = *self.current();
        self.errors.push(ParseError {
            message: format!("{message}; found {}", self.describe_token(token)),
            span: token.span,
        });
    }

    fn expect_keyword_or_recover(
        &mut self,
        keyword: Keyword,
        message: &str,
        fallback_end: harbour_rust_lexer::Position,
    ) -> harbour_rust_lexer::Position {
        if let Some(token) = self.expect_keyword(keyword, message) {
            token.span.end
        } else {
            fallback_end
        }
    }

    fn describe_token(&self, token: Token) -> String {
        match token.kind {
            TokenKind::Identifier
            | TokenKind::Integer
            | TokenKind::Float
            | TokenKind::String
            | TokenKind::Newline => format!("`{}`", token.text(self.source).escape_default()),
            TokenKind::Keyword(keyword) => format!("keyword {:?}", keyword),
            TokenKind::Eof => "end of file".to_owned(),
            other => format!("token {:?}", other),
        }
    }

    fn bump(&mut self) {
        if self.cursor < self.tokens.len().saturating_sub(1) {
            self.cursor += 1;
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.cursor]
    }

    fn peek_kind(&self, offset: usize) -> Option<TokenKind> {
        self.tokens
            .get(self.cursor + offset)
            .map(|token| token.kind)
    }

    fn previous(&self) -> &Token {
        let index = self.cursor.saturating_sub(1);
        &self.tokens[index]
    }
}

#[derive(Clone, Copy)]
enum Terminator {
    Keyword(Keyword),
    Eof,
}

#[cfg(test)]
mod tests {
    use harbour_rust_ast::{
        BinaryOperator, Expression, Item, MemvarClass, RoutineKind, Statement, StorageClass,
    };

    use crate::parse;

    #[test]
    fn parses_local_if_while_and_for_statements() {
        let source = r#"
PROCEDURE Main()
   LOCAL x := 0, done
   IF x == 0
      ? "start"
   ELSE
      ? "other"
   ENDIF
   DO WHILE x++ < 10
      ? x
   ENDDO
   FOR x := 1 TO 10 STEP 2
      ? x
   NEXT
   RETURN
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);

        let Item::Routine(routine) = &parsed.program.items[0] else {
            panic!("expected routine item");
        };
        assert_eq!(routine.kind, RoutineKind::Procedure);
        assert!(matches!(&routine.body[0], Statement::Local(_)));
        assert!(matches!(&routine.body[1], Statement::If(_)));
        assert!(matches!(&routine.body[2], Statement::DoWhile(_)));
        assert!(matches!(&routine.body[3], Statement::For(_)));
    }

    #[test]
    fn parses_static_statement_with_initializer_list() {
        let source = r#"
PROCEDURE Main()
   STATIC cache := "memo", hits := 0
   RETURN
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);

        let Item::Routine(routine) = &parsed.program.items[0] else {
            panic!("expected routine item");
        };
        match &routine.body[0] {
            Statement::Static(statement) => {
                assert_eq!(statement.storage_class, StorageClass::Static);
                assert_eq!(statement.bindings.len(), 2);
                assert_eq!(statement.bindings[0].name.text, "cache");
                assert!(matches!(
                    statement.bindings[0].initializer,
                    Some(Expression::String(_))
                ));
                assert_eq!(statement.bindings[1].name.text, "hits");
                assert!(matches!(
                    statement.bindings[1].initializer,
                    Some(Expression::Integer(_))
                ));
            }
            statement => panic!("expected static statement, found {statement:?}"),
        }
    }

    #[test]
    fn parses_module_level_static_declaration_before_routine() {
        let source = r#"
STATIC s_count := 0

PROCEDURE Main()
   RETURN
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        assert_eq!(parsed.program.items.len(), 2);

        let Item::Static(statement) = &parsed.program.items[0] else {
            panic!("expected module static item");
        };
        assert_eq!(statement.storage_class, StorageClass::Static);
        assert_eq!(statement.bindings.len(), 1);
        assert_eq!(statement.bindings[0].name.text, "s_count");
        assert!(matches!(
            statement.bindings[0].initializer,
            Some(Expression::Integer(_))
        ));

        let Item::Routine(routine) = &parsed.program.items[1] else {
            panic!("expected routine item");
        };
        assert_eq!(routine.name.text, "Main");
    }

    #[test]
    fn parses_empty_and_comma_separated_array_literals() {
        let source = r#"
FUNCTION Build()
   RETURN { {}, { 1, "x", cache } }
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);

        let Item::Routine(routine) = &parsed.program.items[0] else {
            panic!("expected routine item");
        };
        let Statement::Return(statement) = &routine.body[0] else {
            panic!("expected return statement");
        };
        let Some(Expression::Array(outer)) = &statement.value else {
            panic!("expected array literal");
        };

        assert_eq!(outer.elements.len(), 2);
        assert!(
            matches!(outer.elements[0], Expression::Array(ref array) if array.elements.is_empty())
        );
        assert!(
            matches!(outer.elements[1], Expression::Array(ref array) if array.elements.len() == 3)
        );
    }

    #[test]
    fn reports_missing_right_brace_in_array_literal() {
        let source = r#"
FUNCTION Build()
   RETURN { 1, 2
"#;
        let parsed = parse(source);

        assert_eq!(parsed.errors.len(), 1);
        assert_eq!(
            parsed.errors[0].to_string(),
            "expected `}` after array literal; found `\\n` at line 3, column 17"
        );
    }

    #[test]
    fn parses_compound_assignment_for_identifier_targets() {
        let source = r#"
PROCEDURE Main()
   LOCAL total := 1
   STATIC factor := 2
   total += 3
   factor *= total
   RETURN factor
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);

        let Item::Routine(routine) = &parsed.program.items[0] else {
            panic!("expected routine item");
        };
        let Statement::Expression(total_update) = &routine.body[2] else {
            panic!("expected expression statement for total update");
        };
        let Expression::Assignment(total_assign) = &total_update.expression else {
            panic!("expected assignment expression");
        };
        assert!(
            matches!(total_assign.target.as_ref(), Expression::Identifier(identifier) if identifier.text == "total")
        );
        assert!(matches!(
            total_assign.value.as_ref(),
            Expression::Binary(binary)
                if binary.operator == BinaryOperator::Add
                    && matches!(binary.left.as_ref(), Expression::Identifier(identifier) if identifier.text == "total")
        ));

        let Statement::Expression(factor_update) = &routine.body[3] else {
            panic!("expected expression statement for factor update");
        };
        let Expression::Assignment(factor_assign) = &factor_update.expression else {
            panic!("expected assignment expression");
        };
        assert!(
            matches!(factor_assign.target.as_ref(), Expression::Identifier(identifier) if identifier.text == "factor")
        );
        assert!(matches!(
            factor_assign.value.as_ref(),
            Expression::Binary(binary)
                if binary.operator == BinaryOperator::Multiply
                    && matches!(binary.left.as_ref(), Expression::Identifier(identifier) if identifier.text == "factor")
        ));
    }

    #[test]
    fn parses_array_indexing_expressions() {
        let source = r#"
FUNCTION Pick(row, col)
   RETURN Build()[row][1 + col]
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);

        let Item::Routine(routine) = &parsed.program.items[0] else {
            panic!("expected routine item");
        };
        let Statement::Return(statement) = &routine.body[0] else {
            panic!("expected return statement");
        };
        let Some(Expression::Index(outer_index)) = &statement.value else {
            panic!("expected outer index expression");
        };
        assert_eq!(outer_index.indices.len(), 1);
        assert!(matches!(
            outer_index.indices[0],
            Expression::Binary(ref binary) if binary.operator == BinaryOperator::Add
        ));

        let Expression::Index(inner_index) = outer_index.target.as_ref() else {
            panic!("expected nested index expression");
        };
        assert_eq!(inner_index.indices.len(), 1);
        assert!(matches!(
            inner_index.indices[0],
            Expression::Identifier(ref identifier) if identifier.text == "row"
        ));
        assert!(matches!(inner_index.target.as_ref(), Expression::Call(_)));
    }

    #[test]
    fn parses_private_and_public_memvar_statements() {
        let source = r#"
PROCEDURE Main()
   PRIVATE counter := 1, name := "inner"
   PUBLIC g_count := 10, g_label
   RETURN
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);

        let Item::Routine(routine) = &parsed.program.items[0] else {
            panic!("expected routine item");
        };
        match &routine.body[0] {
            Statement::Private(statement) => {
                assert_eq!(statement.memvar_class, MemvarClass::Private);
                assert_eq!(statement.bindings.len(), 2);
            }
            statement => panic!("expected private statement, found {statement:?}"),
        }
        match &routine.body[1] {
            Statement::Public(statement) => {
                assert_eq!(statement.memvar_class, MemvarClass::Public);
                assert_eq!(statement.bindings.len(), 2);
            }
            statement => panic!("expected public statement, found {statement:?}"),
        }
    }

    #[test]
    fn parses_codeblock_literals_with_params_and_nested_blocks() {
        let source = r#"
FUNCTION Build()
   RETURN {|x, y| x + y, {|| x} }
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);

        let Item::Routine(routine) = &parsed.program.items[0] else {
            panic!("expected routine item");
        };
        let Statement::Return(statement) = &routine.body[0] else {
            panic!("expected return statement");
        };
        let Some(Expression::Codeblock(codeblock)) = &statement.value else {
            panic!("expected codeblock literal");
        };
        assert_eq!(codeblock.params.len(), 2);
        assert_eq!(codeblock.body.len(), 2);
        assert!(matches!(
            codeblock.body[0],
            Expression::Binary(ref binary) if binary.operator == BinaryOperator::Add
        ));
        assert!(matches!(codeblock.body[1], Expression::Codeblock(_)));
    }

    #[test]
    fn parses_macro_read_expressions() {
        let source = r#"
FUNCTION Lookup(cName, cExpr)
   RETURN &cName + &( cExpr )
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);

        let Item::Routine(routine) = &parsed.program.items[0] else {
            panic!("expected routine item");
        };
        let Statement::Return(statement) = &routine.body[0] else {
            panic!("expected return statement");
        };
        let Some(Expression::Binary(binary)) = &statement.value else {
            panic!("expected binary expression");
        };
        assert!(matches!(binary.left.as_ref(), Expression::Macro(_)));
        assert!(matches!(binary.right.as_ref(), Expression::Macro(_)));
    }

    #[test]
    fn reports_missing_pipe_after_codeblock_parameters() {
        let source = r#"
FUNCTION Build()
   RETURN {|x, y x + y }
"#;
        let parsed = parse(source);

        assert_eq!(parsed.errors.len(), 2);
        assert_eq!(
            parsed.errors[0].to_string(),
            "expected `|` after codeblock parameters; found `x` at line 3, column 18"
        );
    }

    #[test]
    fn reports_missing_right_bracket_in_array_index() {
        let source = r#"
FUNCTION Pick()
   RETURN values[1 + 2
"#;
        let parsed = parse(source);

        assert_eq!(parsed.errors.len(), 1);
        assert_eq!(
            parsed.errors[0].to_string(),
            "expected `]` after array index; found `\\n` at line 3, column 23"
        );
    }

    #[test]
    fn reports_non_identifier_compound_assignment_target() {
        let source = r#"
PROCEDURE Main()
   ( total + 1 ) += 2
"#;
        let parsed = parse(source);

        assert_eq!(parsed.errors.len(), 1);
        assert_eq!(
            parsed.errors[0].to_string(),
            "expected identifier before compound assignment operator at line 3, column 21"
        );
    }

    #[test]
    fn reports_missing_identifier_in_static_initializer_list() {
        let source = r#"
PROCEDURE Main()
   STATIC cache := "memo",
   RETURN
"#;
        let parsed = parse(source);

        assert_eq!(parsed.errors.len(), 1);
        assert_eq!(
            parsed.errors[0].to_string(),
            "expected identifier at line 3, column 27"
        );
    }

    #[test]
    fn parses_if_condition_and_else_body() {
        let source = r#"
PROCEDURE Main()
   IF x == 1
      ? "one"
   ELSE
      ? "other"
   ENDIF
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let Item::Routine(routine) = &parsed.program.items[0] else {
            panic!("expected routine item");
        };
        match &routine.body[0] {
            Statement::If(statement) => {
                assert_eq!(statement.branches.len(), 1);
                assert!(statement.else_branch.is_some());
                let Expression::Binary(condition) = &statement.branches[0].condition else {
                    panic!("expected binary condition");
                };
                assert_eq!(condition.operator, BinaryOperator::ExactEqual);
            }
            statement => panic!("expected if statement, found {statement:?}"),
        }
    }

    #[test]
    fn parses_do_while_postfix_condition() {
        let source = r#"
PROCEDURE Main()
   DO WHILE x++ < 1000
      ? x
   ENDDO
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let Item::Routine(routine) = &parsed.program.items[0] else {
            panic!("expected routine item");
        };
        match &routine.body[0] {
            Statement::DoWhile(statement) => {
                assert!(matches!(statement.condition, Expression::Binary(_)));
            }
            statement => panic!("expected do while, found {statement:?}"),
        }
    }

    #[test]
    fn parses_for_header_with_step() {
        let source = r#"
PROCEDURE Main()
   FOR n := 10 TO 1 STEP -1
      ? n
   NEXT
"#;
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let Item::Routine(routine) = &parsed.program.items[0] else {
            panic!("expected routine item");
        };
        match &routine.body[0] {
            Statement::For(statement) => {
                assert_eq!(statement.variable.text, "n");
                assert!(statement.step.is_some());
            }
            statement => panic!("expected for, found {statement:?}"),
        }
    }

    #[test]
    fn recovers_from_missing_endif_before_next_routine() {
        let source = r#"
PROCEDURE Main()
   IF x == 1
      ? "one"

PROCEDURE NextProc()
   RETURN
"#;
        let parsed = parse(source);
        assert_eq!(parsed.program.items.len(), 2);
        assert_eq!(parsed.errors.len(), 1);
        assert_eq!(
            parsed.errors[0].to_string(),
            "expected ENDIF after IF block; found keyword Procedure at line 6, column 1"
        );
    }

    #[test]
    fn recovers_from_missing_enddo_before_next_routine() {
        let source = r#"
PROCEDURE Main()
   DO WHILE x < 3
      ? x

FUNCTION NextFunc()
   RETURN NIL
"#;
        let parsed = parse(source);
        assert_eq!(parsed.program.items.len(), 2);
        assert_eq!(parsed.errors.len(), 1);
        assert_eq!(
            parsed.errors[0].to_string(),
            "expected ENDDO after DO WHILE block; found keyword Function at line 6, column 1"
        );
    }

    #[test]
    fn recovers_from_missing_next_before_next_routine() {
        let source = r#"
PROCEDURE Main()
   FOR n := 1 TO 3
      ? n

PROCEDURE Tail()
   RETURN
"#;
        let parsed = parse(source);
        assert_eq!(parsed.program.items.len(), 2);
        assert_eq!(parsed.errors.len(), 1);
        assert_eq!(
            parsed.errors[0].to_string(),
            "expected NEXT after FOR block; found keyword Procedure at line 6, column 1"
        );
    }
}
