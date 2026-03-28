use std::fmt;

use harbour_rust_ast as ast;
use harbour_rust_lexer::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweringError {
    pub message: String,
    pub span: Span,
}

impl LoweringError {
    pub fn line(&self) -> usize {
        self.span.start.line
    }

    pub fn column(&self) -> usize {
        self.span.start.column
    }
}

impl fmt::Display for LoweringError {
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
pub struct LoweringOutput {
    pub program: Program,
    pub errors: Vec<LoweringError>,
}

pub fn lower_program(program: &ast::Program) -> LoweringOutput {
    let mut errors = Vec::new();
    let routines = program
        .items
        .iter()
        .map(|item| lower_item(item, &mut errors))
        .collect();

    LoweringOutput {
        program: Program { routines },
        errors,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub routines: Vec<Routine>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutineKind {
    Procedure,
    Function,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Routine {
    pub kind: RoutineKind,
    pub name: Symbol,
    pub params: Vec<Symbol>,
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
    Evaluate(ExpressionStatement),
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
            Self::Evaluate(statement) => statement.span,
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
    pub name: Symbol,
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
    pub variable: Symbol,
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
    Symbol(Symbol),
    Nil(NilLiteral),
    Logical(LogicalLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    String(StringLiteral),
    Call(CallExpression),
    Assign(AssignExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Postfix(PostfixExpression),
    Error(ErrorExpression),
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Self::Symbol(symbol) => symbol.span,
            Self::Nil(literal) => literal.span,
            Self::Logical(literal) => literal.span,
            Self::Integer(literal) => literal.span,
            Self::Float(literal) => literal.span,
            Self::String(literal) => literal.span,
            Self::Call(expression) => expression.span,
            Self::Assign(expression) => expression.span,
            Self::Binary(expression) => expression.span,
            Self::Unary(expression) => expression.span,
            Self::Postfix(expression) => expression.span,
            Self::Error(expression) => expression.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
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
pub struct AssignExpression {
    pub target: Symbol,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorExpression {
    pub span: Span,
}

fn lower_item(item: &ast::Item, errors: &mut Vec<LoweringError>) -> Routine {
    match item {
        ast::Item::Routine(routine) => lower_routine(routine, errors),
    }
}

fn lower_routine(routine: &ast::Routine, errors: &mut Vec<LoweringError>) -> Routine {
    Routine {
        kind: lower_routine_kind(routine.kind),
        name: lower_identifier(&routine.name),
        params: routine.params.iter().map(lower_identifier).collect(),
        body: routine
            .body
            .iter()
            .map(|statement| lower_statement(statement, errors))
            .collect(),
        span: routine.span,
    }
}

fn lower_routine_kind(kind: ast::RoutineKind) -> RoutineKind {
    match kind {
        ast::RoutineKind::Procedure => RoutineKind::Procedure,
        ast::RoutineKind::Function => RoutineKind::Function,
    }
}

fn lower_statement(statement: &ast::Statement, errors: &mut Vec<LoweringError>) -> Statement {
    match statement {
        ast::Statement::Local(statement) => Statement::Local(LocalStatement {
            bindings: statement
                .bindings
                .iter()
                .map(|binding| LocalBinding {
                    name: lower_identifier(&binding.name),
                    initializer: binding
                        .initializer
                        .as_ref()
                        .map(|expression| lower_expression(expression, errors)),
                    span: binding.span,
                })
                .collect(),
            span: statement.span,
        }),
        ast::Statement::If(statement) => Statement::If(Box::new(IfStatement {
            branches: statement
                .branches
                .iter()
                .map(|branch| ConditionalBranch {
                    condition: lower_expression(&branch.condition, errors),
                    body: branch
                        .body
                        .iter()
                        .map(|statement| lower_statement(statement, errors))
                        .collect(),
                    span: branch.span,
                })
                .collect(),
            else_branch: statement.else_branch.as_ref().map(|branch| {
                branch
                    .iter()
                    .map(|statement| lower_statement(statement, errors))
                    .collect()
            }),
            span: statement.span,
        })),
        ast::Statement::DoWhile(statement) => Statement::DoWhile(Box::new(DoWhileStatement {
            condition: lower_expression(&statement.condition, errors),
            body: statement
                .body
                .iter()
                .map(|statement| lower_statement(statement, errors))
                .collect(),
            span: statement.span,
        })),
        ast::Statement::For(statement) => Statement::For(Box::new(ForStatement {
            variable: lower_identifier(&statement.variable),
            initial_value: lower_expression(&statement.initial_value, errors),
            limit: lower_expression(&statement.limit, errors),
            step: statement
                .step
                .as_ref()
                .map(|expression| lower_expression(expression, errors)),
            body: statement
                .body
                .iter()
                .map(|statement| lower_statement(statement, errors))
                .collect(),
            span: statement.span,
        })),
        ast::Statement::Return(statement) => Statement::Return(ReturnStatement {
            value: statement
                .value
                .as_ref()
                .map(|expression| lower_expression(expression, errors)),
            span: statement.span,
        }),
        ast::Statement::Print(statement) => Statement::Print(PrintStatement {
            arguments: statement
                .arguments
                .iter()
                .map(|expression| lower_expression(expression, errors))
                .collect(),
            span: statement.span,
        }),
        ast::Statement::Expression(statement) => Statement::Evaluate(ExpressionStatement {
            expression: lower_expression(&statement.expression, errors),
            span: statement.span,
        }),
    }
}

fn lower_expression(expression: &ast::Expression, errors: &mut Vec<LoweringError>) -> Expression {
    match expression {
        ast::Expression::Identifier(identifier) => Expression::Symbol(lower_identifier(identifier)),
        ast::Expression::Nil(literal) => Expression::Nil(NilLiteral { span: literal.span }),
        ast::Expression::Logical(literal) => Expression::Logical(LogicalLiteral {
            value: literal.value,
            span: literal.span,
        }),
        ast::Expression::Integer(literal) => Expression::Integer(IntegerLiteral {
            lexeme: literal.lexeme.clone(),
            span: literal.span,
        }),
        ast::Expression::Float(literal) => Expression::Float(FloatLiteral {
            lexeme: literal.lexeme.clone(),
            span: literal.span,
        }),
        ast::Expression::String(literal) => Expression::String(StringLiteral {
            lexeme: literal.lexeme.clone(),
            span: literal.span,
        }),
        ast::Expression::Call(expression) => Expression::Call(CallExpression {
            callee: Box::new(lower_expression(&expression.callee, errors)),
            arguments: expression
                .arguments
                .iter()
                .map(|expression| lower_expression(expression, errors))
                .collect(),
            span: expression.span,
        }),
        ast::Expression::Assignment(expression) => {
            if let ast::Expression::Identifier(identifier) = expression.target.as_ref() {
                Expression::Assign(AssignExpression {
                    target: lower_identifier(identifier),
                    value: Box::new(lower_expression(&expression.value, errors)),
                    span: expression.span,
                })
            } else {
                errors.push(LoweringError {
                    message: "expected identifier on assignment left-hand side".to_owned(),
                    span: expression.target.span(),
                });
                Expression::Error(ErrorExpression {
                    span: expression.span,
                })
            }
        }
        ast::Expression::Binary(expression) => Expression::Binary(BinaryExpression {
            left: Box::new(lower_expression(&expression.left, errors)),
            operator: lower_binary_operator(expression.operator),
            right: Box::new(lower_expression(&expression.right, errors)),
            span: expression.span,
        }),
        ast::Expression::Unary(expression) => Expression::Unary(UnaryExpression {
            operator: lower_unary_operator(expression.operator),
            operand: Box::new(lower_expression(&expression.operand, errors)),
            span: expression.span,
        }),
        ast::Expression::Postfix(expression) => Expression::Postfix(PostfixExpression {
            operand: Box::new(lower_expression(&expression.operand, errors)),
            operator: lower_postfix_operator(expression.operator),
            span: expression.span,
        }),
    }
}

fn lower_identifier(identifier: &ast::Identifier) -> Symbol {
    Symbol {
        text: identifier.text.clone(),
        span: identifier.span,
    }
}

fn lower_binary_operator(operator: ast::BinaryOperator) -> BinaryOperator {
    match operator {
        ast::BinaryOperator::Or => BinaryOperator::Or,
        ast::BinaryOperator::And => BinaryOperator::And,
        ast::BinaryOperator::Equal => BinaryOperator::Equal,
        ast::BinaryOperator::ExactEqual => BinaryOperator::ExactEqual,
        ast::BinaryOperator::NotEqual => BinaryOperator::NotEqual,
        ast::BinaryOperator::Less => BinaryOperator::Less,
        ast::BinaryOperator::LessEqual => BinaryOperator::LessEqual,
        ast::BinaryOperator::Greater => BinaryOperator::Greater,
        ast::BinaryOperator::GreaterEqual => BinaryOperator::GreaterEqual,
        ast::BinaryOperator::Add => BinaryOperator::Add,
        ast::BinaryOperator::Subtract => BinaryOperator::Subtract,
        ast::BinaryOperator::Multiply => BinaryOperator::Multiply,
        ast::BinaryOperator::Divide => BinaryOperator::Divide,
        ast::BinaryOperator::Modulo => BinaryOperator::Modulo,
        ast::BinaryOperator::Power => BinaryOperator::Power,
    }
}

fn lower_unary_operator(operator: ast::UnaryOperator) -> UnaryOperator {
    match operator {
        ast::UnaryOperator::Plus => UnaryOperator::Plus,
        ast::UnaryOperator::Minus => UnaryOperator::Minus,
        ast::UnaryOperator::Not => UnaryOperator::Not,
    }
}

fn lower_postfix_operator(operator: ast::PostfixOperator) -> PostfixOperator {
    match operator {
        ast::PostfixOperator::Increment => PostfixOperator::Increment,
        ast::PostfixOperator::Decrement => PostfixOperator::Decrement,
    }
}

#[cfg(test)]
mod tests {
    use harbour_rust_ast as ast;
    use harbour_rust_lexer::{Position, Span};

    use crate::{
        Expression, ExpressionStatement, LocalBinding, LocalStatement, LoweringOutput,
        ReturnStatement, Routine, RoutineKind, Statement, Symbol, lower_program,
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

    fn identifier(text: &str, span: Span) -> ast::Identifier {
        ast::Identifier {
            text: text.to_owned(),
            span,
        }
    }

    #[test]
    fn lowers_manual_routine_to_hir() {
        let program = ast::Program {
            items: vec![ast::Item::Routine(ast::Routine {
                kind: ast::RoutineKind::Procedure,
                name: identifier("Main", span(0, 1, 1, 4, 1, 5)),
                params: vec![identifier("name", span(15, 1, 16, 19, 1, 20))],
                body: vec![
                    ast::Statement::Local(ast::LocalStatement {
                        bindings: vec![ast::LocalBinding {
                            name: identifier("x", span(25, 2, 7, 26, 2, 8)),
                            initializer: Some(ast::Expression::Integer(ast::IntegerLiteral {
                                lexeme: "1".to_owned(),
                                span: span(30, 2, 12, 31, 2, 13),
                            })),
                            span: span(25, 2, 7, 31, 2, 13),
                        }],
                        span: span(19, 2, 1, 31, 2, 13),
                    }),
                    ast::Statement::Return(ast::ReturnStatement {
                        value: Some(ast::Expression::Identifier(identifier(
                            "x",
                            span(39, 3, 8, 40, 3, 9),
                        ))),
                        span: span(32, 3, 1, 40, 3, 9),
                    }),
                ],
                span: span(0, 1, 1, 40, 3, 9),
            })],
        };

        let lowered = lower_program(&program);

        assert_eq!(lowered.errors, Vec::new());
        assert_eq!(
            lowered.program.routines,
            vec![Routine {
                kind: RoutineKind::Procedure,
                name: Symbol {
                    text: "Main".to_owned(),
                    span: span(0, 1, 1, 4, 1, 5),
                },
                params: vec![Symbol {
                    text: "name".to_owned(),
                    span: span(15, 1, 16, 19, 1, 20),
                }],
                body: vec![
                    Statement::Local(LocalStatement {
                        bindings: vec![LocalBinding {
                            name: Symbol {
                                text: "x".to_owned(),
                                span: span(25, 2, 7, 26, 2, 8),
                            },
                            initializer: Some(Expression::Integer(crate::IntegerLiteral {
                                lexeme: "1".to_owned(),
                                span: span(30, 2, 12, 31, 2, 13),
                            })),
                            span: span(25, 2, 7, 31, 2, 13),
                        }],
                        span: span(19, 2, 1, 31, 2, 13),
                    }),
                    Statement::Return(ReturnStatement {
                        value: Some(Expression::Symbol(Symbol {
                            text: "x".to_owned(),
                            span: span(39, 3, 8, 40, 3, 9),
                        })),
                        span: span(32, 3, 1, 40, 3, 9),
                    }),
                ],
                span: span(0, 1, 1, 40, 3, 9),
            }]
        );
    }

    #[test]
    fn reports_invalid_assignment_target() {
        let call_span = span(0, 1, 1, 10, 1, 11);
        let call = ast::Expression::Call(ast::CallExpression {
            callee: Box::new(ast::Expression::Identifier(identifier(
                "MakeTarget",
                span(0, 1, 1, 10, 1, 11),
            ))),
            arguments: Vec::new(),
            span: call_span,
        });
        let program = ast::Program {
            items: vec![ast::Item::Routine(ast::Routine {
                kind: ast::RoutineKind::Procedure,
                name: identifier("Main", span(12, 2, 1, 16, 2, 5)),
                params: Vec::new(),
                body: vec![ast::Statement::Expression(ast::ExpressionStatement {
                    expression: ast::Expression::Assignment(ast::AssignmentExpression {
                        target: Box::new(call),
                        value: Box::new(ast::Expression::Integer(ast::IntegerLiteral {
                            lexeme: "1".to_owned(),
                            span: span(20, 2, 9, 21, 2, 10),
                        })),
                        span: span(0, 1, 1, 21, 2, 10),
                    }),
                    span: span(0, 1, 1, 21, 2, 10),
                })],
                span: span(12, 2, 1, 21, 2, 10),
            })],
        };

        let lowered = lower_program(&program);

        assert_eq!(
            lowered,
            LoweringOutput {
                program: crate::Program {
                    routines: vec![Routine {
                        kind: RoutineKind::Procedure,
                        name: Symbol {
                            text: "Main".to_owned(),
                            span: span(12, 2, 1, 16, 2, 5),
                        },
                        params: Vec::new(),
                        body: vec![Statement::Evaluate(ExpressionStatement {
                            expression: Expression::Error(crate::ErrorExpression {
                                span: span(0, 1, 1, 21, 2, 10),
                            }),
                            span: span(0, 1, 1, 21, 2, 10),
                        })],
                        span: span(12, 2, 1, 21, 2, 10),
                    }],
                },
                errors: vec![crate::LoweringError {
                    message: "expected identifier on assignment left-hand side".to_owned(),
                    span: call_span,
                }],
            }
        );
    }
}
