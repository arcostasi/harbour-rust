use std::{collections::HashMap, fmt};

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
    current_static_bindings: HashMap<String, StaticBindingStorage>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StaticBindingStorage {
    storage_name: String,
    initialized_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuntimeBuiltin {
    QOut,
    Len,
    Str,
    ValType,
    SubStr,
    Left,
    Right,
    Upper,
    Lower,
    Trim,
    LTrim,
    RTrim,
    At,
    Replicate,
    Space,
    AClone,
    AAdd,
    ASize,
}

impl RuntimeBuiltin {
    fn lookup(name: &str) -> Option<Self> {
        if name.eq_ignore_ascii_case("QOUT") {
            Some(Self::QOut)
        } else if name.eq_ignore_ascii_case("LEN") {
            Some(Self::Len)
        } else if name.eq_ignore_ascii_case("STR") {
            Some(Self::Str)
        } else if name.eq_ignore_ascii_case("VALTYPE") {
            Some(Self::ValType)
        } else if name.eq_ignore_ascii_case("SUBSTR") {
            Some(Self::SubStr)
        } else if name.eq_ignore_ascii_case("LEFT") {
            Some(Self::Left)
        } else if name.eq_ignore_ascii_case("RIGHT") {
            Some(Self::Right)
        } else if name.eq_ignore_ascii_case("UPPER") {
            Some(Self::Upper)
        } else if name.eq_ignore_ascii_case("LOWER") {
            Some(Self::Lower)
        } else if name.eq_ignore_ascii_case("TRIM") {
            Some(Self::Trim)
        } else if name.eq_ignore_ascii_case("LTRIM") {
            Some(Self::LTrim)
        } else if name.eq_ignore_ascii_case("RTRIM") {
            Some(Self::RTrim)
        } else if name.eq_ignore_ascii_case("AT") {
            Some(Self::At)
        } else if name.eq_ignore_ascii_case("REPLICATE") {
            Some(Self::Replicate)
        } else if name.eq_ignore_ascii_case("SPACE") {
            Some(Self::Space)
        } else if name.eq_ignore_ascii_case("ACLONE") {
            Some(Self::AClone)
        } else if name.eq_ignore_ascii_case("AADD") {
            Some(Self::AAdd)
        } else if name.eq_ignore_ascii_case("ASIZE") {
            Some(Self::ASize)
        } else {
            None
        }
    }

    fn helper_name(self) -> &'static str {
        match self {
            Self::QOut => "harbour_builtin_qout",
            Self::Len => "harbour_builtin_len",
            Self::Str => "harbour_builtin_str",
            Self::ValType => "harbour_builtin_valtype",
            Self::SubStr => "harbour_builtin_substr",
            Self::Left => "harbour_builtin_left",
            Self::Right => "harbour_builtin_right",
            Self::Upper => "harbour_builtin_upper",
            Self::Lower => "harbour_builtin_lower",
            Self::Trim => "harbour_builtin_trim",
            Self::LTrim => "harbour_builtin_ltrim",
            Self::RTrim => "harbour_builtin_rtrim",
            Self::At => "harbour_builtin_at",
            Self::Replicate => "harbour_builtin_replicate",
            Self::Space => "harbour_builtin_space",
            Self::AClone => "harbour_builtin_aclone",
            Self::AAdd => "harbour_builtin_aadd",
            Self::ASize => "harbour_builtin_asize",
        }
    }

    fn source_name(self) -> &'static str {
        match self {
            Self::QOut => "QOut",
            Self::Len => "Len",
            Self::Str => "Str",
            Self::ValType => "ValType",
            Self::SubStr => "SubStr",
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Upper => "Upper",
            Self::Lower => "Lower",
            Self::Trim => "Trim",
            Self::LTrim => "LTrim",
            Self::RTrim => "RTrim",
            Self::At => "At",
            Self::Replicate => "Replicate",
            Self::Space => "Space",
            Self::AClone => "AClone",
            Self::AAdd => "AAdd",
            Self::ASize => "ASize",
        }
    }

    fn requires_mutable_dispatch(self) -> bool {
        matches!(self, Self::AAdd | Self::ASize)
    }
}

impl Emitter {
    fn new() -> Self {
        let mut emitter = Self {
            source: String::new(),
            errors: Vec::new(),
            indent_level: 0,
            current_static_bindings: HashMap::new(),
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

        let static_bindings = collect_program_static_bindings(program);
        if !static_bindings.is_empty() {
            self.emit_line("");
            for binding in &static_bindings {
                self.emit_line(&format!(
                    "static harbour_runtime_Value {};",
                    binding.storage_name
                ));
                self.emit_line(&format!("static bool {};", binding.initialized_name));
            }
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
            "extern harbour_runtime_Value harbour_value_array_get(harbour_runtime_Value value, harbour_runtime_Value index);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_array_set_path(harbour_runtime_Value *value, const harbour_runtime_Value *indices, size_t index_count, harbour_runtime_Value assigned);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_equals(harbour_runtime_Value left, harbour_runtime_Value right);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_exact_equals(harbour_runtime_Value left, harbour_runtime_Value right);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_not_equals(harbour_runtime_Value left, harbour_runtime_Value right);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_add(harbour_runtime_Value left, harbour_runtime_Value right);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_subtract(harbour_runtime_Value left, harbour_runtime_Value right);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_multiply(harbour_runtime_Value left, harbour_runtime_Value right);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_divide(harbour_runtime_Value left, harbour_runtime_Value right);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_greater_than(harbour_runtime_Value left, harbour_runtime_Value right);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_value_greater_than_or_equal(harbour_runtime_Value left, harbour_runtime_Value right);",
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
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_len(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_str(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_valtype(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_substr(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_left(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_right(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_upper(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_lower(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_trim(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_ltrim(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_rtrim(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_at(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_replicate(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_space(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_aclone(const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_aadd(harbour_runtime_Value *array, const harbour_runtime_Value *arguments, size_t argument_count);",
        );
        self.emit_line(
            "extern harbour_runtime_Value harbour_builtin_asize(harbour_runtime_Value *array, const harbour_runtime_Value *arguments, size_t argument_count);",
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
        self.current_static_bindings = routine_static_binding_map(routine);
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
        self.current_static_bindings.clear();
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
            ir::Statement::Static(statement) => {
                for binding in &statement.bindings {
                    self.emit_static_binding_initialization(binding);
                }
            }
            ir::Statement::Assign(statement) => self.emit_assign_statement(statement),
            ir::Statement::If(statement) => self.emit_if_statement(statement),
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

    fn emit_if_statement(&mut self, statement: &ir::IfStatement) {
        for (index, branch) in statement.branches.iter().enumerate() {
            let condition = self
                .emit_expression(&branch.condition)
                .unwrap_or_else(|| "harbour_value_from_logical(false)".to_owned());

            if index == 0 {
                self.emit_line(&format!("if (harbour_value_is_true({})) {{", condition));
            } else {
                self.emit_line(&format!(
                    "else if (harbour_value_is_true({})) {{",
                    condition
                ));
            }

            self.indent_level += 1;
            for nested in &branch.body {
                self.emit_statement(nested);
            }
            self.indent_level -= 1;
            self.emit_line("}");
        }

        if let Some(else_branch) = &statement.else_branch {
            self.emit_line("else {");
            self.indent_level += 1;
            for nested in else_branch {
                self.emit_statement(nested);
            }
            self.indent_level -= 1;
            self.emit_line("}");
        }
    }

    fn emit_builtin_call(&mut self, statement: &ir::BuiltinCallStatement) {
        match statement.builtin {
            Builtin::QOut => {
                let Some(call) = self
                    .emit_runtime_builtin_invocation(RuntimeBuiltin::QOut, &statement.arguments)
                else {
                    self.emit_line("harbour_builtin_qout(NULL, 0);");
                    return;
                };

                self.emit_line(&format!("{};", call));
            }
        }
    }

    fn emit_assign_statement(&mut self, statement: &ir::AssignStatement) {
        let Some(value) = self.emit_expression(&statement.value) else {
            match &statement.target {
                ir::AssignTarget::Symbol(target) => self.emit_line(&format!(
                    "{} = harbour_value_nil();",
                    mangle_symbol(&target.text)
                )),
                ir::AssignTarget::Index(_) => {
                    self.emit_line("/* TODO: emit indexed assignment */");
                }
            }
            return;
        };

        match &statement.target {
            ir::AssignTarget::Symbol(target) => {
                self.emit_line(&format!(
                    "{} = {};",
                    self.resolve_symbol_storage_name(&target.text),
                    value
                ));
            }
            ir::AssignTarget::Index(target) => {
                let mut indices = Vec::with_capacity(target.indices.len());
                for index in &target.indices {
                    let Some(index_expression) = self.emit_expression(index) else {
                        self.emit_line("/* TODO: emit indexed assignment */");
                        return;
                    };
                    indices.push(index_expression);
                }

                self.emit_line(&format!(
                    "(void) harbour_value_array_set_path(&{}, (harbour_runtime_Value[]) {{ {} }}, {}, {});",
                    self.resolve_symbol_storage_name(&target.root.text),
                    indices.join(", "),
                    indices.len(),
                    value
                ));
            }
        }
    }

    fn emit_expression(&mut self, expression: &Expression) -> Option<String> {
        match expression {
            Expression::Read(read) => self.emit_read_expression(read, read.span),
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
            Expression::Array(expression) => {
                let mut elements = Vec::with_capacity(expression.elements.len());
                for element in &expression.elements {
                    elements.push(self.emit_expression(element)?);
                }

                if elements.is_empty() {
                    Some("harbour_value_from_array_items(NULL, 0)".to_owned())
                } else {
                    Some(format!(
                        "harbour_value_from_array_items((harbour_runtime_Value[]) {{ {} }}, {})",
                        elements.join(", "),
                        elements.len()
                    ))
                }
            }
            Expression::Call(expression) => {
                if let Some(symbol) = self.named_read_symbol(expression.callee.as_ref()) {
                    if let Some(builtin) = RuntimeBuiltin::lookup(&symbol.text) {
                        self.emit_runtime_builtin_expression(
                            builtin,
                            &expression.arguments,
                            expression.span,
                        )
                    } else {
                        let mut arguments = Vec::with_capacity(expression.arguments.len());
                        for argument in &expression.arguments {
                            arguments.push(self.emit_expression(argument)?);
                        }
                        Some(format!(
                            "{}({})",
                            mangle_routine_name(&symbol.text),
                            arguments.join(", ")
                        ))
                    }
                } else {
                    self.push_error("C emission requires a named call target", expression.span);
                    None
                }
            }
            Expression::Index(expression) => {
                let mut target = self.emit_expression(&expression.target)?;

                for index in &expression.indices {
                    let emitted_index = self.emit_expression(index)?;
                    target = format!("harbour_value_array_get({}, {})", target, emitted_index);
                }

                Some(target)
            }
            Expression::Binary(expression) => {
                let left = self.emit_expression(&expression.left)?;
                let right = self.emit_expression(&expression.right)?;

                match expression.operator {
                    ir::BinaryOperator::Equal => {
                        Some(format!("harbour_value_equals({}, {})", left, right))
                    }
                    ir::BinaryOperator::ExactEqual => {
                        Some(format!("harbour_value_exact_equals({}, {})", left, right))
                    }
                    ir::BinaryOperator::NotEqual => {
                        Some(format!("harbour_value_not_equals({}, {})", left, right))
                    }
                    ir::BinaryOperator::Add => {
                        Some(format!("harbour_value_add({}, {})", left, right))
                    }
                    ir::BinaryOperator::Subtract => {
                        Some(format!("harbour_value_subtract({}, {})", left, right))
                    }
                    ir::BinaryOperator::Multiply => {
                        Some(format!("harbour_value_multiply({}, {})", left, right))
                    }
                    ir::BinaryOperator::Divide => {
                        Some(format!("harbour_value_divide({}, {})", left, right))
                    }
                    ir::BinaryOperator::Greater => {
                        Some(format!("harbour_value_greater_than({}, {})", left, right))
                    }
                    ir::BinaryOperator::GreaterEqual => Some(format!(
                        "harbour_value_greater_than_or_equal({}, {})",
                        left, right
                    )),
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
                match (
                    expression.operator,
                    self.named_read_symbol(expression.operand.as_ref()),
                ) {
                    (ir::PostfixOperator::Increment, Some(symbol)) => Some(format!(
                        "harbour_value_postfix_increment(&{})",
                        self.resolve_symbol_storage_name(&symbol.text)
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

    fn emit_runtime_builtin_expression(
        &mut self,
        builtin: RuntimeBuiltin,
        arguments: &[Expression],
        span: Span,
    ) -> Option<String> {
        if builtin.requires_mutable_dispatch() {
            return self.emit_mutable_runtime_builtin_expression(builtin, arguments, span);
        }

        self.emit_runtime_builtin_invocation(builtin, arguments)
    }

    fn emit_runtime_builtin_invocation(
        &mut self,
        builtin: RuntimeBuiltin,
        arguments: &[Expression],
    ) -> Option<String> {
        let mut emitted_arguments = Vec::with_capacity(arguments.len());
        for argument in arguments {
            emitted_arguments.push(self.emit_expression(argument)?);
        }

        if emitted_arguments.is_empty() {
            Some(format!("{}(NULL, 0)", builtin.helper_name()))
        } else {
            Some(format!(
                "{}((harbour_runtime_Value[]) {{ {} }}, {})",
                builtin.helper_name(),
                emitted_arguments.join(", "),
                emitted_arguments.len()
            ))
        }
    }

    fn emit_mutable_runtime_builtin_expression(
        &mut self,
        builtin: RuntimeBuiltin,
        arguments: &[Expression],
        span: Span,
    ) -> Option<String> {
        let Some((target, remaining_arguments)) = arguments.split_first() else {
            return Some(format!("{}(NULL, NULL, 0)", builtin.helper_name()));
        };

        let Some(symbol) = self.named_read_symbol(target) else {
            self.push_error(
                &format!(
                    "C emission for mutable builtin {} requires an addressable first argument",
                    builtin.source_name()
                ),
                span,
            );
            return None;
        };

        let mut emitted_arguments = Vec::with_capacity(remaining_arguments.len());
        for argument in remaining_arguments {
            emitted_arguments.push(self.emit_expression(argument)?);
        }

        let addressable_target = format!("&{}", self.resolve_symbol_storage_name(&symbol.text));
        if emitted_arguments.is_empty() {
            Some(format!(
                "{}({}, NULL, 0)",
                builtin.helper_name(),
                addressable_target
            ))
        } else {
            Some(format!(
                "{}({}, (harbour_runtime_Value[]) {{ {} }}, {})",
                builtin.helper_name(),
                addressable_target,
                emitted_arguments.join(", "),
                emitted_arguments.len()
            ))
        }
    }

    fn emit_read_expression(&mut self, read: &ir::ReadExpression, _span: Span) -> Option<String> {
        match &read.path {
            ir::ReadPath::Name(symbol) => Some(self.resolve_symbol_storage_name(&symbol.text)),
        }
    }

    fn named_read_symbol<'a>(&self, expression: &'a Expression) -> Option<&'a ir::Symbol> {
        match expression {
            Expression::Read(read) => match &read.path {
                ir::ReadPath::Name(symbol) => Some(symbol),
            },
            _ => None,
        }
    }

    fn emit_static_binding_initialization(&mut self, binding: &ir::StaticBinding) {
        let Some(storage) = self
            .current_static_bindings
            .get(&normalize_symbol_name(&binding.name.text))
            .cloned()
        else {
            self.push_error(
                &format!(
                    "missing C storage mapping for STATIC symbol `{}`",
                    binding.name.text
                ),
                binding.span,
            );
            return;
        };

        let initializer = if let Some(expression) = &binding.initializer {
            self.emit_expression(expression)
                .unwrap_or_else(|| "harbour_value_nil()".to_owned())
        } else {
            "harbour_value_nil()".to_owned()
        };

        self.emit_line(&format!("if (!{}) {{", storage.initialized_name));
        self.indent_level += 1;
        self.emit_line(&format!("{} = {};", storage.storage_name, initializer));
        self.emit_line(&format!("{} = true;", storage.initialized_name));
        self.indent_level -= 1;
        self.emit_line("}");
    }

    fn resolve_symbol_storage_name(&self, name: &str) -> String {
        self.current_static_bindings
            .get(&normalize_symbol_name(name))
            .map(|binding| binding.storage_name.clone())
            .unwrap_or_else(|| mangle_symbol(name))
    }
}

fn collect_program_static_bindings(program: &ir::Program) -> Vec<StaticBindingStorage> {
    program
        .routines
        .iter()
        .flat_map(collect_routine_static_bindings)
        .collect()
}

fn routine_static_binding_map(routine: &ir::Routine) -> HashMap<String, StaticBindingStorage> {
    let mut seen = HashMap::<String, StaticBindingStorage>::new();
    collect_statement_static_bindings(&routine.name.text, &routine.body, &mut seen);
    seen
}

fn collect_routine_static_bindings(routine: &ir::Routine) -> Vec<StaticBindingStorage> {
    let mut seen = HashMap::<String, StaticBindingStorage>::new();
    collect_statement_static_bindings(&routine.name.text, &routine.body, &mut seen);
    seen.into_values().collect()
}

fn collect_statement_static_bindings(
    routine_name: &str,
    statements: &[ir::Statement],
    seen: &mut HashMap<String, StaticBindingStorage>,
) {
    for statement in statements {
        match statement {
            ir::Statement::Static(statement) => {
                for binding in &statement.bindings {
                    let normalized_name = normalize_symbol_name(&binding.name.text);
                    seen.entry(normalized_name)
                        .or_insert_with(|| StaticBindingStorage {
                            storage_name: mangle_static_storage_name(
                                routine_name,
                                &binding.name.text,
                            ),
                            initialized_name: mangle_static_initialized_name(
                                routine_name,
                                &binding.name.text,
                            ),
                        });
                }
            }
            ir::Statement::If(statement) => {
                for branch in &statement.branches {
                    collect_statement_static_bindings(routine_name, &branch.body, seen);
                }
                if let Some(else_branch) = &statement.else_branch {
                    collect_statement_static_bindings(routine_name, else_branch, seen);
                }
            }
            ir::Statement::DoWhile(statement) => {
                collect_statement_static_bindings(routine_name, &statement.body, seen);
            }
            ir::Statement::For(statement) => {
                collect_statement_static_bindings(routine_name, &statement.body, seen);
            }
            _ => {}
        }
    }
}

fn mangle_static_storage_name(routine_name: &str, symbol_name: &str) -> String {
    format!(
        "harbour_static_{}_{}",
        mangle_symbol(routine_name),
        mangle_symbol(symbol_name)
    )
}

fn mangle_static_initialized_name(routine_name: &str, symbol_name: &str) -> String {
    format!(
        "{}__initialized",
        mangle_static_storage_name(routine_name, symbol_name)
    )
}

fn normalize_symbol_name(name: &str) -> String {
    name.to_ascii_lowercase()
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

    fn read(text: &str, span: Span) -> ir::Expression {
        ir::Expression::Read(ir::ReadExpression {
            path: ir::ReadPath::Name(symbol(text, span)),
            span,
        })
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
                    "extern harbour_runtime_Value harbour_value_array_get(harbour_runtime_Value value, harbour_runtime_Value index);\n",
                    "extern harbour_runtime_Value harbour_value_array_set_path(harbour_runtime_Value *value, const harbour_runtime_Value *indices, size_t index_count, harbour_runtime_Value assigned);\n",
                    "extern harbour_runtime_Value harbour_value_equals(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_exact_equals(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_not_equals(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_add(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_subtract(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_multiply(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_divide(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_greater_than(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_greater_than_or_equal(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_less_than(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_less_than_or_equal(harbour_runtime_Value left, harbour_runtime_Value right);\n",
                    "extern harbour_runtime_Value harbour_value_postfix_increment(harbour_runtime_Value *value);\n",
                    "extern harbour_runtime_Value harbour_builtin_qout(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_len(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_str(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_valtype(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_substr(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_left(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_right(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_upper(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_lower(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_trim(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_ltrim(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_rtrim(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_at(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_replicate(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_space(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_aclone(const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_aadd(harbour_runtime_Value *array, const harbour_runtime_Value *arguments, size_t argument_count);\n",
                    "extern harbour_runtime_Value harbour_builtin_asize(harbour_runtime_Value *array, const harbour_runtime_Value *arguments, size_t argument_count);\n",
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
                                operand: Box::new(read("x", span(18, 3, 8, 19, 3, 9))),
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
                            arguments: vec![read("x", span(32, 4, 6, 33, 4, 7))],
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
                            target: ir::AssignTarget::Symbol(symbol(
                                "sum",
                                span(30, 4, 7, 33, 4, 10),
                            )),
                            value: ir::Expression::Binary(ir::BinaryExpression {
                                left: Box::new(read("sum", span(37, 4, 14, 40, 4, 17))),
                                operator: ir::BinaryOperator::Add,
                                right: Box::new(read("n", span(43, 4, 20, 44, 4, 21))),
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
    fn emits_array_indexing_with_runtime_helpers() {
        let index_span = span(12, 2, 4, 20, 2, 12);
        let program = ir::Program {
            routines: vec![ir::Routine {
                kind: ir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![ir::Statement::Return(ir::ReturnStatement {
                    value: Some(ir::Expression::Index(ir::IndexExpression {
                        target: Box::new(read("matrix", span(12, 2, 4, 18, 2, 10))),
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

        assert!(emitted.errors.is_empty(), "{:?}", emitted.errors);
        assert!(
            emitted.source.contains(
                "return harbour_value_array_get(matrix, harbour_value_from_integer(1LL));"
            )
        );
    }

    #[test]
    fn emits_indexed_assignment_with_runtime_set_path_helper() {
        let assign_span = span(12, 2, 4, 32, 2, 24);
        let program = ir::Program {
            routines: vec![ir::Routine {
                kind: ir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![ir::Statement::Assign(ir::AssignStatement {
                    target: ir::AssignTarget::Index(ir::IndexedAssignTarget {
                        root: symbol("matrix", span(12, 2, 4, 18, 2, 10)),
                        indices: vec![
                            ir::Expression::Integer(ir::IntegerLiteral {
                                lexeme: "2".to_owned(),
                                span: span(19, 2, 11, 20, 2, 12),
                            }),
                            ir::Expression::Integer(ir::IntegerLiteral {
                                lexeme: "1".to_owned(),
                                span: span(22, 2, 14, 23, 2, 15),
                            }),
                        ],
                        span: span(12, 2, 4, 23, 2, 15),
                    }),
                    value: ir::Expression::Integer(ir::IntegerLiteral {
                        lexeme: "99".to_owned(),
                        span: span(28, 2, 20, 30, 2, 22),
                    }),
                    span: assign_span,
                })],
                span: span(0, 1, 1, 32, 2, 24),
            }],
        };

        let emitted = emit_program(&program);

        assert!(emitted.errors.is_empty(), "{:?}", emitted.errors);
        assert!(emitted.source.contains(
            "(void) harbour_value_array_set_path(&matrix, (harbour_runtime_Value[]) { harbour_value_from_integer(2LL), harbour_value_from_integer(1LL) }, 2, harbour_value_from_integer(99LL));"
        ));
    }

    #[test]
    fn emits_array_literals_with_runtime_helpers() {
        let array_span = span(12, 2, 4, 22, 2, 14);
        let program = ir::Program {
            routines: vec![ir::Routine {
                kind: ir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![ir::Statement::Return(ir::ReturnStatement {
                    value: Some(ir::Expression::Array(ir::ArrayLiteral {
                        elements: vec![ir::Expression::Integer(ir::IntegerLiteral {
                            lexeme: "1".to_owned(),
                            span: span(14, 2, 6, 15, 2, 7),
                        })],
                        span: array_span,
                    })),
                    span: array_span,
                })],
                span: span(0, 1, 1, 22, 2, 14),
            }],
        };

        let emitted = emit_program(&program);

        assert!(emitted.errors.is_empty(), "{:?}", emitted.errors);
        assert!(
            emitted
                .source
                .contains("return harbour_value_from_array_items((harbour_runtime_Value[]) { harbour_value_from_integer(1LL) }, 1);")
        );
    }

    #[test]
    fn emits_runtime_builtin_calls_for_aclone_expressions() {
        let call_span = span(12, 2, 4, 26, 2, 18);
        let program = ir::Program {
            routines: vec![ir::Routine {
                kind: ir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![ir::Statement::Return(ir::ReturnStatement {
                    value: Some(ir::Expression::Call(ir::CallExpression {
                        callee: Box::new(read("AClone", span(12, 2, 4, 18, 2, 10))),
                        arguments: vec![read("source", span(20, 2, 12, 26, 2, 18))],
                        span: call_span,
                    })),
                    span: call_span,
                })],
                span: span(0, 1, 1, 26, 2, 18),
            }],
        };

        let emitted = emit_program(&program);

        assert!(emitted.errors.is_empty(), "{:?}", emitted.errors);
        assert!(
            emitted.source.contains(
                "return harbour_builtin_aclone((harbour_runtime_Value[]) { source }, 1);"
            )
        );
    }

    #[test]
    fn reports_mutable_runtime_builtin_calls_as_codegen_errors() {
        let call_span = span(12, 2, 4, 24, 2, 16);
        let program = ir::Program {
            routines: vec![ir::Routine {
                kind: ir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![ir::Statement::Return(ir::ReturnStatement {
                    value: Some(ir::Expression::Call(ir::CallExpression {
                        callee: Box::new(read("AAdd", span(12, 2, 4, 16, 2, 8))),
                        arguments: vec![ir::Expression::Array(ir::ArrayLiteral {
                            elements: vec![],
                            span: span(18, 2, 10, 20, 2, 12),
                        })],
                        span: call_span,
                    })),
                    span: call_span,
                })],
                span: span(0, 1, 1, 24, 2, 16),
            }],
        };

        let emitted = emit_program(&program);

        assert_eq!(emitted.errors.len(), 1);
        assert_eq!(
            emitted.errors[0].message,
            "C emission for mutable builtin AAdd requires an addressable first argument"
        );
    }

    #[test]
    fn emits_mutable_runtime_builtin_calls_for_symbol_first_argument() {
        let call_span = span(12, 2, 4, 31, 2, 23);
        let program = ir::Program {
            routines: vec![ir::Routine {
                kind: ir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![ir::Statement::Return(ir::ReturnStatement {
                    value: Some(ir::Expression::Call(ir::CallExpression {
                        callee: Box::new(read("ASize", span(12, 2, 4, 17, 2, 9))),
                        arguments: vec![
                            read("items", span(18, 2, 10, 23, 2, 15)),
                            ir::Expression::Integer(ir::IntegerLiteral {
                                lexeme: "3".to_owned(),
                                span: span(25, 2, 17, 26, 2, 18),
                            }),
                        ],
                        span: call_span,
                    })),
                    span: call_span,
                })],
                span: span(0, 1, 1, 31, 2, 23),
            }],
        };

        let emitted = emit_program(&program);

        assert!(emitted.errors.is_empty(), "{:?}", emitted.errors);
        assert!(
            emitted.source.contains(
                "return harbour_builtin_asize(&items, (harbour_runtime_Value[]) { harbour_value_from_integer(3LL) }, 1);"
            )
        );
    }

    #[test]
    fn emits_static_statements_as_persistent_c_storage() {
        let static_span = span(12, 2, 4, 32, 2, 24);
        let program = ir::Program {
            routines: vec![ir::Routine {
                kind: ir::RoutineKind::Procedure,
                name: symbol("Main", span(0, 1, 1, 4, 1, 5)),
                params: Vec::new(),
                body: vec![ir::Statement::Static(ir::StaticStatement {
                    bindings: vec![ir::StaticBinding {
                        name: symbol("cache", span(19, 2, 11, 24, 2, 16)),
                        initializer: None,
                        span: span(19, 2, 11, 24, 2, 16),
                    }],
                    span: static_span,
                })],
                span: span(0, 1, 1, 32, 2, 24),
            }],
        };

        let emitted = emit_program(&program);

        assert!(emitted.errors.is_empty(), "{:?}", emitted.errors);
        assert!(
            emitted
                .source
                .contains("static harbour_runtime_Value harbour_static_main_cache;")
        );
        assert!(
            emitted
                .source
                .contains("static bool harbour_static_main_cache__initialized;")
        );
        assert!(
            emitted
                .source
                .contains("if (!harbour_static_main_cache__initialized) {")
        );
        assert!(
            emitted
                .source
                .contains("harbour_static_main_cache = harbour_value_nil();")
        );
        assert!(
            emitted
                .source
                .contains("harbour_static_main_cache__initialized = true;")
        );
    }
}
