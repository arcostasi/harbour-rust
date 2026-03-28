use harbour_rust_ast::{
    BinaryExpression, BinaryOperator, CallExpression, Expression, Identifier, Item,
    PostfixExpression, PostfixOperator, Program, Routine, RoutineKind, Statement, UnaryExpression,
    UnaryOperator,
};
use harbour_rust_lexer::{LexedSource, Position, Span, lex};
use harbour_rust_parser::{ParseOutput, parse};

pub fn render_lexed(source: &str) -> String {
    let lexed = lex(source);
    render_lexed_source(source, &lexed)
}

pub fn render_lexed_source(source: &str, lexed: &LexedSource) -> String {
    let mut out = String::new();

    for token in &lexed.tokens {
        let text = token.text(source).escape_default().to_string();
        out.push_str(&format!(
            "{}:{}-{}:{} {:?} \"{}\"\n",
            token.span.start.line,
            token.span.start.column,
            token.span.end.line,
            token.span.end.column,
            token.kind,
            text
        ));
    }

    if !lexed.errors.is_empty() {
        out.push_str("-- errors --\n");
        for error in &lexed.errors {
            out.push_str(&format!("{error}\n"));
        }
    }

    out
}

pub fn render_parsed(source: &str) -> String {
    let parsed = parse(source);
    render_parse_output(&parsed)
}

pub fn render_parse_output(parsed: &ParseOutput) -> String {
    let mut out = String::new();
    render_program(&mut out, &parsed.program, 0);

    if !parsed.errors.is_empty() {
        out.push_str("Errors\n");
        for error in &parsed.errors {
            push_indent(&mut out, 1);
            out.push_str(&format!("{error}\n"));
        }
    }

    out
}

fn render_program(out: &mut String, program: &Program, level: usize) {
    push_line(out, level, "Program");
    for item in &program.items {
        render_item(out, item, level + 1);
    }
}

fn render_item(out: &mut String, item: &Item, level: usize) {
    match item {
        Item::Routine(routine) => render_routine(out, routine, level),
    }
}

fn render_routine(out: &mut String, routine: &Routine, level: usize) {
    push_line(
        out,
        level,
        &format!(
            "Routine {} {} [{}]",
            render_routine_kind(routine.kind),
            routine.name.text,
            format_span(routine.span)
        ),
    );
    if routine.params.is_empty() {
        push_line(out, level + 1, "Params []");
    } else {
        push_line(out, level + 1, "Params");
        for param in &routine.params {
            render_identifier(out, "Param", param, level + 2);
        }
    }

    if routine.body.is_empty() {
        push_line(out, level + 1, "Body []");
    } else {
        push_line(out, level + 1, "Body");
        for statement in &routine.body {
            render_statement(out, statement, level + 2);
        }
    }
}

fn render_statement(out: &mut String, statement: &Statement, level: usize) {
    match statement {
        Statement::Local(statement) => {
            push_line(
                out,
                level,
                &format!("Local [{}]", format_span(statement.span)),
            );
            for binding in &statement.bindings {
                push_line(
                    out,
                    level + 1,
                    &format!(
                        "Binding {} [{}]",
                        binding.name.text,
                        format_span(binding.span)
                    ),
                );
                if let Some(initializer) = &binding.initializer {
                    render_expression(out, initializer, level + 2);
                }
            }
        }
        Statement::If(statement) => {
            push_line(out, level, &format!("If [{}]", format_span(statement.span)));
            for (index, branch) in statement.branches.iter().enumerate() {
                push_line(
                    out,
                    level + 1,
                    &format!("Branch {} [{}]", index, format_span(branch.span)),
                );
                push_line(out, level + 2, "Condition");
                render_expression(out, &branch.condition, level + 3);
                if branch.body.is_empty() {
                    push_line(out, level + 2, "Body []");
                } else {
                    push_line(out, level + 2, "Body");
                    for nested in &branch.body {
                        render_statement(out, nested, level + 3);
                    }
                }
            }
            if let Some(else_branch) = &statement.else_branch {
                if else_branch.is_empty() {
                    push_line(out, level + 1, "Else []");
                } else {
                    push_line(out, level + 1, "Else");
                    for nested in else_branch {
                        render_statement(out, nested, level + 2);
                    }
                }
            }
        }
        Statement::DoWhile(statement) => {
            push_line(
                out,
                level,
                &format!("DoWhile [{}]", format_span(statement.span)),
            );
            push_line(out, level + 1, "Condition");
            render_expression(out, &statement.condition, level + 2);
            if statement.body.is_empty() {
                push_line(out, level + 1, "Body []");
            } else {
                push_line(out, level + 1, "Body");
                for nested in &statement.body {
                    render_statement(out, nested, level + 2);
                }
            }
        }
        Statement::For(statement) => {
            push_line(
                out,
                level,
                &format!("For [{}]", format_span(statement.span)),
            );
            render_identifier(out, "Variable", &statement.variable, level + 1);
            push_line(out, level + 1, "Initial");
            render_expression(out, &statement.initial_value, level + 2);
            push_line(out, level + 1, "Limit");
            render_expression(out, &statement.limit, level + 2);
            if let Some(step) = &statement.step {
                push_line(out, level + 1, "Step");
                render_expression(out, step, level + 2);
            }
            if statement.body.is_empty() {
                push_line(out, level + 1, "Body []");
            } else {
                push_line(out, level + 1, "Body");
                for nested in &statement.body {
                    render_statement(out, nested, level + 2);
                }
            }
        }
        Statement::Return(statement) => {
            push_line(
                out,
                level,
                &format!("Return [{}]", format_span(statement.span)),
            );
            if let Some(value) = &statement.value {
                render_expression(out, value, level + 1);
            }
        }
        Statement::Print(statement) => {
            push_line(
                out,
                level,
                &format!("Print [{}]", format_span(statement.span)),
            );
            for argument in &statement.arguments {
                render_expression(out, argument, level + 1);
            }
        }
        Statement::Expression(statement) => {
            push_line(
                out,
                level,
                &format!("Expression [{}]", format_span(statement.span)),
            );
            render_expression(out, &statement.expression, level + 1);
        }
    }
}

fn render_expression(out: &mut String, expression: &Expression, level: usize) {
    match expression {
        Expression::Identifier(identifier) => {
            render_identifier(out, "Identifier", identifier, level)
        }
        Expression::Nil(literal) => {
            push_line(out, level, &format!("Nil [{}]", format_span(literal.span)));
        }
        Expression::Logical(literal) => {
            push_line(
                out,
                level,
                &format!("Logical {} [{}]", literal.value, format_span(literal.span)),
            );
        }
        Expression::Integer(literal) => {
            push_line(
                out,
                level,
                &format!("Integer {} [{}]", literal.lexeme, format_span(literal.span)),
            );
        }
        Expression::Float(literal) => {
            push_line(
                out,
                level,
                &format!("Float {} [{}]", literal.lexeme, format_span(literal.span)),
            );
        }
        Expression::String(literal) => {
            push_line(
                out,
                level,
                &format!("String {} [{}]", literal.lexeme, format_span(literal.span)),
            );
        }
        Expression::Call(expression) => render_call_expression(out, expression, level),
        Expression::Assignment(expression) => {
            push_line(
                out,
                level,
                &format!("Assignment [{}]", format_span(expression.span)),
            );
            push_line(out, level + 1, "Target");
            render_expression(out, &expression.target, level + 2);
            push_line(out, level + 1, "Value");
            render_expression(out, &expression.value, level + 2);
        }
        Expression::Binary(expression) => render_binary_expression(out, expression, level),
        Expression::Unary(expression) => render_unary_expression(out, expression, level),
        Expression::Postfix(expression) => render_postfix_expression(out, expression, level),
    }
}

fn render_identifier(out: &mut String, label: &str, identifier: &Identifier, level: usize) {
    push_line(
        out,
        level,
        &format!(
            "{} {} [{}]",
            label,
            identifier.text,
            format_span(identifier.span)
        ),
    );
}

fn render_call_expression(out: &mut String, expression: &CallExpression, level: usize) {
    push_line(
        out,
        level,
        &format!("Call [{}]", format_span(expression.span)),
    );
    push_line(out, level + 1, "Callee");
    render_expression(out, &expression.callee, level + 2);
    if expression.arguments.is_empty() {
        push_line(out, level + 1, "Args []");
    } else {
        push_line(out, level + 1, "Args");
        for argument in &expression.arguments {
            render_expression(out, argument, level + 2);
        }
    }
}

fn render_binary_expression(out: &mut String, expression: &BinaryExpression, level: usize) {
    push_line(
        out,
        level,
        &format!(
            "Binary {} [{}]",
            render_binary_operator(expression.operator),
            format_span(expression.span)
        ),
    );
    render_expression(out, &expression.left, level + 1);
    render_expression(out, &expression.right, level + 1);
}

fn render_unary_expression(out: &mut String, expression: &UnaryExpression, level: usize) {
    push_line(
        out,
        level,
        &format!(
            "Unary {} [{}]",
            render_unary_operator(expression.operator),
            format_span(expression.span)
        ),
    );
    render_expression(out, &expression.operand, level + 1);
}

fn render_postfix_expression(out: &mut String, expression: &PostfixExpression, level: usize) {
    push_line(
        out,
        level,
        &format!(
            "Postfix {} [{}]",
            render_postfix_operator(expression.operator),
            format_span(expression.span)
        ),
    );
    render_expression(out, &expression.operand, level + 1);
}

fn render_routine_kind(kind: RoutineKind) -> &'static str {
    match kind {
        RoutineKind::Procedure => "Procedure",
        RoutineKind::Function => "Function",
    }
}

fn render_binary_operator(operator: BinaryOperator) -> &'static str {
    match operator {
        BinaryOperator::Or => "Or",
        BinaryOperator::And => "And",
        BinaryOperator::Equal => "Equal",
        BinaryOperator::ExactEqual => "ExactEqual",
        BinaryOperator::NotEqual => "NotEqual",
        BinaryOperator::Less => "Less",
        BinaryOperator::LessEqual => "LessEqual",
        BinaryOperator::Greater => "Greater",
        BinaryOperator::GreaterEqual => "GreaterEqual",
        BinaryOperator::Add => "Add",
        BinaryOperator::Subtract => "Subtract",
        BinaryOperator::Multiply => "Multiply",
        BinaryOperator::Divide => "Divide",
        BinaryOperator::Modulo => "Modulo",
        BinaryOperator::Power => "Power",
    }
}

fn render_unary_operator(operator: UnaryOperator) -> &'static str {
    match operator {
        UnaryOperator::Plus => "Plus",
        UnaryOperator::Minus => "Minus",
        UnaryOperator::Not => "Not",
    }
}

fn render_postfix_operator(operator: PostfixOperator) -> &'static str {
    match operator {
        PostfixOperator::Increment => "Increment",
        PostfixOperator::Decrement => "Decrement",
    }
}

fn format_span(span: Span) -> String {
    format!(
        "{}-{}",
        format_position(span.start),
        format_position(span.end)
    )
}

fn format_position(position: Position) -> String {
    format!("{}:{}", position.line, position.column)
}

fn push_indent(out: &mut String, level: usize) {
    for _ in 0..level {
        out.push_str("  ");
    }
}

fn push_line(out: &mut String, level: usize, line: &str) {
    push_indent(out, level);
    out.push_str(line);
    out.push('\n');
}
