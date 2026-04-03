use std::{collections::HashMap, fmt};

use harbour_rust_hir as hir;
use harbour_rust_lexer::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub message: String,
    pub span: Span,
}

impl SemanticError {
    pub fn line(&self) -> usize {
        self.span.start.line
    }

    pub fn column(&self) -> usize {
        self.span.start.column
    }
}

impl fmt::Display for SemanticError {
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
pub struct Analysis {
    pub routine_symbols: Vec<RoutineSymbol>,
    pub routines: Vec<RoutineAnalysis>,
    pub errors: Vec<SemanticError>,
}

pub fn analyze_program(program: &hir::Program) -> Analysis {
    let mut errors = Vec::new();
    let mut routine_lookup = HashMap::new();
    let mut routine_symbols: Vec<RoutineSymbol> = Vec::with_capacity(program.routines.len());

    for (id, routine) in program.routines.iter().enumerate() {
        let symbol = RoutineSymbol {
            id,
            name: routine.name.text.clone(),
            span: routine.name.span,
        };
        let key = normalize_name(&symbol.name);
        if let Some(existing) = routine_lookup.insert(key, id) {
            let previous = &routine_symbols[existing];
            errors.push(SemanticError {
                message: format!(
                    "duplicate routine symbol `{}`; first declared at line {}, column {}",
                    symbol.name, previous.span.start.line, previous.span.start.column
                ),
                span: symbol.span,
            });
        }
        routine_symbols.push(symbol);
    }

    let routines = program
        .routines
        .iter()
        .enumerate()
        .map(|(routine_id, routine)| {
            analyze_routine(
                routine_id,
                routine,
                &routine_lookup,
                &routine_symbols,
                &mut errors,
            )
        })
        .collect();

    Analysis {
        routine_symbols,
        routines,
        errors,
    }
}

pub fn render_errors(analysis: &Analysis) -> String {
    let mut out = String::new();
    for error in &analysis.errors {
        out.push_str(&format!("{error}\n"));
    }
    out
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutineSymbol {
    pub id: usize,
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutineAnalysis {
    pub routine_id: usize,
    pub locals: Vec<LocalSymbol>,
    pub resolutions: Vec<SymbolResolution>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSymbol {
    pub id: usize,
    pub kind: LocalSymbolKind,
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSymbolKind {
    Parameter,
    Local,
    Static,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolResolution {
    pub name: String,
    pub span: Span,
    pub binding: Binding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binding {
    Routine(usize),
    Local(usize),
}

struct RoutineAnalyzer<'a> {
    routine_name: &'a str,
    routine_lookup: &'a HashMap<String, usize>,
    locals: Vec<LocalSymbol>,
    local_lookup: HashMap<String, usize>,
    resolutions: Vec<SymbolResolution>,
    errors: &'a mut Vec<SemanticError>,
}

fn analyze_routine(
    routine_id: usize,
    routine: &hir::Routine,
    routine_lookup: &HashMap<String, usize>,
    routine_symbols: &[RoutineSymbol],
    errors: &mut Vec<SemanticError>,
) -> RoutineAnalysis {
    let mut analyzer = RoutineAnalyzer {
        routine_name: &routine_symbols[routine_id].name,
        routine_lookup,
        locals: Vec::new(),
        local_lookup: HashMap::new(),
        resolutions: Vec::new(),
        errors,
    };

    for parameter in &routine.params {
        analyzer.declare_local(parameter, LocalSymbolKind::Parameter);
    }

    for statement in &routine.body {
        analyzer.analyze_statement(statement);
    }

    RoutineAnalysis {
        routine_id,
        locals: analyzer.locals,
        resolutions: analyzer.resolutions,
    }
}

impl<'a> RoutineAnalyzer<'a> {
    fn declare_local(&mut self, symbol: &hir::Symbol, kind: LocalSymbolKind) {
        let key = normalize_name(&symbol.text);
        if let Some(existing) = self.local_lookup.get(&key) {
            let previous = &self.locals[*existing];
            self.errors.push(SemanticError {
                message: format!(
                    "duplicate local symbol `{}` in routine `{}`; first declared at line {}, column {}",
                    symbol.text,
                    self.routine_name,
                    previous.span.start.line,
                    previous.span.start.column
                ),
                span: symbol.span,
            });
            return;
        }

        let id = self.locals.len();
        self.locals.push(LocalSymbol {
            id,
            kind,
            name: symbol.text.clone(),
            span: symbol.span,
        });
        self.local_lookup.insert(key, id);
    }

    fn analyze_statement(&mut self, statement: &hir::Statement) {
        match statement {
            hir::Statement::Local(statement) => {
                for binding in &statement.bindings {
                    if let Some(initializer) = &binding.initializer {
                        self.analyze_expression(initializer, ExpressionContext::Value);
                    }
                    self.declare_local(&binding.name, LocalSymbolKind::Local);
                }
            }
            hir::Statement::Static(statement) => {
                for binding in &statement.bindings {
                    if let Some(initializer) = &binding.initializer {
                        self.analyze_expression(initializer, ExpressionContext::Value);
                    }
                    self.declare_local(&binding.name, LocalSymbolKind::Static);
                }
            }
            hir::Statement::If(statement) => {
                for branch in &statement.branches {
                    self.analyze_expression(&branch.condition, ExpressionContext::Value);
                    for statement in &branch.body {
                        self.analyze_statement(statement);
                    }
                }
                if let Some(else_branch) = &statement.else_branch {
                    for statement in else_branch {
                        self.analyze_statement(statement);
                    }
                }
            }
            hir::Statement::DoWhile(statement) => {
                self.analyze_expression(&statement.condition, ExpressionContext::Value);
                for statement in &statement.body {
                    self.analyze_statement(statement);
                }
            }
            hir::Statement::For(statement) => {
                self.resolve_local_symbol(&statement.variable);
                self.analyze_expression(&statement.initial_value, ExpressionContext::Value);
                self.analyze_expression(&statement.limit, ExpressionContext::Value);
                if let Some(step) = &statement.step {
                    self.analyze_expression(step, ExpressionContext::Value);
                }
                for statement in &statement.body {
                    self.analyze_statement(statement);
                }
            }
            hir::Statement::Return(statement) => {
                if let Some(value) = &statement.value {
                    self.analyze_expression(value, ExpressionContext::Value);
                }
            }
            hir::Statement::Print(statement) => {
                for argument in &statement.arguments {
                    self.analyze_expression(argument, ExpressionContext::Value);
                }
            }
            hir::Statement::Evaluate(statement) => {
                self.analyze_expression(&statement.expression, ExpressionContext::Value);
            }
        }
    }

    fn analyze_expression(&mut self, expression: &hir::Expression, context: ExpressionContext) {
        match expression {
            hir::Expression::Read(read) => match context {
                ExpressionContext::Value => {
                    self.resolve_local_symbol(read.symbol());
                }
                ExpressionContext::CallCallee => {
                    self.resolve_callable_symbol(read.symbol());
                }
            },
            hir::Expression::Nil(_)
            | hir::Expression::Logical(_)
            | hir::Expression::Integer(_)
            | hir::Expression::Float(_)
            | hir::Expression::String(_)
            | hir::Expression::Error(_) => {}
            hir::Expression::Array(expression) => {
                for element in &expression.elements {
                    self.analyze_expression(element, ExpressionContext::Value);
                }
            }
            hir::Expression::Call(expression) => {
                self.analyze_expression(&expression.callee, ExpressionContext::CallCallee);
                for argument in &expression.arguments {
                    self.analyze_expression(argument, ExpressionContext::Value);
                }
            }
            hir::Expression::Index(expression) => {
                self.analyze_expression(&expression.target, ExpressionContext::Value);
                for index in &expression.indices {
                    self.analyze_expression(index, ExpressionContext::Value);
                }
            }
            hir::Expression::Assign(expression) => {
                self.analyze_assign_target(&expression.target);
                self.analyze_expression(&expression.value, ExpressionContext::Value);
            }
            hir::Expression::Binary(expression) => {
                self.analyze_expression(&expression.left, ExpressionContext::Value);
                self.analyze_expression(&expression.right, ExpressionContext::Value);
            }
            hir::Expression::Unary(expression) => {
                self.analyze_expression(&expression.operand, ExpressionContext::Value);
            }
            hir::Expression::Postfix(expression) => {
                self.analyze_expression(&expression.operand, ExpressionContext::Value);
            }
        }
    }

    fn resolve_local_symbol(&mut self, symbol: &hir::Symbol) {
        let key = normalize_name(&symbol.text);
        if let Some(local_id) = self.local_lookup.get(&key) {
            self.resolutions.push(SymbolResolution {
                name: symbol.text.clone(),
                span: symbol.span,
                binding: Binding::Local(*local_id),
            });
            return;
        }

        self.errors.push(SemanticError {
            message: format!(
                "unresolved local symbol `{}` in routine `{}`",
                symbol.text, self.routine_name
            ),
            span: symbol.span,
        });
    }

    fn resolve_callable_symbol(&mut self, symbol: &hir::Symbol) {
        let local_key = normalize_name(&symbol.text);
        if let Some(local_id) = self.local_lookup.get(&local_key) {
            self.resolutions.push(SymbolResolution {
                name: symbol.text.clone(),
                span: symbol.span,
                binding: Binding::Local(*local_id),
            });
            return;
        }

        if let Some(routine_id) = self.routine_lookup.get(&local_key) {
            self.resolutions.push(SymbolResolution {
                name: symbol.text.clone(),
                span: symbol.span,
                binding: Binding::Routine(*routine_id),
            });
            return;
        }

        if is_runtime_builtin(&symbol.text) {
            return;
        }

        self.errors.push(SemanticError {
            message: format!(
                "unresolved callable symbol `{}` in routine `{}`",
                symbol.text, self.routine_name
            ),
            span: symbol.span,
        });
    }

    fn analyze_assign_target(&mut self, target: &hir::AssignTarget) {
        match target {
            hir::AssignTarget::Symbol(symbol) => self.resolve_local_symbol(symbol),
            hir::AssignTarget::Index(target) => {
                self.resolve_local_symbol(&target.root);
                for index in &target.indices {
                    self.analyze_expression(index, ExpressionContext::Value);
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum ExpressionContext {
    Value,
    CallCallee,
}

fn normalize_name(name: &str) -> String {
    name.to_ascii_lowercase()
}

fn is_runtime_builtin(name: &str) -> bool {
    name.eq_ignore_ascii_case("QOUT")
        || name.eq_ignore_ascii_case("ABS")
        || name.eq_ignore_ascii_case("SQRT")
        || name.eq_ignore_ascii_case("SIN")
        || name.eq_ignore_ascii_case("COS")
        || name.eq_ignore_ascii_case("TAN")
        || name.eq_ignore_ascii_case("EXP")
        || name.eq_ignore_ascii_case("LOG")
        || name.eq_ignore_ascii_case("INT")
        || name.eq_ignore_ascii_case("ROUND")
        || name.eq_ignore_ascii_case("MOD")
        || name.eq_ignore_ascii_case("MAX")
        || name.eq_ignore_ascii_case("MIN")
        || name.eq_ignore_ascii_case("LEN")
        || name.eq_ignore_ascii_case("STR")
        || name.eq_ignore_ascii_case("VAL")
        || name.eq_ignore_ascii_case("VALTYPE")
        || name.eq_ignore_ascii_case("TYPE")
        || name.eq_ignore_ascii_case("EMPTY")
        || name.eq_ignore_ascii_case("SUBSTR")
        || name.eq_ignore_ascii_case("LEFT")
        || name.eq_ignore_ascii_case("RIGHT")
        || name.eq_ignore_ascii_case("UPPER")
        || name.eq_ignore_ascii_case("LOWER")
        || name.eq_ignore_ascii_case("TRIM")
        || name.eq_ignore_ascii_case("LTRIM")
        || name.eq_ignore_ascii_case("RTRIM")
        || name.eq_ignore_ascii_case("AT")
        || name.eq_ignore_ascii_case("REPLICATE")
        || name.eq_ignore_ascii_case("SPACE")
        || name.eq_ignore_ascii_case("ACLONE")
        || name.eq_ignore_ascii_case("AADD")
        || name.eq_ignore_ascii_case("ASIZE")
}

#[cfg(test)]
mod tests {
    use harbour_rust_hir as hir;
    use harbour_rust_lexer::{Position, Span};

    use crate::{
        Analysis, Binding, LocalSymbol, LocalSymbolKind, RoutineAnalysis, RoutineSymbol,
        SemanticError, SymbolResolution, analyze_program,
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

    fn read(text: &str, span: Span) -> hir::Expression {
        hir::Expression::Read(hir::ReadExpression {
            path: hir::ReadPath::Name(symbol(text, span)),
            span,
        })
    }

    #[test]
    fn resolves_routine_calls_and_local_symbols() {
        let program = hir::Program {
            routines: vec![
                hir::Routine {
                    kind: hir::RoutineKind::Procedure,
                    name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                    params: vec![symbol("Value", span(10, 1, 11, 15, 1, 16))],
                    body: vec![
                        hir::Statement::Evaluate(hir::ExpressionStatement {
                            expression: hir::Expression::Call(hir::CallExpression {
                                callee: Box::new(read("helper", span(20, 2, 1, 26, 2, 7))),
                                arguments: vec![read("value", span(27, 2, 8, 32, 2, 13))],
                                span: span(20, 2, 1, 33, 2, 14),
                            }),
                            span: span(20, 2, 1, 33, 2, 14),
                        }),
                        hir::Statement::Local(hir::LocalStatement {
                            bindings: vec![hir::LocalBinding {
                                name: symbol("total", span(40, 3, 7, 45, 3, 12)),
                                initializer: Some(read("VALUE", span(49, 3, 16, 54, 3, 21))),
                                span: span(40, 3, 7, 54, 3, 21),
                            }],
                            span: span(34, 3, 1, 54, 3, 21),
                        }),
                        hir::Statement::Return(hir::ReturnStatement {
                            value: Some(read("Total", span(62, 4, 8, 67, 4, 13))),
                            span: span(55, 4, 1, 67, 4, 13),
                        }),
                    ],
                    span: span(0, 1, 1, 67, 4, 13),
                },
                hir::Routine {
                    kind: hir::RoutineKind::Function,
                    name: symbol("Helper", span(68, 6, 1, 74, 6, 7)),
                    params: vec![symbol("x", span(75, 6, 8, 76, 6, 9))],
                    body: Vec::new(),
                    span: span(68, 6, 1, 76, 6, 9),
                },
            ],
        };

        let analysis = analyze_program(&program);

        assert_eq!(
            analysis,
            Analysis {
                routine_symbols: vec![
                    RoutineSymbol {
                        id: 0,
                        name: "Main".to_owned(),
                        span: span(0, 1, 1, 4, 1, 5),
                    },
                    RoutineSymbol {
                        id: 1,
                        name: "Helper".to_owned(),
                        span: span(68, 6, 1, 74, 6, 7),
                    },
                ],
                routines: vec![
                    RoutineAnalysis {
                        routine_id: 0,
                        locals: vec![
                            LocalSymbol {
                                id: 0,
                                kind: LocalSymbolKind::Parameter,
                                name: "Value".to_owned(),
                                span: span(10, 1, 11, 15, 1, 16),
                            },
                            LocalSymbol {
                                id: 1,
                                kind: LocalSymbolKind::Local,
                                name: "total".to_owned(),
                                span: span(40, 3, 7, 45, 3, 12),
                            },
                        ],
                        resolutions: vec![
                            SymbolResolution {
                                name: "helper".to_owned(),
                                span: span(20, 2, 1, 26, 2, 7),
                                binding: Binding::Routine(1),
                            },
                            SymbolResolution {
                                name: "value".to_owned(),
                                span: span(27, 2, 8, 32, 2, 13),
                                binding: Binding::Local(0),
                            },
                            SymbolResolution {
                                name: "VALUE".to_owned(),
                                span: span(49, 3, 16, 54, 3, 21),
                                binding: Binding::Local(0),
                            },
                            SymbolResolution {
                                name: "Total".to_owned(),
                                span: span(62, 4, 8, 67, 4, 13),
                                binding: Binding::Local(1),
                            },
                        ],
                    },
                    RoutineAnalysis {
                        routine_id: 1,
                        locals: vec![LocalSymbol {
                            id: 0,
                            kind: LocalSymbolKind::Parameter,
                            name: "x".to_owned(),
                            span: span(75, 6, 8, 76, 6, 9),
                        }],
                        resolutions: Vec::new(),
                    },
                ],
                errors: Vec::new(),
            }
        );
    }

    #[test]
    fn reports_duplicate_and_unresolved_local_symbols() {
        let program = hir::Program {
            routines: vec![hir::Routine {
                kind: hir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: vec![symbol("value", span(5, 1, 6, 10, 1, 11))],
                body: vec![
                    hir::Statement::Local(hir::LocalStatement {
                        bindings: vec![hir::LocalBinding {
                            name: symbol("Value", span(11, 2, 7, 16, 2, 12)),
                            initializer: None,
                            span: span(11, 2, 7, 16, 2, 12),
                        }],
                        span: span(11, 2, 1, 16, 2, 12),
                    }),
                    hir::Statement::Return(hir::ReturnStatement {
                        value: Some(read("missing", span(17, 3, 8, 24, 3, 15))),
                        span: span(17, 3, 1, 24, 3, 15),
                    }),
                ],
                span: span(0, 1, 1, 24, 3, 15),
            }],
        };

        let analysis = analyze_program(&program);

        assert_eq!(
            analysis.errors,
            vec![
                SemanticError {
                    message:
                        "duplicate local symbol `Value` in routine `Main`; first declared at line 1, column 6"
                            .to_owned(),
                    span: span(11, 2, 7, 16, 2, 12),
                },
                SemanticError {
                    message: "unresolved local symbol `missing` in routine `Main`".to_owned(),
                    span: span(17, 3, 8, 24, 3, 15),
                },
            ]
        );
    }

    #[test]
    fn resolves_static_symbols_without_placeholder_diagnostics() {
        let program = hir::Program {
            routines: vec![hir::Routine {
                kind: hir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![
                    hir::Statement::Static(hir::StaticStatement {
                        bindings: vec![hir::StaticBinding {
                            name: symbol("cache", span(11, 2, 11, 16, 2, 16)),
                            initializer: Some(hir::Expression::String(hir::StringLiteral {
                                lexeme: "memo".to_owned(),
                                span: span(20, 2, 20, 26, 2, 26),
                            })),
                            span: span(11, 2, 11, 26, 2, 26),
                        }],
                        span: span(5, 2, 1, 26, 2, 26),
                    }),
                    hir::Statement::Return(hir::ReturnStatement {
                        value: Some(read("cache", span(34, 3, 8, 39, 3, 13))),
                        span: span(27, 3, 1, 39, 3, 13),
                    }),
                ],
                span: span(0, 1, 1, 39, 3, 13),
            }],
        };

        let analysis = analyze_program(&program);

        assert_eq!(
            analysis.routines[0].locals,
            vec![LocalSymbol {
                id: 0,
                kind: LocalSymbolKind::Static,
                name: "cache".to_owned(),
                span: span(11, 2, 11, 16, 2, 16),
            }]
        );
        assert_eq!(
            analysis.routines[0].resolutions,
            vec![SymbolResolution {
                name: "cache".to_owned(),
                span: span(34, 3, 8, 39, 3, 13),
                binding: Binding::Local(0),
            }]
        );
        assert_eq!(analysis.errors, Vec::new());
    }

    #[test]
    fn resolves_symbols_inside_array_literals() {
        let program = hir::Program {
            routines: vec![hir::Routine {
                kind: hir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![
                    hir::Statement::Local(hir::LocalStatement {
                        bindings: vec![hir::LocalBinding {
                            name: symbol("cache", span(11, 2, 7, 16, 2, 12)),
                            initializer: Some(hir::Expression::Array(hir::ArrayLiteral {
                                elements: vec![
                                    hir::Expression::Integer(hir::IntegerLiteral {
                                        lexeme: "1".to_owned(),
                                        span: span(22, 2, 18, 23, 2, 19),
                                    }),
                                    read("seed", span(25, 2, 21, 29, 2, 25)),
                                ],
                                span: span(20, 2, 16, 30, 2, 26),
                            })),
                            span: span(11, 2, 7, 30, 2, 26),
                        }],
                        span: span(5, 2, 1, 30, 2, 26),
                    }),
                    hir::Statement::Local(hir::LocalStatement {
                        bindings: vec![hir::LocalBinding {
                            name: symbol("seed", span(38, 3, 7, 42, 3, 11)),
                            initializer: None,
                            span: span(38, 3, 7, 42, 3, 11),
                        }],
                        span: span(32, 3, 1, 42, 3, 11),
                    }),
                ],
                span: span(0, 1, 1, 42, 3, 11),
            }],
        };

        let analysis = analyze_program(&program);

        assert_eq!(
            analysis.errors,
            vec![SemanticError {
                message: "unresolved local symbol `seed` in routine `Main`".to_owned(),
                span: span(25, 2, 21, 29, 2, 25),
            }]
        );
    }

    #[test]
    fn resolves_symbols_inside_array_index_expressions() {
        let program = hir::Program {
            routines: vec![hir::Routine {
                kind: hir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![
                    hir::Statement::Local(hir::LocalStatement {
                        bindings: vec![
                            hir::LocalBinding {
                                name: symbol("matrix", span(11, 2, 7, 17, 2, 13)),
                                initializer: None,
                                span: span(11, 2, 7, 17, 2, 13),
                            },
                            hir::LocalBinding {
                                name: symbol("row", span(19, 2, 15, 22, 2, 18)),
                                initializer: None,
                                span: span(19, 2, 15, 22, 2, 18),
                            },
                            hir::LocalBinding {
                                name: symbol("col", span(24, 2, 20, 27, 2, 23)),
                                initializer: None,
                                span: span(24, 2, 20, 27, 2, 23),
                            },
                        ],
                        span: span(5, 2, 1, 27, 2, 23),
                    }),
                    hir::Statement::Return(hir::ReturnStatement {
                        value: Some(hir::Expression::Index(hir::IndexExpression {
                            target: Box::new(read("matrix", span(35, 3, 8, 41, 3, 14))),
                            indices: vec![
                                read("row", span(42, 3, 15, 45, 3, 18)),
                                hir::Expression::Binary(hir::BinaryExpression {
                                    left: Box::new(hir::Expression::Integer(hir::IntegerLiteral {
                                        lexeme: "1".to_owned(),
                                        span: span(47, 3, 20, 48, 3, 21),
                                    })),
                                    operator: hir::BinaryOperator::Add,
                                    right: Box::new(read("col", span(51, 3, 24, 54, 3, 27))),
                                    span: span(47, 3, 20, 54, 3, 27),
                                }),
                            ],
                            span: span(35, 3, 8, 55, 3, 28),
                        })),
                        span: span(28, 3, 1, 55, 3, 28),
                    }),
                ],
                span: span(0, 1, 1, 55, 3, 28),
            }],
        };

        let analysis = analyze_program(&program);

        assert_eq!(analysis.errors, Vec::new());
        assert_eq!(
            analysis.routines[0].resolutions,
            vec![
                SymbolResolution {
                    name: "matrix".to_owned(),
                    span: span(35, 3, 8, 41, 3, 14),
                    binding: Binding::Local(0),
                },
                SymbolResolution {
                    name: "row".to_owned(),
                    span: span(42, 3, 15, 45, 3, 18),
                    binding: Binding::Local(1),
                },
                SymbolResolution {
                    name: "col".to_owned(),
                    span: span(51, 3, 24, 54, 3, 27),
                    binding: Binding::Local(2),
                },
            ]
        );
    }

    #[test]
    fn resolves_explicit_read_paths_without_hir_rewrite() {
        let program = hir::Program {
            routines: vec![
                hir::Routine {
                    kind: hir::RoutineKind::Procedure,
                    name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                    params: Vec::new(),
                    body: vec![
                        hir::Statement::Static(hir::StaticStatement {
                            bindings: vec![hir::StaticBinding {
                                name: symbol("cache", span(11, 2, 11, 16, 2, 16)),
                                initializer: None,
                                span: span(11, 2, 11, 16, 2, 16),
                            }],
                            span: span(5, 2, 1, 16, 2, 16),
                        }),
                        hir::Statement::Evaluate(hir::ExpressionStatement {
                            expression: hir::Expression::Call(hir::CallExpression {
                                callee: Box::new(read("Helper", span(18, 3, 4, 24, 3, 10))),
                                arguments: vec![read("cache", span(25, 3, 11, 30, 3, 16))],
                                span: span(18, 3, 4, 31, 3, 17),
                            }),
                            span: span(18, 3, 4, 31, 3, 17),
                        }),
                    ],
                    span: span(0, 1, 1, 31, 3, 17),
                },
                hir::Routine {
                    kind: hir::RoutineKind::Function,
                    name: symbol("Helper", span(32, 5, 1, 38, 5, 7)),
                    params: Vec::new(),
                    body: Vec::new(),
                    span: span(32, 5, 1, 38, 5, 7),
                },
            ],
        };

        let analysis = analyze_program(&program);

        assert_eq!(
            analysis.routines[0].resolutions,
            vec![
                SymbolResolution {
                    name: "Helper".to_owned(),
                    span: span(18, 3, 4, 24, 3, 10),
                    binding: Binding::Routine(1),
                },
                SymbolResolution {
                    name: "cache".to_owned(),
                    span: span(25, 3, 11, 30, 3, 16),
                    binding: Binding::Local(0),
                },
            ]
        );
        assert_eq!(analysis.errors, Vec::new());
    }
}
