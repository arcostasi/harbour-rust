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
    Local(LocalStatement),
    If(Box<IfStatement>),
    DoWhile(Box<DoWhileStatement>),
    For(Box<ForStatement>),
    Return(ReturnStatement),
    Print(PrintStatement),
    Expression(ExpressionStatement),
}

impl Statement {
    pub fn span(&self) -> Span {
        match self {
            Self::Local(statement) => statement.span,
            Self::If(statement) => statement.span,
            Self::DoWhile(statement) => statement.span,
            Self::For(statement) => statement.span,
            Self::Return(statement) => statement.span,
            Self::Print(statement) => statement.span,
            Self::Expression(statement) => statement.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStatement {
    pub bindings: Vec<LocalBinding>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalBinding {
    pub name: Identifier,
    pub initializer: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfStatement {
    pub branches: Vec<ConditionalBranch>,
    pub else_branch: Option<Vec<Statement>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalBranch {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoWhileStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForStatement {
    pub variable: Identifier,
    pub initial_value: Expression,
    pub limit: Expression,
    pub step: Option<Expression>,
    pub body: Vec<Statement>,
    pub span: Span,
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
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Postfix(PostfixExpression),
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
            Self::Binary(expression) => expression.span,
            Self::Unary(expression) => expression.span,
            Self::Postfix(expression) => expression.span,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Or,
    And,
    Equal,
    ExactEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostfixOperator {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostfixExpression {
    pub operand: Box<Expression>,
    pub operator: PostfixOperator,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use harbour_rust_lexer::{Position, Span};

    use crate::{
        BinaryExpression, BinaryOperator, CallExpression, ConditionalBranch, DoWhileStatement,
        Expression, ExpressionStatement, ForStatement, Identifier, IfStatement, Item, LocalBinding,
        LocalStatement, NilLiteral, PostfixExpression, PostfixOperator, PrintStatement, Program,
        ReturnStatement, Routine, RoutineKind, Statement, StringLiteral,
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

    #[test]
    fn control_flow_statements_report_their_spans() {
        let local = Statement::Local(LocalStatement {
            bindings: vec![LocalBinding {
                name: Identifier {
                    text: "x".to_owned(),
                    span: span(6, 2, 7, 7, 2, 8),
                },
                initializer: Some(Expression::Nil(NilLiteral {
                    span: span(11, 2, 12, 14, 2, 15),
                })),
                span: span(6, 2, 7, 14, 2, 15),
            }],
            span: span(0, 2, 1, 14, 2, 15),
        });
        let if_statement = Statement::If(Box::new(IfStatement {
            branches: vec![ConditionalBranch {
                condition: Expression::Logical(super::LogicalLiteral {
                    value: true,
                    span: span(3, 4, 4, 6, 4, 7),
                }),
                body: Vec::new(),
                span: span(0, 4, 1, 10, 5, 1),
            }],
            else_branch: None,
            span: span(0, 4, 1, 15, 6, 1),
        }));
        let while_statement = Statement::DoWhile(Box::new(DoWhileStatement {
            condition: Expression::Identifier(Identifier {
                text: "done".to_owned(),
                span: span(9, 8, 10, 13, 8, 14),
            }),
            body: Vec::new(),
            span: span(0, 8, 1, 18, 9, 1),
        }));
        let for_statement = Statement::For(Box::new(ForStatement {
            variable: Identifier {
                text: "i".to_owned(),
                span: span(4, 10, 5, 5, 10, 6),
            },
            initial_value: Expression::Integer(super::IntegerLiteral {
                lexeme: "1".to_owned(),
                span: span(9, 10, 10, 10, 10, 11),
            }),
            limit: Expression::Integer(super::IntegerLiteral {
                lexeme: "10".to_owned(),
                span: span(15, 10, 16, 17, 10, 18),
            }),
            step: None,
            body: Vec::new(),
            span: span(0, 10, 1, 22, 11, 1),
        }));

        assert_eq!(local.span(), span(0, 2, 1, 14, 2, 15));
        assert_eq!(if_statement.span(), span(0, 4, 1, 15, 6, 1));
        assert_eq!(while_statement.span(), span(0, 8, 1, 18, 9, 1));
        assert_eq!(for_statement.span(), span(0, 10, 1, 22, 11, 1));
    }

    #[test]
    fn binary_and_postfix_expressions_use_outer_span() {
        let expression = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Postfix(PostfixExpression {
                operand: Box::new(Expression::Identifier(Identifier {
                    text: "x".to_owned(),
                    span: span(0, 1, 1, 1, 1, 2),
                })),
                operator: PostfixOperator::Increment,
                span: span(0, 1, 1, 3, 1, 4),
            })),
            operator: BinaryOperator::Less,
            right: Box::new(Expression::Integer(super::IntegerLiteral {
                lexeme: "10".to_owned(),
                span: span(6, 1, 7, 8, 1, 9),
            })),
            span: span(0, 1, 1, 8, 1, 9),
        });

        assert_eq!(expression.span(), span(0, 1, 1, 8, 1, 9));
    }
}
