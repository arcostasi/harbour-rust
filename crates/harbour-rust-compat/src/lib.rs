use harbour_rust_lexer::{LexedSource, lex};

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
