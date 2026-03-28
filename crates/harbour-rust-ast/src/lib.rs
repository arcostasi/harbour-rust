use harbour_rust_lexer::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub items: Vec<Item>,
}

impl Program {
    pub fn span(&self) -> Option<Span> {
        let first = self.items.first()?;
        let last = self.items.last()?;
        Some(Span {
            start: first.span().start,
            end: last.span().end,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Routine(Routine),
}

impl Item {
    pub fn span(&self) -> Span {
        match self {
            Self::Routine(routine) => routine.span,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutineKind {
    Procedure,
    Function,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Routine {
    pub kind: RoutineKind,
    pub name: Identifier,
    pub params: Vec<Identifier>,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Return(ReturnStatement),
    Print(PrintStatement),
    Expression(ExpressionStatement),
}

impl Statement {
    pub fn span(&self) -> Span {
        match self {
            Self::Return(statement) => statement.span,
            Self::Print(statement) => statement.span,
            Self::Expression(statement) => statement.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintStatement {
    pub arguments: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    Nil(NilLiteral),
    Logical(LogicalLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    String(StringLiteral),
    Call(CallExpression),
    Assignment(AssignmentExpression),
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Self::Identifier(expression) => expression.span,
            Self::Nil(expression) => expression.span,
            Self::Logical(expression) => expression.span,
            Self::Integer(expression) => expression.span,
            Self::Float(expression) => expression.span,
            Self::String(expression) => expression.span,
            Self::Call(expression) => expression.span,
            Self::Assignment(expression) => expression.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    pub text: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NilLiteral {
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalLiteral {
    pub value: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub lexeme: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloatLiteral {
    pub lexeme: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteral {
    pub lexeme: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentExpression {
    pub target: Box<Expression>,
    pub value: Box<Expression>,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use harbour_rust_lexer::{Position, Span};

    use crate::{
        CallExpression, Expression, ExpressionStatement, Identifier, Item, NilLiteral,
        PrintStatement, Program, ReturnStatement, Routine, RoutineKind, Statement, StringLiteral,
    };

    fn span(
        start_offset: usize,
        start_line: usize,
        start_column: usize,
        end_offset: usize,
        end_line: usize,
        end_column: usize,
    ) -> Span {
        Span {
            start: Position {
                offset: start_offset,
                line: start_line,
                column: start_column,
            },
            end: Position {
                offset: end_offset,
                line: end_line,
                column: end_column,
            },
        }
    }

    #[test]
    fn program_span_covers_first_and_last_item() {
        let routine_a = Routine {
            kind: RoutineKind::Procedure,
            name: Identifier {
                text: "Main".to_owned(),
                span: span(0, 1, 1, 4, 1, 5),
            },
            params: Vec::new(),
            body: Vec::new(),
            span: span(0, 1, 1, 10, 2, 1),
        };
        let routine_b = Routine {
            kind: RoutineKind::Function,
            name: Identifier {
                text: "Helper".to_owned(),
                span: span(11, 3, 1, 17, 3, 7),
            },
            params: Vec::new(),
            body: Vec::new(),
            span: span(11, 3, 1, 30, 4, 1),
        };
        let program = Program {
            items: vec![Item::Routine(routine_a), Item::Routine(routine_b)],
        };

        assert_eq!(program.span(), Some(span(0, 1, 1, 30, 4, 1)));
    }

    #[test]
    fn statements_report_their_own_spans() {
        let return_statement = Statement::Return(ReturnStatement {
            value: Some(Expression::Nil(NilLiteral {
                span: span(7, 2, 8, 10, 2, 11),
            })),
            span: span(0, 2, 1, 10, 2, 11),
        });
        let print_statement = Statement::Print(PrintStatement {
            arguments: vec![Expression::String(StringLiteral {
                lexeme: "\"hello\"".to_owned(),
                span: span(2, 4, 3, 9, 4, 10),
            })],
            span: span(0, 4, 1, 9, 4, 10),
        });

        assert_eq!(return_statement.span(), span(0, 2, 1, 10, 2, 11));
        assert_eq!(print_statement.span(), span(0, 4, 1, 9, 4, 10));
    }

    #[test]
    fn call_expression_uses_outer_span() {
        let call = Expression::Call(CallExpression {
            callee: Box::new(Expression::Identifier(Identifier {
                text: "QOut".to_owned(),
                span: span(0, 1, 1, 4, 1, 5),
            })),
            arguments: vec![Expression::String(StringLiteral {
                lexeme: "\"Hello\"".to_owned(),
                span: span(5, 1, 6, 12, 1, 13),
            })],
            span: span(0, 1, 1, 13, 1, 14),
        });
        let statement = Statement::Expression(ExpressionStatement {
            expression: call.clone(),
            span: call.span(),
        });

        assert_eq!(call.span(), span(0, 1, 1, 13, 1, 14));
        assert_eq!(statement.span(), span(0, 1, 1, 13, 1, 14));
    }
}
