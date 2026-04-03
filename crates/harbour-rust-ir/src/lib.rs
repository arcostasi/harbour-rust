use std::{collections::HashSet, fmt};

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
    let module_static_names = collect_module_static_names(program);
    let dynamic_memvar_mode = program_uses_dynamic_features(program);
    let module_statics = program
        .module_statics
        .iter()
        .map(|statement| lower_static_statement(statement, &module_static_names, &mut errors))
        .collect();
    let routines = program
        .routines
        .iter()
        .map(|routine| {
            let mut lowerer = RoutineLowerer::new(
                &module_static_names,
                routine,
                dynamic_memvar_mode,
                &mut errors,
            );
            lowerer.lower()
        })
        .collect();

    LoweringOutput {
        program: Program {
            module_statics,
            routines,
        },
        errors,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub module_statics: Vec<StaticStatement>,
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
    Static(StaticStatement),
    Private(MemvarStatement),
    Public(MemvarStatement),
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
            Self::Static(statement) => statement.span,
            Self::Private(statement) => statement.span,
            Self::Public(statement) => statement.span,
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
pub struct StaticStatement {
    pub bindings: Vec<StaticBinding>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticBinding {
    pub name: Symbol,
    pub initializer: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemvarClass {
    Private,
    Public,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemvarStatement {
    pub memvar_class: MemvarClass,
    pub bindings: Vec<MemvarBinding>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemvarBinding {
    pub name: Symbol,
    pub initializer: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignTarget {
    Symbol(Symbol),
    Memvar(Symbol),
    Index(IndexedAssignTarget),
}

impl AssignTarget {
    pub fn span(&self) -> Span {
        match self {
            Self::Symbol(symbol) => symbol.span,
            Self::Memvar(symbol) => symbol.span,
            Self::Index(target) => target.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedAssignTarget {
    pub root: Symbol,
    pub indices: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignStatement {
    pub target: AssignTarget,
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
    Read(ReadExpression),
    Nil(NilLiteral),
    Logical(LogicalLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    String(StringLiteral),
    Array(ArrayLiteral),
    Codeblock(CodeblockLiteral),
    Macro(MacroExpression),
    Call(CallExpression),
    Index(IndexExpression),
    Assign(AssignExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Postfix(PostfixExpression),
    Error(ErrorExpression),
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Self::Read(expression) => expression.span,
            Self::Nil(literal) => literal.span,
            Self::Logical(literal) => literal.span,
            Self::Integer(literal) => literal.span,
            Self::Float(literal) => literal.span,
            Self::String(literal) => literal.span,
            Self::Array(expression) => expression.span,
            Self::Codeblock(expression) => expression.span,
            Self::Macro(expression) => expression.span,
            Self::Call(expression) => expression.span,
            Self::Index(expression) => expression.span,
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
pub struct ReadExpression {
    pub path: ReadPath,
    pub span: Span,
}

impl ReadExpression {
    pub fn symbol(&self) -> &Symbol {
        match &self.path {
            ReadPath::Name(symbol) => symbol,
            ReadPath::Memvar(symbol) => symbol,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadPath {
    Name(Symbol),
    Memvar(Symbol),
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
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeblockLiteral {
    pub params: Vec<Symbol>,
    pub body: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroExpression {
    pub value: Box<Expression>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignExpression {
    pub target: AssignTarget,
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

fn lower_static_statement(
    statement: &hir::StaticStatement,
    module_static_names: &HashSet<String>,
    errors: &mut Vec<LoweringError>,
) -> StaticStatement {
    StaticStatement {
        bindings: statement
            .bindings
            .iter()
            .map(|binding| StaticBinding {
                name: lower_symbol(&binding.name),
                initializer: binding.initializer.as_ref().map(|expression| {
                    lower_expression(expression, module_static_names, &[], false, errors)
                }),
                span: binding.span,
            })
            .collect(),
        span: statement.span,
    }
}

fn lower_routine_kind(kind: hir::RoutineKind) -> RoutineKind {
    match kind {
        hir::RoutineKind::Procedure => RoutineKind::Procedure,
        hir::RoutineKind::Function => RoutineKind::Function,
    }
}

struct RoutineLowerer<'a> {
    module_static_names: &'a HashSet<String>,
    dynamic_memvar_mode: bool,
    errors: &'a mut Vec<LoweringError>,
    local_scopes: Vec<HashSet<String>>,
    routine: &'a hir::Routine,
}

impl<'a> RoutineLowerer<'a> {
    fn new(
        module_static_names: &'a HashSet<String>,
        routine: &'a hir::Routine,
        dynamic_memvar_mode: bool,
        errors: &'a mut Vec<LoweringError>,
    ) -> Self {
        let mut initial_scope = HashSet::new();
        for param in &routine.params {
            initial_scope.insert(normalize_name(&param.text));
        }
        Self {
            module_static_names,
            dynamic_memvar_mode,
            errors,
            local_scopes: vec![initial_scope],
            routine,
        }
    }

    fn lower(&mut self) -> Routine {
        Routine {
            kind: lower_routine_kind(self.routine.kind),
            name: lower_symbol(&self.routine.name),
            params: self.routine.params.iter().map(lower_symbol).collect(),
            body: self
                .routine
                .body
                .iter()
                .map(|statement| self.lower_statement(statement))
                .collect(),
            span: self.routine.span,
        }
    }

    fn lower_statement(&mut self, statement: &hir::Statement) -> Statement {
        match statement {
            hir::Statement::Local(statement) => {
                for binding in &statement.bindings {
                    self.local_scopes
                        .last_mut()
                        .expect("routine scope")
                        .insert(normalize_name(&binding.name.text));
                }
                Statement::Local(LocalStatement {
                    bindings: statement
                        .bindings
                        .iter()
                        .map(|binding| LocalBinding {
                            name: lower_symbol(&binding.name),
                            initializer: binding
                                .initializer
                                .as_ref()
                                .map(|expression| self.lower_expression(expression)),
                            span: binding.span,
                        })
                        .collect(),
                    span: statement.span,
                })
            }
            hir::Statement::Static(statement) => {
                for binding in &statement.bindings {
                    self.local_scopes
                        .last_mut()
                        .expect("routine scope")
                        .insert(normalize_name(&binding.name.text));
                }
                Statement::Static(StaticStatement {
                    bindings: statement
                        .bindings
                        .iter()
                        .map(|binding| StaticBinding {
                            name: lower_symbol(&binding.name),
                            initializer: binding
                                .initializer
                                .as_ref()
                                .map(|expression| self.lower_expression(expression)),
                            span: binding.span,
                        })
                        .collect(),
                    span: statement.span,
                })
            }
            hir::Statement::Private(statement) => {
                Statement::Private(self.lower_memvar_statement(statement, MemvarClass::Private))
            }
            hir::Statement::Public(statement) => {
                Statement::Public(self.lower_memvar_statement(statement, MemvarClass::Public))
            }
            hir::Statement::If(statement) => Statement::If(Box::new(IfStatement {
                branches: statement
                    .branches
                    .iter()
                    .map(|branch| ConditionalBranch {
                        condition: self.lower_expression(&branch.condition),
                        body: branch
                            .body
                            .iter()
                            .map(|statement| self.lower_statement(statement))
                            .collect(),
                        span: branch.span,
                    })
                    .collect(),
                else_branch: statement.else_branch.as_ref().map(|branch| {
                    branch
                        .iter()
                        .map(|statement| self.lower_statement(statement))
                        .collect()
                }),
                span: statement.span,
            })),
            hir::Statement::DoWhile(statement) => Statement::DoWhile(Box::new(DoWhileStatement {
                condition: self.lower_expression(&statement.condition),
                body: statement
                    .body
                    .iter()
                    .map(|statement| self.lower_statement(statement))
                    .collect(),
                span: statement.span,
            })),
            hir::Statement::For(statement) => {
                self.local_scopes
                    .last_mut()
                    .expect("routine scope")
                    .insert(normalize_name(&statement.variable.text));
                Statement::For(Box::new(ForStatement {
                    variable: lower_symbol(&statement.variable),
                    initial_value: self.lower_expression(&statement.initial_value),
                    limit: self.lower_expression(&statement.limit),
                    step: statement
                        .step
                        .as_ref()
                        .map(|expression| self.lower_expression(expression)),
                    body: statement
                        .body
                        .iter()
                        .map(|statement| self.lower_statement(statement))
                        .collect(),
                    span: statement.span,
                }))
            }
            hir::Statement::Return(statement) => Statement::Return(ReturnStatement {
                value: statement
                    .value
                    .as_ref()
                    .map(|expression| self.lower_expression(expression)),
                span: statement.span,
            }),
            hir::Statement::Print(statement) => Statement::BuiltinCall(BuiltinCallStatement {
                builtin: Builtin::QOut,
                arguments: statement
                    .arguments
                    .iter()
                    .map(|expression| self.lower_expression(expression))
                    .collect(),
                span: statement.span,
            }),
            hir::Statement::Evaluate(statement) => self.lower_expression_statement(statement),
        }
    }

    fn lower_memvar_statement(
        &mut self,
        statement: &hir::MemvarStatement,
        memvar_class: MemvarClass,
    ) -> MemvarStatement {
        MemvarStatement {
            memvar_class,
            bindings: statement
                .bindings
                .iter()
                .map(|binding| MemvarBinding {
                    name: lower_symbol(&binding.name),
                    initializer: binding
                        .initializer
                        .as_ref()
                        .map(|expression| self.lower_expression(expression)),
                    span: binding.span,
                })
                .collect(),
            span: statement.span,
        }
    }

    fn lower_expression_statement(&mut self, statement: &hir::ExpressionStatement) -> Statement {
        match &statement.expression {
            hir::Expression::Assign(expression) => Statement::Assign(AssignStatement {
                target: self.lower_assign_target(&expression.target),
                value: self.lower_expression(&expression.value),
                span: statement.span,
            }),
            expression => Statement::Evaluate(ExpressionStatement {
                expression: self.lower_expression(expression),
                span: statement.span,
            }),
        }
    }

    fn lower_assign_target(&mut self, target: &hir::AssignTarget) -> AssignTarget {
        lower_assign_target(
            target,
            self.module_static_names,
            &self.local_scopes,
            self.dynamic_memvar_mode,
            self.errors,
        )
    }

    fn lower_expression(&mut self, expression: &hir::Expression) -> Expression {
        lower_expression(
            expression,
            self.module_static_names,
            &self.local_scopes,
            self.dynamic_memvar_mode,
            self.errors,
        )
    }
}

fn lower_assign_target(
    target: &hir::AssignTarget,
    module_static_names: &HashSet<String>,
    local_scopes: &[HashSet<String>],
    dynamic_memvar_mode: bool,
    errors: &mut Vec<LoweringError>,
) -> AssignTarget {
    match target {
        hir::AssignTarget::Symbol(symbol) => {
            let lowered = lower_symbol(symbol);
            if is_nominal_name(&lowered.text, module_static_names, local_scopes) {
                AssignTarget::Symbol(lowered)
            } else if dynamic_memvar_mode {
                AssignTarget::Memvar(lowered)
            } else {
                AssignTarget::Symbol(lowered)
            }
        }
        hir::AssignTarget::Index(target) => AssignTarget::Index(IndexedAssignTarget {
            root: lower_symbol(&target.root),
            indices: target
                .indices
                .iter()
                .map(|index| {
                    lower_expression(
                        index,
                        module_static_names,
                        local_scopes,
                        dynamic_memvar_mode,
                        errors,
                    )
                })
                .collect(),
            span: target.span,
        }),
    }
}

fn lower_expression(
    expression: &hir::Expression,
    module_static_names: &HashSet<String>,
    local_scopes: &[HashSet<String>],
    dynamic_memvar_mode: bool,
    errors: &mut Vec<LoweringError>,
) -> Expression {
    match expression {
        hir::Expression::Read(read) => Expression::Read(ReadExpression {
            path: lower_read_path(
                &read.path,
                module_static_names,
                local_scopes,
                dynamic_memvar_mode,
            ),
            span: read.span,
        }),
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
        hir::Expression::Array(expression) => Expression::Array(ArrayLiteral {
            elements: expression
                .elements
                .iter()
                .map(|element| {
                    lower_expression(
                        element,
                        module_static_names,
                        local_scopes,
                        dynamic_memvar_mode,
                        errors,
                    )
                })
                .collect(),
            span: expression.span,
        }),
        hir::Expression::Codeblock(expression) => {
            let params: Vec<Symbol> = expression.params.iter().map(lower_symbol).collect();
            let mut nested_scopes = local_scopes.to_vec();
            nested_scopes.push(
                params
                    .iter()
                    .map(|param| normalize_name(&param.text))
                    .collect(),
            );
            Expression::Codeblock(CodeblockLiteral {
                params,
                body: expression
                    .body
                    .iter()
                    .map(|value| {
                        lower_expression(
                            value,
                            module_static_names,
                            &nested_scopes,
                            dynamic_memvar_mode,
                            errors,
                        )
                    })
                    .collect(),
                span: expression.span,
            })
        }
        hir::Expression::Macro(expression) => Expression::Macro(MacroExpression {
            value: Box::new(lower_expression(
                &expression.value,
                module_static_names,
                local_scopes,
                dynamic_memvar_mode,
                errors,
            )),
            span: expression.span,
        }),
        hir::Expression::Call(expression) => Expression::Call(CallExpression {
            callee: Box::new(lower_call_callee(
                &expression.callee,
                module_static_names,
                local_scopes,
                dynamic_memvar_mode,
                errors,
            )),
            arguments: expression
                .arguments
                .iter()
                .map(|argument| {
                    lower_expression(
                        argument,
                        module_static_names,
                        local_scopes,
                        dynamic_memvar_mode,
                        errors,
                    )
                })
                .collect(),
            span: expression.span,
        }),
        hir::Expression::Index(expression) => Expression::Index(IndexExpression {
            target: Box::new(lower_expression(
                &expression.target,
                module_static_names,
                local_scopes,
                dynamic_memvar_mode,
                errors,
            )),
            indices: expression
                .indices
                .iter()
                .map(|index| {
                    lower_expression(
                        index,
                        module_static_names,
                        local_scopes,
                        dynamic_memvar_mode,
                        errors,
                    )
                })
                .collect(),
            span: expression.span,
        }),
        hir::Expression::Assign(expression) => Expression::Assign(AssignExpression {
            target: lower_assign_target(
                &expression.target,
                module_static_names,
                local_scopes,
                dynamic_memvar_mode,
                errors,
            ),
            value: Box::new(lower_expression(
                &expression.value,
                module_static_names,
                local_scopes,
                dynamic_memvar_mode,
                errors,
            )),
            span: expression.span,
        }),
        hir::Expression::Binary(expression) => Expression::Binary(BinaryExpression {
            left: Box::new(lower_expression(
                &expression.left,
                module_static_names,
                local_scopes,
                dynamic_memvar_mode,
                errors,
            )),
            operator: lower_binary_operator(expression.operator),
            right: Box::new(lower_expression(
                &expression.right,
                module_static_names,
                local_scopes,
                dynamic_memvar_mode,
                errors,
            )),
            span: expression.span,
        }),
        hir::Expression::Unary(expression) => Expression::Unary(UnaryExpression {
            operator: lower_unary_operator(expression.operator),
            operand: Box::new(lower_expression(
                &expression.operand,
                module_static_names,
                local_scopes,
                dynamic_memvar_mode,
                errors,
            )),
            span: expression.span,
        }),
        hir::Expression::Postfix(expression) => Expression::Postfix(PostfixExpression {
            operand: Box::new(lower_expression(
                &expression.operand,
                module_static_names,
                local_scopes,
                dynamic_memvar_mode,
                errors,
            )),
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

fn lower_read_path(
    path: &hir::ReadPath,
    module_static_names: &HashSet<String>,
    local_scopes: &[HashSet<String>],
    dynamic_memvar_mode: bool,
) -> ReadPath {
    match path {
        hir::ReadPath::Name(symbol) => {
            let lowered = lower_symbol(symbol);
            if is_nominal_name(&lowered.text, module_static_names, local_scopes) {
                ReadPath::Name(lowered)
            } else if dynamic_memvar_mode {
                ReadPath::Memvar(lowered)
            } else {
                ReadPath::Name(lowered)
            }
        }
    }
}

fn lower_call_callee(
    expression: &hir::Expression,
    module_static_names: &HashSet<String>,
    local_scopes: &[HashSet<String>],
    dynamic_memvar_mode: bool,
    errors: &mut Vec<LoweringError>,
) -> Expression {
    match expression {
        hir::Expression::Read(read) => Expression::Read(ReadExpression {
            path: match &read.path {
                hir::ReadPath::Name(symbol) => ReadPath::Name(lower_symbol(symbol)),
            },
            span: read.span,
        }),
        other => lower_expression(
            other,
            module_static_names,
            local_scopes,
            dynamic_memvar_mode,
            errors,
        ),
    }
}

fn collect_module_static_names(program: &hir::Program) -> HashSet<String> {
    program
        .module_statics
        .iter()
        .flat_map(|statement| statement.bindings.iter())
        .map(|binding| normalize_name(&binding.name.text))
        .collect()
}

fn program_uses_dynamic_features(program: &hir::Program) -> bool {
    program
        .routines
        .iter()
        .any(|routine| routine.body.iter().any(statement_uses_dynamic_features))
}

fn statement_uses_dynamic_features(statement: &hir::Statement) -> bool {
    match statement {
        hir::Statement::Private(_) | hir::Statement::Public(_) => true,
        hir::Statement::Local(statement) => statement
            .bindings
            .iter()
            .filter_map(|binding| binding.initializer.as_ref())
            .any(expression_uses_dynamic_features),
        hir::Statement::Static(statement) => statement
            .bindings
            .iter()
            .filter_map(|binding| binding.initializer.as_ref())
            .any(expression_uses_dynamic_features),
        hir::Statement::If(statement) => {
            statement.branches.iter().any(|branch| {
                expression_uses_dynamic_features(&branch.condition)
                    || branch.body.iter().any(statement_uses_dynamic_features)
            }) || statement
                .else_branch
                .as_ref()
                .is_some_and(|branch| branch.iter().any(statement_uses_dynamic_features))
        }
        hir::Statement::DoWhile(statement) => {
            expression_uses_dynamic_features(&statement.condition)
                || statement.body.iter().any(statement_uses_dynamic_features)
        }
        hir::Statement::For(statement) => {
            expression_uses_dynamic_features(&statement.initial_value)
                || expression_uses_dynamic_features(&statement.limit)
                || statement
                    .step
                    .as_ref()
                    .is_some_and(expression_uses_dynamic_features)
                || statement.body.iter().any(statement_uses_dynamic_features)
        }
        hir::Statement::Return(statement) => statement
            .value
            .as_ref()
            .is_some_and(expression_uses_dynamic_features),
        hir::Statement::Print(statement) => statement
            .arguments
            .iter()
            .any(expression_uses_dynamic_features),
        hir::Statement::Evaluate(statement) => {
            expression_uses_dynamic_features(&statement.expression)
        }
    }
}

fn expression_uses_dynamic_features(expression: &hir::Expression) -> bool {
    match expression {
        hir::Expression::Codeblock(_) | hir::Expression::Macro(_) => true,
        hir::Expression::Array(expression) => expression
            .elements
            .iter()
            .any(expression_uses_dynamic_features),
        hir::Expression::Call(expression) => {
            expression_uses_dynamic_features(&expression.callee)
                || expression
                    .arguments
                    .iter()
                    .any(expression_uses_dynamic_features)
        }
        hir::Expression::Index(expression) => {
            expression_uses_dynamic_features(&expression.target)
                || expression
                    .indices
                    .iter()
                    .any(expression_uses_dynamic_features)
        }
        hir::Expression::Assign(expression) => {
            expression_uses_dynamic_features(&expression.value)
                || match &expression.target {
                    hir::AssignTarget::Symbol(_) => false,
                    hir::AssignTarget::Index(target) => {
                        target.indices.iter().any(expression_uses_dynamic_features)
                    }
                }
        }
        hir::Expression::Binary(expression) => {
            expression_uses_dynamic_features(&expression.left)
                || expression_uses_dynamic_features(&expression.right)
        }
        hir::Expression::Unary(expression) => expression_uses_dynamic_features(&expression.operand),
        hir::Expression::Postfix(expression) => {
            expression_uses_dynamic_features(&expression.operand)
        }
        hir::Expression::Read(_)
        | hir::Expression::Nil(_)
        | hir::Expression::Logical(_)
        | hir::Expression::Integer(_)
        | hir::Expression::Float(_)
        | hir::Expression::String(_)
        | hir::Expression::Error(_) => false,
    }
}

fn is_nominal_name(
    name: &str,
    module_static_names: &HashSet<String>,
    local_scopes: &[HashSet<String>],
) -> bool {
    let normalized = normalize_name(name);
    module_static_names.contains(&normalized)
        || local_scopes
            .iter()
            .rev()
            .any(|scope| scope.contains(&normalized))
}

fn normalize_name(name: &str) -> String {
    name.to_ascii_uppercase()
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
        ArrayLiteral, AssignStatement, AssignTarget, Builtin, BuiltinCallStatement,
        ErrorExpression, Expression, IndexedAssignTarget, LoweringError, LoweringOutput,
        ReadExpression, ReadPath, ReturnStatement, Routine, RoutineKind, Statement, StaticBinding,
        StaticStatement, Symbol, lower_program,
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
            module_statics: Vec::new(),
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
                            target: hir::AssignTarget::Symbol(symbol(
                                "x",
                                span(24, 3, 4, 25, 3, 5),
                            )),
                            value: Box::new(hir::Expression::Integer(hir::IntegerLiteral {
                                lexeme: "1".to_owned(),
                                span: span(30, 3, 10, 31, 3, 11),
                            })),
                            span: assign_span,
                        }),
                        span: assign_span,
                    }),
                    hir::Statement::Return(hir::ReturnStatement {
                        value: Some(hir::Expression::Read(hir::ReadExpression {
                            path: hir::ReadPath::Name(symbol("x", span(40, 4, 11, 41, 4, 12))),
                            span: span(40, 4, 11, 41, 4, 12),
                        })),
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
                        target: AssignTarget::Symbol(Symbol {
                            text: "x".to_owned(),
                            span: span(24, 3, 4, 25, 3, 5),
                        }),
                        value: Expression::Integer(crate::IntegerLiteral {
                            lexeme: "1".to_owned(),
                            span: span(30, 3, 10, 31, 3, 11),
                        }),
                        span: assign_span,
                    }),
                    Statement::Return(ReturnStatement {
                        value: Some(Expression::Read(ReadExpression {
                            path: ReadPath::Name(Symbol {
                                text: "x".to_owned(),
                                span: span(40, 4, 11, 41, 4, 12),
                            }),
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
            module_statics: Vec::new(),
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
                    module_statics: Vec::new(),
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
    fn lowers_array_literals_to_explicit_ir_nodes() {
        let expression_span = span(15, 2, 6, 24, 2, 15);
        let program = hir::Program {
            module_statics: Vec::new(),
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

        assert_eq!(lowered.errors, Vec::new());
        assert!(matches!(
            lowered.program.routines[0].body[0],
            Statement::Evaluate(crate::ExpressionStatement {
                expression: Expression::Array(ArrayLiteral { span, .. }),
                span: _
            }) if span == expression_span
        ));
    }

    #[test]
    fn lowers_static_statements_to_explicit_ir_nodes() {
        let statement_span = span(12, 2, 4, 33, 2, 25);
        let program = hir::Program {
            module_statics: Vec::new(),
            routines: vec![hir::Routine {
                kind: hir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![hir::Statement::Static(hir::StaticStatement {
                    bindings: vec![hir::StaticBinding {
                        name: symbol("cache", span(19, 2, 11, 24, 2, 16)),
                        initializer: Some(hir::Expression::String(hir::StringLiteral {
                            lexeme: "memo".to_owned(),
                            span: span(28, 2, 20, 34, 2, 26),
                        })),
                        span: span(19, 2, 11, 34, 2, 26),
                    }],
                    span: statement_span,
                })],
                span: span(0, 1, 1, 34, 2, 26),
            }],
        };

        let lowered = lower_program(&program);

        assert_eq!(lowered.errors, Vec::new());
        assert_eq!(
            lowered.program.routines[0].body,
            vec![Statement::Static(StaticStatement {
                bindings: vec![StaticBinding {
                    name: Symbol {
                        text: "cache".to_owned(),
                        span: span(19, 2, 11, 24, 2, 16),
                    },
                    initializer: Some(Expression::String(crate::StringLiteral {
                        lexeme: "memo".to_owned(),
                        span: span(28, 2, 20, 34, 2, 26),
                    })),
                    span: span(19, 2, 11, 34, 2, 26),
                }],
                span: statement_span,
            })]
        );
    }

    #[test]
    fn lowers_array_indexing_to_explicit_ir_nodes() {
        let expression_span = span(15, 2, 6, 25, 2, 16);
        let program = hir::Program {
            module_statics: Vec::new(),
            routines: vec![hir::Routine {
                kind: hir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![hir::Statement::Evaluate(hir::ExpressionStatement {
                    expression: hir::Expression::Index(hir::IndexExpression {
                        target: Box::new(hir::Expression::Read(hir::ReadExpression {
                            path: hir::ReadPath::Name(symbol("matrix", span(15, 2, 6, 21, 2, 12))),
                            span: span(15, 2, 6, 21, 2, 12),
                        })),
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
        assert!(matches!(index.target.as_ref(), Expression::Read(_)));
        assert_eq!(index.indices.len(), 1);
        assert!(matches!(index.indices[0], Expression::Integer(_)));
        assert_eq!(index.span, expression_span);
    }

    #[test]
    fn lowers_identifier_reads_to_explicit_ir_read_paths() {
        let expression_span = span(12, 2, 4, 17, 2, 9);
        let program = hir::Program {
            module_statics: Vec::new(),
            routines: vec![hir::Routine {
                kind: hir::RoutineKind::Function,
                name: symbol("ReadIt", span(0, 1, 1, 6, 1, 7)),
                params: Vec::new(),
                body: vec![hir::Statement::Return(hir::ReturnStatement {
                    value: Some(hir::Expression::Read(hir::ReadExpression {
                        path: hir::ReadPath::Name(symbol("cache", expression_span)),
                        span: expression_span,
                    })),
                    span: expression_span,
                })],
                span: span(0, 1, 1, 17, 2, 9),
            }],
        };

        let lowered = lower_program(&program);

        assert_eq!(lowered.errors, Vec::new());
        let Statement::Return(ReturnStatement {
            value: Some(Expression::Read(read)),
            ..
        }) = &lowered.program.routines[0].body[0]
        else {
            panic!("expected return with explicit read");
        };
        assert_eq!(read.span, expression_span);
        assert!(matches!(
            read.path,
            ReadPath::Name(Symbol { ref text, span }) if text == "cache" && span == expression_span
        ));
    }

    #[test]
    fn lowers_indexed_assignment_targets_to_ir_surface() {
        let assign_span = span(15, 2, 6, 34, 2, 25);
        let program = hir::Program {
            module_statics: Vec::new(),
            routines: vec![hir::Routine {
                kind: hir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![hir::Statement::Evaluate(hir::ExpressionStatement {
                    expression: hir::Expression::Assign(hir::AssignExpression {
                        target: hir::AssignTarget::Index(hir::IndexedAssignTarget {
                            root: symbol("matrix", span(15, 2, 6, 21, 2, 12)),
                            indices: vec![
                                hir::Expression::Integer(hir::IntegerLiteral {
                                    lexeme: "2".to_owned(),
                                    span: span(22, 2, 13, 23, 2, 14),
                                }),
                                hir::Expression::Integer(hir::IntegerLiteral {
                                    lexeme: "1".to_owned(),
                                    span: span(26, 2, 17, 27, 2, 18),
                                }),
                            ],
                            span: span(15, 2, 6, 28, 2, 19),
                        }),
                        value: Box::new(hir::Expression::Integer(hir::IntegerLiteral {
                            lexeme: "99".to_owned(),
                            span: span(32, 2, 23, 34, 2, 25),
                        })),
                        span: assign_span,
                    }),
                    span: assign_span,
                })],
                span: span(0, 1, 1, 34, 2, 25),
            }],
        };

        let lowered = lower_program(&program);

        assert_eq!(lowered.errors, Vec::new());
        let Statement::Assign(assign) = &lowered.program.routines[0].body[0] else {
            panic!("expected lowered assign statement");
        };
        let AssignTarget::Index(IndexedAssignTarget {
            root,
            indices,
            span: target_span,
        }) = &assign.target
        else {
            panic!("expected indexed assign target");
        };
        assert_eq!(root.text, "matrix");
        assert_eq!(indices.len(), 2);
        assert!(matches!(indices[0], Expression::Integer(_)));
        assert!(matches!(indices[1], Expression::Integer(_)));
        assert_eq!(*target_span, span(15, 2, 6, 28, 2, 19));
    }
}
