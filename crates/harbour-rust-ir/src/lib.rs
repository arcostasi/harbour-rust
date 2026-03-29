use std::fmt;

use harbour_rust_hir as hir;
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

pub fn lower_program(program: &hir::Program) -> LoweringOutput {
    let mut errors = Vec::new();
    let routines = program
        .routines
        .iter()
        .map(|routine| lower_routine(routine, &mut errors))
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
    Assign(AssignStatement),
    If(Box<IfStatement>),
    DoWhile(Box<DoWhileStatement>),
    For(Box<ForStatement>),
    Return(ReturnStatement),
    BuiltinCall(BuiltinCallStatement),
    Evaluate(ExpressionStatement),
}

impl Statement {
    pub fn span(&self) -> Span {
        match self {
            Self::Local(statement) => statement.span,
            Self::Assign(statement) => statement.span,
            Self::If(statement) => statement.span,
            Self::DoWhile(statement) => statement.span,
            Self::For(statement) => statement.span,
            Self::Return(statement) => statement.span,
            Self::BuiltinCall(statement) => statement.span,
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
pub struct AssignStatement {
    pub target: Symbol,
    pub value: Expression,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Builtin {
    QOut,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltinCallStatement {
    pub builtin: Builtin,
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
    Index(IndexExpression),
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
            Self::Index(expression) => expression.span,
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
pub struct IndexExpression {
    pub target: Box<Expression>,
    pub indices: Vec<Expression>,
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

fn lower_routine(routine: &hir::Routine, errors: &mut Vec<LoweringError>) -> Routine {
    Routine {
        kind: lower_routine_kind(routine.kind),
        name: lower_symbol(&routine.name),
        params: routine.params.iter().map(lower_symbol).collect(),
        body: routine
            .body
            .iter()
            .map(|statement| lower_statement(statement, errors))
            .collect(),
        span: routine.span,
    }
}

fn lower_routine_kind(kind: hir::RoutineKind) -> RoutineKind {
    match kind {
        hir::RoutineKind::Procedure => RoutineKind::Procedure,
        hir::RoutineKind::Function => RoutineKind::Function,
    }
}

fn lower_statement(statement: &hir::Statement, errors: &mut Vec<LoweringError>) -> Statement {
    match statement {
        hir::Statement::Local(statement) => Statement::Local(LocalStatement {
            bindings: statement
                .bindings
                .iter()
                .map(|binding| LocalBinding {
                    name: lower_symbol(&binding.name),
                    initializer: binding
                        .initializer
                        .as_ref()
                        .map(|expression| lower_expression(expression, errors)),
                    span: binding.span,
                })
                .collect(),
            span: statement.span,
        }),
        hir::Statement::If(statement) => Statement::If(Box::new(IfStatement {
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
        hir::Statement::DoWhile(statement) => Statement::DoWhile(Box::new(DoWhileStatement {
            condition: lower_expression(&statement.condition, errors),
            body: statement
                .body
                .iter()
                .map(|statement| lower_statement(statement, errors))
                .collect(),
            span: statement.span,
        })),
        hir::Statement::For(statement) => Statement::For(Box::new(ForStatement {
            variable: lower_symbol(&statement.variable),
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
        hir::Statement::Return(statement) => Statement::Return(ReturnStatement {
            value: statement
                .value
                .as_ref()
                .map(|expression| lower_expression(expression, errors)),
            span: statement.span,
        }),
        hir::Statement::Print(statement) => Statement::BuiltinCall(BuiltinCallStatement {
            builtin: Builtin::QOut,
            arguments: statement
                .arguments
                .iter()
                .map(|expression| lower_expression(expression, errors))
                .collect(),
            span: statement.span,
        }),
        hir::Statement::Evaluate(statement) => lower_expression_statement(statement, errors),
    }
}

fn lower_expression_statement(
    statement: &hir::ExpressionStatement,
    errors: &mut Vec<LoweringError>,
) -> Statement {
    match &statement.expression {
        hir::Expression::Assign(expression) => Statement::Assign(AssignStatement {
            target: lower_symbol(&expression.target),
            value: lower_expression(&expression.value, errors),
            span: statement.span,
        }),
        expression => Statement::Evaluate(ExpressionStatement {
            expression: lower_expression(expression, errors),
            span: statement.span,
        }),
    }
}

fn lower_expression(expression: &hir::Expression, errors: &mut Vec<LoweringError>) -> Expression {
    match expression {
        hir::Expression::Symbol(symbol) => Expression::Symbol(lower_symbol(symbol)),
        hir::Expression::Nil(literal) => Expression::Nil(NilLiteral { span: literal.span }),
        hir::Expression::Logical(literal) => Expression::Logical(LogicalLiteral {
            value: literal.value,
            span: literal.span,
        }),
        hir::Expression::Integer(literal) => Expression::Integer(IntegerLiteral {
            lexeme: literal.lexeme.clone(),
            span: literal.span,
        }),
        hir::Expression::Float(literal) => Expression::Float(FloatLiteral {
            lexeme: literal.lexeme.clone(),
            span: literal.span,
        }),
        hir::Expression::String(literal) => Expression::String(StringLiteral {
            lexeme: literal.lexeme.clone(),
            span: literal.span,
        }),
        hir::Expression::Array(expression) => {
            errors.push(LoweringError {
                message: "array literals are not supported in IR yet".to_owned(),
                span: expression.span,
            });
            Expression::Error(ErrorExpression {
                span: expression.span,
            })
        }
        hir::Expression::Call(expression) => Expression::Call(CallExpression {
            callee: Box::new(lower_expression(&expression.callee, errors)),
            arguments: expression
                .arguments
                .iter()
                .map(|argument| lower_expression(argument, errors))
                .collect(),
            span: expression.span,
        }),
        hir::Expression::Index(expression) => Expression::Index(IndexExpression {
            target: Box::new(lower_expression(&expression.target, errors)),
            indices: expression
                .indices
                .iter()
                .map(|index| lower_expression(index, errors))
                .collect(),
            span: expression.span,
        }),
        hir::Expression::Assign(expression) => {
            errors.push(LoweringError {
                message: "cannot lower assignment expression outside statement position".to_owned(),
                span: expression.span,
            });
            Expression::Error(ErrorExpression {
                span: expression.span,
            })
        }
        hir::Expression::Binary(expression) => Expression::Binary(BinaryExpression {
            left: Box::new(lower_expression(&expression.left, errors)),
            operator: lower_binary_operator(expression.operator),
            right: Box::new(lower_expression(&expression.right, errors)),
            span: expression.span,
        }),
        hir::Expression::Unary(expression) => Expression::Unary(UnaryExpression {
            operator: lower_unary_operator(expression.operator),
            operand: Box::new(lower_expression(&expression.operand, errors)),
            span: expression.span,
        }),
        hir::Expression::Postfix(expression) => Expression::Postfix(PostfixExpression {
            operand: Box::new(lower_expression(&expression.operand, errors)),
            operator: lower_postfix_operator(expression.operator),
            span: expression.span,
        }),
        hir::Expression::Error(expression) => {
            errors.push(LoweringError {
                message: "cannot lower invalid HIR expression".to_owned(),
                span: expression.span,
            });
            Expression::Error(ErrorExpression {
                span: expression.span,
            })
        }
    }
}

fn lower_symbol(symbol: &hir::Symbol) -> Symbol {
    Symbol {
        text: symbol.text.clone(),
        span: symbol.span,
    }
}

fn lower_binary_operator(operator: hir::BinaryOperator) -> BinaryOperator {
    match operator {
        hir::BinaryOperator::Or => BinaryOperator::Or,
        hir::BinaryOperator::And => BinaryOperator::And,
        hir::BinaryOperator::Equal => BinaryOperator::Equal,
        hir::BinaryOperator::ExactEqual => BinaryOperator::ExactEqual,
        hir::BinaryOperator::NotEqual => BinaryOperator::NotEqual,
        hir::BinaryOperator::Less => BinaryOperator::Less,
        hir::BinaryOperator::LessEqual => BinaryOperator::LessEqual,
        hir::BinaryOperator::Greater => BinaryOperator::Greater,
        hir::BinaryOperator::GreaterEqual => BinaryOperator::GreaterEqual,
        hir::BinaryOperator::Add => BinaryOperator::Add,
        hir::BinaryOperator::Subtract => BinaryOperator::Subtract,
        hir::BinaryOperator::Multiply => BinaryOperator::Multiply,
        hir::BinaryOperator::Divide => BinaryOperator::Divide,
        hir::BinaryOperator::Modulo => BinaryOperator::Modulo,
        hir::BinaryOperator::Power => BinaryOperator::Power,
    }
}

fn lower_unary_operator(operator: hir::UnaryOperator) -> UnaryOperator {
    match operator {
        hir::UnaryOperator::Plus => UnaryOperator::Plus,
        hir::UnaryOperator::Minus => UnaryOperator::Minus,
        hir::UnaryOperator::Not => UnaryOperator::Not,
    }
}

fn lower_postfix_operator(operator: hir::PostfixOperator) -> PostfixOperator {
    match operator {
        hir::PostfixOperator::Increment => PostfixOperator::Increment,
        hir::PostfixOperator::Decrement => PostfixOperator::Decrement,
    }
}

#[cfg(test)]
mod tests {
    use harbour_rust_hir as hir;
    use harbour_rust_lexer::{Position, Span};

    use crate::{
        AssignStatement, Builtin, BuiltinCallStatement, ErrorExpression, Expression, LoweringError,
        LoweringOutput, ReturnStatement, Routine, RoutineKind, Statement, Symbol, lower_program,
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

    fn symbol(text: &str, span: Span) -> hir::Symbol {
        hir::Symbol {
            text: text.to_owned(),
            span,
        }
    }

    #[test]
    fn lowers_print_and_assignment_statements_to_ir_surface() {
        let routine_span = span(0, 1, 1, 40, 4, 10);
        let print_span = span(10, 2, 4, 22, 2, 16);
        let assign_span = span(24, 3, 4, 31, 3, 11);
        let return_span = span(33, 4, 4, 40, 4, 10);
        let program = hir::Program {
            routines: vec![hir::Routine {
                kind: hir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![
                    hir::Statement::Print(hir::PrintStatement {
                        arguments: vec![hir::Expression::String(hir::StringLiteral {
                            lexeme: "hello".to_owned(),
                            span: span(12, 2, 6, 19, 2, 13),
                        })],
                        span: print_span,
                    }),
                    hir::Statement::Evaluate(hir::ExpressionStatement {
                        expression: hir::Expression::Assign(hir::AssignExpression {
                            target: symbol("x", span(24, 3, 4, 25, 3, 5)),
                            value: Box::new(hir::Expression::Integer(hir::IntegerLiteral {
                                lexeme: "1".to_owned(),
                                span: span(30, 3, 10, 31, 3, 11),
                            })),
                            span: assign_span,
                        }),
                        span: assign_span,
                    }),
                    hir::Statement::Return(hir::ReturnStatement {
                        value: Some(hir::Expression::Symbol(symbol(
                            "x",
                            span(40, 4, 11, 41, 4, 12),
                        ))),
                        span: return_span,
                    }),
                ],
                span: routine_span,
            }],
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
                params: Vec::new(),
                body: vec![
                    Statement::BuiltinCall(BuiltinCallStatement {
                        builtin: Builtin::QOut,
                        arguments: vec![Expression::String(crate::StringLiteral {
                            lexeme: "hello".to_owned(),
                            span: span(12, 2, 6, 19, 2, 13),
                        })],
                        span: print_span,
                    }),
                    Statement::Assign(AssignStatement {
                        target: Symbol {
                            text: "x".to_owned(),
                            span: span(24, 3, 4, 25, 3, 5),
                        },
                        value: Expression::Integer(crate::IntegerLiteral {
                            lexeme: "1".to_owned(),
                            span: span(30, 3, 10, 31, 3, 11),
                        }),
                        span: assign_span,
                    }),
                    Statement::Return(ReturnStatement {
                        value: Some(Expression::Symbol(Symbol {
                            text: "x".to_owned(),
                            span: span(40, 4, 11, 41, 4, 12),
                        })),
                        span: return_span,
                    }),
                ],
                span: routine_span,
            }]
        );
    }

    #[test]
    fn reports_invalid_hir_expression_during_lowering() {
        let expression_span = span(15, 2, 6, 16, 2, 7);
        let program = hir::Program {
            routines: vec![hir::Routine {
                kind: hir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![hir::Statement::Evaluate(hir::ExpressionStatement {
                    expression: hir::Expression::Error(hir::ErrorExpression {
                        span: expression_span,
                    }),
                    span: expression_span,
                })],
                span: span(0, 1, 1, 16, 2, 7),
            }],
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
                            span: span(0, 1, 1, 4, 1, 5),
                        },
                        params: Vec::new(),
                        body: vec![Statement::Evaluate(crate::ExpressionStatement {
                            expression: Expression::Error(ErrorExpression {
                                span: expression_span,
                            }),
                            span: expression_span,
                        })],
                        span: span(0, 1, 1, 16, 2, 7),
                    }],
                },
                errors: vec![LoweringError {
                    message: "cannot lower invalid HIR expression".to_owned(),
                    span: expression_span,
                }],
            }
        );
    }

    #[test]
    fn reports_array_literals_as_unsupported_in_ir_lowering() {
        let expression_span = span(15, 2, 6, 24, 2, 15);
        let program = hir::Program {
            routines: vec![hir::Routine {
                kind: hir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![hir::Statement::Evaluate(hir::ExpressionStatement {
                    expression: hir::Expression::Array(hir::ArrayLiteral {
                        elements: vec![hir::Expression::Integer(hir::IntegerLiteral {
                            lexeme: "1".to_owned(),
                            span: span(17, 2, 8, 18, 2, 9),
                        })],
                        span: expression_span,
                    }),
                    span: expression_span,
                })],
                span: span(0, 1, 1, 24, 2, 15),
            }],
        };

        let lowered = lower_program(&program);

        assert_eq!(
            lowered.errors,
            vec![LoweringError {
                message: "array literals are not supported in IR yet".to_owned(),
                span: expression_span,
            }]
        );
        assert!(matches!(
            lowered.program.routines[0].body[0],
            Statement::Evaluate(crate::ExpressionStatement {
                expression: Expression::Error(ErrorExpression { span }),
                span: _
            }) if span == expression_span
        ));
    }

    #[test]
    fn lowers_array_indexing_to_explicit_ir_nodes() {
        let expression_span = span(15, 2, 6, 25, 2, 16);
        let program = hir::Program {
            routines: vec![hir::Routine {
                kind: hir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![hir::Statement::Evaluate(hir::ExpressionStatement {
                    expression: hir::Expression::Index(hir::IndexExpression {
                        target: Box::new(hir::Expression::Symbol(symbol(
                            "matrix",
                            span(15, 2, 6, 21, 2, 12),
                        ))),
                        indices: vec![hir::Expression::Integer(hir::IntegerLiteral {
                            lexeme: "1".to_owned(),
                            span: span(22, 2, 13, 23, 2, 14),
                        })],
                        span: expression_span,
                    }),
                    span: expression_span,
                })],
                span: span(0, 1, 1, 25, 2, 16),
            }],
        };

        let lowered = lower_program(&program);

        assert_eq!(lowered.errors, Vec::new());
        let Statement::Evaluate(crate::ExpressionStatement { expression, .. }) =
            &lowered.program.routines[0].body[0]
        else {
            panic!("expected evaluation statement");
        };
        let Expression::Index(index) = expression else {
            panic!("expected lowered index expression");
        };
        assert!(matches!(index.target.as_ref(), Expression::Symbol(_)));
        assert_eq!(index.indices.len(), 1);
        assert!(matches!(index.indices[0], Expression::Integer(_)));
        assert_eq!(index.span, expression_span);
    }
}
