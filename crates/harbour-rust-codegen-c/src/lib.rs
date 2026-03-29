use std::fmt;

use harbour_rust_ir as ir;
use harbour_rust_ir::Builtin;
use harbour_rust_ir::Expression;
use harbour_rust_lexer::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodegenError {
    pub message: String,
    pub span: Span,
}

impl CodegenError {
    pub fn line(&self) -> usize {
        self.span.start.line
    }

    pub fn column(&self) -> usize {
        self.span.start.column
    }
}

impl fmt::Display for CodegenError {
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
pub struct CodegenOutput {
    pub source: String,
    pub errors: Vec<CodegenError>,
}

pub fn emit_program(program: &ir::Program) -> CodegenOutput {
    let mut emitter = Emitter::new();
    emitter.emit_program(program);
    let Emitter { source, errors, .. } = emitter;

    CodegenOutput { source, errors }
}

struct Emitter {
    source: String,
    errors: Vec<CodegenError>,
    indent_level: usize,
}

impl Emitter {
    fn new() -> Self {
        let mut emitter = Self {
            source: String::new(),
            errors: Vec::new(),
            indent_level: 0,
        };
        emitter.emit_prelude();
        emitter
    }

    fn emit_program(&mut self, program: &ir::Program) {
        for routine in &program.routines {
            self.emit_line(&format!(
                "static harbour_runtime_Value {};",
                routine_signature(routine)
            ));
        }

        if !program.routines.is_empty() {
            self.emit_line("");
        }

        if let Some(main_routine) = program.routines.iter().find(|routine| {
            routine.kind == ir::RoutineKind::Procedure
                && routine.name.text.eq_ignore_ascii_case("Main")
        }) {
            self.emit_main_wrapper(main_routine);
        }

        for routine in &program.routines {
            self.emit_routine(routine);
        }
    }

    fn emit_prelude(&mut self) {
        self.emit_line("#include <stdbool.h>");
        self.emit_line("#include <stddef.h>");
        self.emit_line("");
        self.emit_line("typedef struct harbour_runtime_Value harbour_runtime_Value;");
        self.emit_line("");
        self.emit_line("extern harbour_runtime_Value harbour_value_nil(void);");
        self.emit_line("extern harbour_runtime_Value harbour_value_from_logical(bool value);");
        self.emit_line("extern harbour_runtime_Value harbour_value_from_integer(long long value);");
        self.emit_line("extern harbour_runtime_Value harbour_value_from_float(double value);");
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_from_string_literal(const char *value);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_from_array_items(const harbour_runtime_Value *items, size_t length);",
        );
        self.emit_line("extern bool harbour_value_is_true(harbour_runtime_Value value);");
        self.emit_line("extern size_t harbour_value_array_len(harbour_runtime_Value value);");
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_array_get(harbour_runtime_Value value, long long index);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_add(harbour_runtime_Value left, harbour_runtime_Value right);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_less_than(harbour_runtime_Value left, harbour_runtime_Value right);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_less_than_or_equal(harbour_runtime_Value left, harbour_runtime_Value right);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_postfix_increment(harbour_runtime_Value *value);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_qout(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line("");
    }

    fn emit_main_wrapper(&mut self, routine: &ir::Routine) {
        self.emit_line("int main(void) {");
        self.indent_level += 1;
        self.emit_line(&format!("(void) {};", routine_call(routine)));
        self.emit_line("return 0;");
        self.indent_level -= 1;
        self.emit_line("}");
        self.emit_line("");
    }

    fn emit_routine(&mut self, routine: &ir::Routine) {
        self.emit_line(&format!(
            "static harbour_runtime_Value {} {{",
            routine_signature(routine)
        ));
        self.indent_level += 1;

        for statement in &routine.body {
            self.emit_statement(statement);
        }

        if !routine
            .body
            .iter()
            .any(|statement| matches!(statement, ir::Statement::Return(_)))
        {
            self.emit_line("return harbour_value_nil();");
        }

        self.indent_level -= 1;
        self.emit_line("}");
        self.emit_line("");
    }

    fn emit_statement(&mut self, statement: &ir::Statement) {
        match statement {
            ir::Statement::Return(statement) => {
                if let Some(value) = &statement.value {
                    match self.emit_expression(value) {
                        Some(expression) => self.emit_line(&format!("return {};", expression)),
                        None => self.emit_line("return harbour_value_nil();"),
                    }
                } else {
                    self.emit_line("return harbour_value_nil();");
                }
            }
            ir::Statement::BuiltinCall(statement) => self.emit_builtin_call(statement),
            ir::Statement::Local(statement) => {
                for binding in &statement.bindings {
                    let name = mangle_symbol(&binding.name.text);
                    if let Some(initializer) = &binding.initializer {
                        match self.emit_expression(initializer) {
                            Some(expression) => self.emit_line(&format!(
                                "harbour_runtime_Value {} = {};",
                                name, expression
                            )),
                            None => self.emit_line(&format!(
                                "harbour_runtime_Value {} = harbour_value_nil();",
                                name
                            )),
                        }
                    } else {
                        self.emit_line(&format!(
                            "harbour_runtime_Value {} = harbour_value_nil();",
                            name
                        ));
                    }
                }
            }
            ir::Statement::Assign(statement) => match self.emit_expression(&statement.value) {
                Some(expression) => self.emit_line(&format!(
                    "{} = {};",
                    mangle_symbol(&statement.target.text),
                    expression
                )),
                None => self.emit_line(&format!(
                    "{} = harbour_value_nil();",
                    mangle_symbol(&statement.target.text)
                )),
            },
            ir::Statement::If(statement) => {
                self.push_error("C emission for IF is not implemented yet", statement.span);
                self.emit_line("/* TODO: emit IF */");
            }
            ir::Statement::DoWhile(statement) => {
                let condition = self
                    .emit_expression(&statement.condition)
                    .unwrap_or_else(|| "harbour_value_from_logical(false)".to_owned());
                self.emit_line(&format!("while (harbour_value_is_true({})) {{", condition));
                self.indent_level += 1;
                for nested in &statement.body {
                    self.emit_statement(nested);
                }
                self.indent_level -= 1;
                self.emit_line("}");
            }
            ir::Statement::For(statement) => {
                let variable = mangle_symbol(&statement.variable.text);
                let initial_value = self
                    .emit_expression(&statement.initial_value)
                    .unwrap_or_else(|| "harbour_value_nil()".to_owned());
                let limit = self
                    .emit_expression(&statement.limit)
                    .unwrap_or_else(|| "harbour_value_nil()".to_owned());
                let step = if let Some(step) = &statement.step {
                    self.emit_expression(step)
                        .unwrap_or_else(|| "harbour_value_nil()".to_owned())
                } else {
                    "harbour_value_from_integer(1LL)".to_owned()
                };

                self.emit_line(&format!("{} = {};", variable, initial_value));
                self.emit_line(&format!(
                    "while (harbour_value_is_true(harbour_value_less_than_or_equal({}, {}))) {{",
                    variable, limit
                ));
                self.indent_level += 1;
                for nested in &statement.body {
                    self.emit_statement(nested);
                }
                self.emit_line(&format!(
                    "{} = harbour_value_add({}, {});",
                    variable, variable, step
                ));
                self.indent_level -= 1;
                self.emit_line("}");
            }
            ir::Statement::Evaluate(statement) => {
                self.push_error(
                    "C emission for standalone expression statements is not implemented yet",
                    statement.span,
                );
                self.emit_line("/* TODO: emit expression statement */");
            }
        }
    }

    fn emit_builtin_call(&mut self, statement: &ir::BuiltinCallStatement) {
        match statement.builtin {
            Builtin::QOut => {
                let arguments = statement
                    .arguments
                    .iter()
                    .filter_map(|argument| self.emit_expression(argument))
                    .collect::<Vec<_>>();
                let count = arguments.len();

                if count == 0 {
                    self.emit_line("harbour_builtin_qout(NULL, 0);");
                    return;
                }

                self.emit_line(&format!(
                    "harbour_builtin_qout((harbour_runtime_Value[]) {{ {} }}, {});",
                    arguments.join(", "),
                    count
                ));
            }
        }
    }

    fn emit_expression(&mut self, expression: &Expression) -> Option<String> {
        match expression {
            Expression::Symbol(symbol) => Some(mangle_symbol(&symbol.text)),
            Expression::Nil(_) => Some("harbour_value_nil()".to_owned()),
            Expression::Logical(literal) => Some(format!(
                "harbour_value_from_logical({})",
                if literal.value { "true" } else { "false" }
            )),
            Expression::Integer(literal) => {
                Some(format!("harbour_value_from_integer({}LL)", literal.lexeme))
            }
            Expression::Float(literal) => {
                Some(format!("harbour_value_from_float({})", literal.lexeme))
            }
            Expression::String(literal) => Some(format!(
                "harbour_value_from_string_literal(\"{}\")",
                escape_c_string(&normalize_string_lexeme(&literal.lexeme))
            )),
            Expression::Call(expression) => {
                if let Expression::Symbol(symbol) = expression.callee.as_ref() {
                    let arguments = expression
                        .arguments
                        .iter()
                        .filter_map(|argument| self.emit_expression(argument))
                        .collect::<Vec<_>>();
                    Some(format!(
                        "{}({})",
                        mangle_routine_name(&symbol.text),
                        arguments.join(", ")
                    ))
                } else {
                    self.push_error("C emission requires a named call target", expression.span);
                    None
                }
            }
            Expression::Index(expression) => {
                self.push_error(
                    "C emission for array indexing is not implemented yet",
                    expression.span,
                );
                None
            }
            Expression::Binary(expression) => {
                let left = self.emit_expression(&expression.left)?;
                let right = self.emit_expression(&expression.right)?;

                match expression.operator {
                    ir::BinaryOperator::Add => {
                        Some(format!("harbour_value_add({}, {})", left, right))
                    }
                    ir::BinaryOperator::Less => {
                        Some(format!("harbour_value_less_than({}, {})", left, right))
                    }
                    ir::BinaryOperator::LessEqual => Some(format!(
                        "harbour_value_less_than_or_equal({}, {})",
                        left, right
                    )),
                    _ => {
                        self.push_error(
                            "C emission for this binary operator is not implemented yet",
                            expression.span,
                        );
                        None
                    }
                }
            }
            Expression::Unary(expression) => {
                self.push_error(
                    "C emission for unary expressions is not implemented yet",
                    expression.span,
                );
                None
            }
            Expression::Postfix(expression) => {
                match (expression.operator, expression.operand.as_ref()) {
                    (ir::PostfixOperator::Increment, Expression::Symbol(symbol)) => Some(format!(
                        "harbour_value_postfix_increment(&{})",
                        mangle_symbol(&symbol.text)
                    )),
                    _ => {
                        self.push_error(
                            "C emission for this postfix expression is not implemented yet",
                            expression.span,
                        );
                        None
                    }
                }
            }
            Expression::Error(expression) => {
                self.push_error("cannot emit invalid IR expression", expression.span);
                None
            }
        }
    }

    fn push_error(&mut self, message: &str, span: Span) {
        self.errors.push(CodegenError {
            message: message.to_owned(),
            span,
        });
    }

    fn emit_line(&mut self, line: &str) {
        for _ in 0..self.indent_level {
            self.source.push_str("    ");
        }
        self.source.push_str(line);
        self.source.push('\n');
    }
}

fn routine_signature(routine: &ir::Routine) -> String {
    let params = routine
        .params
        .iter()
        .map(|param| format!("harbour_runtime_Value {}", mangle_symbol(&param.text)))
        .collect::<Vec<_>>();

    if params.is_empty() {
        format!("{}(void)", mangle_routine_name(&routine.name.text))
    } else {
        format!(
            "{}({})",
            mangle_routine_name(&routine.name.text),
            params.join(", ")
        )
    }
}

fn routine_call(routine: &ir::Routine) -> String {
    if routine.params.is_empty() {
        format!("{}()", mangle_routine_name(&routine.name.text))
    } else {
        let defaults = routine
            .params
            .iter()
            .map(|_| "harbour_value_nil()".to_owned())
            .collect::<Vec<_>>();
        format!(
            "{}({})",
            mangle_routine_name(&routine.name.text),
            defaults.join(", ")
        )
    }
}

fn mangle_routine_name(name: &str) -> String {
    format!("harbour_routine_{}", mangle_symbol(name))
}

fn mangle_symbol(name: &str) -> String {
    let mut mangled = String::new();

    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            mangled.push(ch.to_ascii_lowercase());
        } else {
            mangled.push('_');
        }
    }

    if mangled.is_empty() {
        "value".to_owned()
    } else {
        mangled
    }
}

fn escape_c_string(value: &str) -> String {
    let mut escaped = String::new();

    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }

    escaped
}

fn normalize_string_lexeme(lexeme: &str) -> String {
    if lexeme.len() >= 2 {
        let mut chars = lexeme.chars();
        let first = chars.next().unwrap_or_default();
        let last = lexeme.chars().next_back().unwrap_or_default();

        if (first == '"' || first == '\'') && first == last {
            return lexeme[1..lexeme.len() - 1].to_owned();
        }
    }

    lexeme.to_owned()
}

#[cfg(test)]
mod tests {
    use harbour_rust_ir as ir;
    use harbour_rust_lexer::{Position, Span};

    use crate::{CodegenOutput, emit_program};

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

    fn symbol(text: &str, span: Span) -> ir::Symbol {
        ir::Symbol {
            text: text.to_owned(),
            span,
        }
    }

    #[test]
    fn emits_c_for_hello_style_procedure() {
        let program = ir::Program {
            routines: vec![ir::Routine {
                kind: ir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![
                    ir::Statement::BuiltinCall(ir::BuiltinCallStatement {
                        builtin: ir::Builtin::QOut,
                        arguments: vec![ir::Expression::String(ir::StringLiteral {
                            lexeme: "Hello, world!".to_owned(),
                            span: span(10, 2, 4, 25, 2, 19),
                        })],
                        span: span(8, 2, 2, 25, 2, 19),
                    }),
                    ir::Statement::Return(ir::ReturnStatement {
                        value: None,
                        span: span(30, 3, 4, 36, 3, 10),
                    }),
                ],
                span: span(0, 1, 1, 36, 3, 10),
            }],
        };

        let emitted = emit_program(&program);

        assert_eq!(
            emitted,
            CodegenOutput {
                source: concat!(
                    "#include <stdbool.h>\n",
                    "#include <stddef.h>\n",
                    "\n",
                    "typedef struct harbour_runtime_Value harbour_runtime_Value;\n",
                    "\n",
                    "extern harbour_runtime_Value harbour_value_nil(void);\n",
                    "extern harbour_runtime_Value harbour_value_from_logical(bool value);\n",
                    "extern harbour_runtime_Value harbour_value_from_integer(long long value);\n",
                    "extern harbour_runtime_Value harbour_value_from_float(double value);\n",
                    "extern harbour_runtime_Value harbour_value_from_string_literal(const char *value);\n",
                    "extern harbour_runtime_Value harbour_value_from_array_items(const harbour_runtime_Value *items, size_t length);\n",
                    "extern bool harbour_value_is_true(harbour_runtime_Value value);\n",
                    "extern size_t harbour_value_array_len(harbour_runtime_Value value);\n",
                    "extern harbour_runtime_Value harbour_value_array_get(harbour_runtime_Value value, long long index);\n",
                    "extern harbour_runtime_Value harbour_value_add(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_less_than(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_less_than_or_equal(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_postfix_increment(harbour_runtime_Value *value);\n",
                    "extern harbour_runtime_Value harbour_builtin_qout(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "\n",
                    "static harbour_runtime_Value harbour_routine_main(void);\n",
                    "\n",
                    "int main(void) {\n",
                    "    (void) harbour_routine_main();\n",
                    "    return 0;\n",
                    "}\n",
                    "\n",
                    "static harbour_runtime_Value harbour_routine_main(void) {\n",
                    "    harbour_builtin_qout((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"Hello, world!\") }, 1);\n",
                    "    return harbour_value_nil();\n",
                    "}\n",
                    "\n",
                )
                .to_owned(),
                errors: Vec::new(),
            }
        );
    }

    #[test]
    fn emits_do_while_using_runtime_condition_helpers() {
        let loop_span = span(12, 2, 4, 24, 4, 9);
        let program = ir::Program {
            routines: vec![ir::Routine {
                kind: ir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![
                    ir::Statement::Local(ir::LocalStatement {
                        bindings: vec![ir::LocalBinding {
                            name: symbol("x", span(8, 2, 5, 9, 2, 6)),
                            initializer: Some(ir::Expression::Integer(ir::IntegerLiteral {
                                lexeme: "0".to_owned(),
                                span: span(13, 2, 10, 14, 2, 11),
                            })),
                            span: span(8, 2, 5, 14, 2, 11),
                        }],
                        span: span(4, 2, 1, 14, 2, 11),
                    }),
                    ir::Statement::DoWhile(Box::new(ir::DoWhileStatement {
                        condition: ir::Expression::Binary(ir::BinaryExpression {
                            left: Box::new(ir::Expression::Postfix(ir::PostfixExpression {
                                operand: Box::new(ir::Expression::Symbol(symbol(
                                    "x",
                                    span(18, 3, 8, 19, 3, 9),
                                ))),
                                operator: ir::PostfixOperator::Increment,
                                span: span(18, 3, 8, 21, 3, 11),
                            })),
                            operator: ir::BinaryOperator::Less,
                            right: Box::new(ir::Expression::Integer(ir::IntegerLiteral {
                                lexeme: "10".to_owned(),
                                span: span(24, 3, 14, 26, 3, 16),
                            })),
                            span: span(18, 3, 8, 26, 3, 16),
                        }),
                        body: vec![ir::Statement::BuiltinCall(ir::BuiltinCallStatement {
                            builtin: ir::Builtin::QOut,
                            arguments: vec![ir::Expression::Symbol(symbol(
                                "x",
                                span(32, 4, 6, 33, 4, 7),
                            ))],
                            span: span(30, 4, 4, 33, 4, 7),
                        })],
                        span: loop_span,
                    })),
                ],
                span: span(0, 1, 1, 24, 4, 9),
            }],
        };

        let emitted = emit_program(&program);

        assert!(emitted.errors.is_empty(), "{:?}", emitted.errors);
        assert!(
            emitted
                .source
                .contains("harbour_runtime_Value x = harbour_value_from_integer(0LL);")
        );
        assert!(emitted.source.contains(
            "while (harbour_value_is_true(harbour_value_less_than(harbour_value_postfix_increment(&x), harbour_value_from_integer(10LL)))) {"
        ));
        assert!(
            emitted
                .source
                .contains("harbour_builtin_qout((harbour_runtime_Value[]) { x }, 1);")
        );
    }

    #[test]
    fn emits_for_loop_with_assignment_updates() {
        let for_span = span(12, 3, 4, 34, 5, 8);
        let program = ir::Program {
            routines: vec![ir::Routine {
                kind: ir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![
                    ir::Statement::Local(ir::LocalStatement {
                        bindings: vec![
                            ir::LocalBinding {
                                name: symbol("n", span(8, 2, 5, 9, 2, 6)),
                                initializer: Some(ir::Expression::Integer(ir::IntegerLiteral {
                                    lexeme: "0".to_owned(),
                                    span: span(13, 2, 10, 14, 2, 11),
                                })),
                                span: span(8, 2, 5, 14, 2, 11),
                            },
                            ir::LocalBinding {
                                name: symbol("sum", span(16, 2, 13, 19, 2, 16)),
                                initializer: Some(ir::Expression::Integer(ir::IntegerLiteral {
                                    lexeme: "0".to_owned(),
                                    span: span(23, 2, 20, 24, 2, 21),
                                })),
                                span: span(16, 2, 13, 24, 2, 21),
                            },
                        ],
                        span: span(4, 2, 1, 24, 2, 21),
                    }),
                    ir::Statement::For(Box::new(ir::ForStatement {
                        variable: symbol("n", span(12, 3, 8, 13, 3, 9)),
                        initial_value: ir::Expression::Integer(ir::IntegerLiteral {
                            lexeme: "1".to_owned(),
                            span: span(18, 3, 14, 19, 3, 15),
                        }),
                        limit: ir::Expression::Integer(ir::IntegerLiteral {
                            lexeme: "5".to_owned(),
                            span: span(23, 3, 19, 24, 3, 20),
                        }),
                        step: None,
                        body: vec![ir::Statement::Assign(ir::AssignStatement {
                            target: symbol("sum", span(30, 4, 7, 33, 4, 10)),
                            value: ir::Expression::Binary(ir::BinaryExpression {
                                left: Box::new(ir::Expression::Symbol(symbol(
                                    "sum",
                                    span(37, 4, 14, 40, 4, 17),
                                ))),
                                operator: ir::BinaryOperator::Add,
                                right: Box::new(ir::Expression::Symbol(symbol(
                                    "n",
                                    span(43, 4, 20, 44, 4, 21),
                                ))),
                                span: span(37, 4, 14, 44, 4, 21),
                            }),
                            span: span(30, 4, 7, 44, 4, 21),
                        })],
                        span: for_span,
                    })),
                ],
                span: span(0, 1, 1, 34, 5, 8),
            }],
        };

        let emitted = emit_program(&program);

        assert!(emitted.errors.is_empty(), "{:?}", emitted.errors);
        assert!(
            emitted
                .source
                .contains("n = harbour_value_from_integer(1LL);")
        );
        assert!(
            emitted
                .source
                .contains("while (harbour_value_is_true(harbour_value_less_than_or_equal(n, harbour_value_from_integer(5LL)))) {")
        );
        assert!(emitted.source.contains("sum = harbour_value_add(sum, n);"));
        assert!(
            emitted
                .source
                .contains("n = harbour_value_add(n, harbour_value_from_integer(1LL));")
        );
    }

    #[test]
    fn reports_array_indexing_as_unimplemented_in_c_emission() {
        let index_span = span(12, 2, 4, 20, 2, 12);
        let program = ir::Program {
            routines: vec![ir::Routine {
                kind: ir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![ir::Statement::Return(ir::ReturnStatement {
                    value: Some(ir::Expression::Index(ir::IndexExpression {
                        target: Box::new(ir::Expression::Symbol(symbol(
                            "matrix",
                            span(12, 2, 4, 18, 2, 10),
                        ))),
                        indices: vec![ir::Expression::Integer(ir::IntegerLiteral {
                            lexeme: "1".to_owned(),
                            span: span(19, 2, 11, 20, 2, 12),
                        })],
                        span: index_span,
                    })),
                    span: index_span,
                })],
                span: span(0, 1, 1, 20, 2, 12),
            }],
        };

        let emitted = emit_program(&program);

        assert_eq!(emitted.errors.len(), 1);
        assert_eq!(
            emitted.errors[0].message,
            "C emission for array indexing is not implemented yet"
        );
    }
}
