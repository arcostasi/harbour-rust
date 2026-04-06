use std::{
    collections::BTreeMap,
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    pub path: PathBuf,
    pub text: String,
}

impl SourceFile {
    pub fn new(path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            text: text.into(),
        }
    }

    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let canonical = fs::canonicalize(path.as_ref())?;
        let text = fs::read_to_string(&canonical)?;
        Ok(Self::new(canonical, text))
    }

    pub fn display_name(&self) -> String {
        display_path(&self.path)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocessError {
    pub message: String,
    pub path: PathBuf,
    pub location: SourceLocation,
}

impl PreprocessError {
    pub fn line(&self) -> usize {
        self.location.line
    }

    pub fn column(&self) -> usize {
        self.location.column
    }
}

impl fmt::Display for PreprocessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {}:{}:{}",
            self.message,
            display_path(&self.path),
            self.line(),
            self.column()
        )
    }
}

impl Error for PreprocessError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncludeDelimiter {
    Quotes,
    Angles,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeDirective {
    pub target: String,
    pub delimiter: IncludeDelimiter,
    pub source_path: PathBuf,
    pub location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefineDirective {
    pub name: String,
    pub normalized_name: String,
    pub parameters: Option<Vec<String>>,
    pub replacement: String,
    pub source_path: PathBuf,
    pub location: SourceLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleKind {
    Command,
    Translate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleDirective {
    pub kind: RuleKind,
    pub pattern: Vec<PatternPart>,
    pub replacement: Vec<ResultPart>,
    pub source_path: PathBuf,
    pub location: SourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternPart {
    Literal(String),
    Marker(PatternMarker),
    Optional(Vec<PatternPart>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternMarker {
    pub name: String,
    pub kind: MarkerKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkerKind {
    Regular,
    List,
    Restricted(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultPart {
    Literal(String),
    Marker(String),
    Stringify(String),
    Optional(Vec<ResultPart>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineOrigin {
    pub output_line: usize,
    pub source_path: PathBuf,
    pub source_line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PreprocessOutput {
    pub text: String,
    pub defines: Vec<DefineDirective>,
    pub rules: Vec<RuleDirective>,
    pub line_origins: Vec<LineOrigin>,
    pub errors: Vec<PreprocessError>,
}

impl PreprocessOutput {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeResolutionError {
    pub message: String,
}

impl IncludeResolutionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for IncludeResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for IncludeResolutionError {}

impl From<io::Error> for IncludeResolutionError {
    fn from(error: io::Error) -> Self {
        Self::new(error.to_string())
    }
}

pub trait IncludeResolver {
    fn resolve_include(
        &self,
        including_source: &SourceFile,
        directive: &IncludeDirective,
    ) -> Result<SourceFile, IncludeResolutionError>;
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FileSystemIncludeResolver {
    search_paths: Vec<PathBuf>,
}

impl FileSystemIncludeResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_search_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.search_paths.push(path.into());
        self
    }

    pub fn search_paths(&self) -> &[PathBuf] {
        &self.search_paths
    }
}

impl IncludeResolver for FileSystemIncludeResolver {
    fn resolve_include(
        &self,
        including_source: &SourceFile,
        directive: &IncludeDirective,
    ) -> Result<SourceFile, IncludeResolutionError> {
        let target_path = Path::new(&directive.target);
        if target_path.is_absolute() {
            return SourceFile::from_path(target_path).map_err(IncludeResolutionError::from);
        }

        let mut candidates = Vec::new();
        if directive.delimiter == IncludeDelimiter::Quotes {
            let base = including_source
                .path
                .parent()
                .unwrap_or_else(|| Path::new("."));
            candidates.push(base.join(&directive.target));
        }

        candidates.extend(
            self.search_paths
                .iter()
                .map(|search_path| search_path.join(&directive.target)),
        );

        for candidate in &candidates {
            if let Ok(source) = SourceFile::from_path(candidate) {
                return Ok(source);
            }
        }

        let rendered_candidates = candidates
            .iter()
            .map(|candidate| candidate.display().to_string())
            .collect::<Vec<_>>();
        Err(IncludeResolutionError::new(format!(
            "no include file found in [{}]",
            rendered_candidates.join(", ")
        )))
    }
}

#[derive(Debug, Clone)]
pub struct Preprocessor<R> {
    include_resolver: R,
}

impl<R> Preprocessor<R> {
    pub fn new(include_resolver: R) -> Self {
        Self { include_resolver }
    }
}

impl Default for Preprocessor<FileSystemIncludeResolver> {
    fn default() -> Self {
        Self::new(FileSystemIncludeResolver::default())
    }
}

impl<R: IncludeResolver> Preprocessor<R> {
    pub fn preprocess(&self, source: SourceFile) -> PreprocessOutput {
        let mut state = PreprocessState::new(&self.include_resolver);
        state.process_source(source);
        state.finish()
    }
}

pub fn preprocess(source: SourceFile) -> PreprocessOutput {
    Preprocessor::default().preprocess(source)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Directive {
    Define(DefineDirective),
    Include(IncludeDirective),
    Rule(RuleDirective),
}

struct PreprocessState<'a, R> {
    include_resolver: &'a R,
    output: String,
    defines: Vec<DefineDirective>,
    rules: Vec<RuleDirective>,
    line_origins: Vec<LineOrigin>,
    errors: Vec<PreprocessError>,
    include_stack: Vec<PathBuf>,
}

impl<'a, R: IncludeResolver> PreprocessState<'a, R> {
    fn new(include_resolver: &'a R) -> Self {
        Self {
            include_resolver,
            output: String::new(),
            defines: Vec::new(),
            rules: Vec::new(),
            line_origins: Vec::new(),
            errors: Vec::new(),
            include_stack: Vec::new(),
        }
    }

    fn finish(self) -> PreprocessOutput {
        PreprocessOutput {
            text: self.output,
            defines: self.defines,
            rules: self.rules,
            line_origins: self.line_origins,
            errors: self.errors,
        }
    }

    fn process_source(&mut self, source: SourceFile) {
        if self.include_stack.contains(&source.path) {
            self.errors.push(PreprocessError {
                message: format!("cyclic include detected for {}", display_path(&source.path)),
                path: source.path,
                location: SourceLocation { line: 1, column: 1 },
            });
            return;
        }

        self.include_stack.push(source.path.clone());

        let lines = split_lines(&source.text).collect::<Vec<_>>();
        let mut index = 0;
        while index < lines.len() {
            let raw_line = lines[index];
            let line_number = index + 1;

            if is_directive_start(raw_line) {
                let (logical_line, end_index) = collect_directive_line(&lines, index);
                match parse_directive(&source.path, &logical_line, line_number) {
                    None => {}
                    Some(Ok(Directive::Define(define))) => self.defines.push(define),
                    Some(Ok(Directive::Include(include))) => {
                        match self.include_resolver.resolve_include(&source, &include) {
                            Ok(included_source) => self.process_source(included_source),
                            Err(error) => self.errors.push(PreprocessError {
                                message: format!(
                                    "failed to resolve include {:?}: {}",
                                    include.target, error
                                ),
                                path: include.source_path,
                                location: include.location,
                            }),
                        }
                    }
                    Some(Ok(Directive::Rule(rule))) => self.rules.push(rule),
                    Some(Err(error)) => self.errors.push(error),
                }
                index = end_index + 1;
                continue;
            }

            let line_content = trim_line_ending(raw_line);
            let line_ending = &raw_line[line_content.len()..];
            let (expanded_line, mut errors) = preprocess_normal_line(
                line_content,
                &self.defines,
                &self.rules,
                &source.path,
                line_number,
            );
            self.errors.append(&mut errors);
            self.push_output_line(
                &source.path,
                line_number,
                &format!("{expanded_line}{line_ending}"),
            );
            index += 1;
        }

        self.include_stack.pop();
    }

    fn push_output_line(&mut self, source_path: &Path, source_line: usize, line: &str) {
        self.output.push_str(line);
        self.line_origins.push(LineOrigin {
            output_line: self.line_origins.len() + 1,
            source_path: source_path.to_path_buf(),
            source_line,
        });
    }
}

fn split_lines(text: &str) -> impl Iterator<Item = &str> {
    text.split_inclusive('\n')
}

fn parse_directive(
    path: &Path,
    line: &str,
    line_number: usize,
) -> Option<Result<Directive, PreprocessError>> {
    let leading_whitespace = line
        .chars()
        .take_while(|ch| ch.is_whitespace() && *ch != '\n' && *ch != '\r')
        .count();
    let trimmed =
        line.trim_start_matches(|ch: char| ch.is_whitespace() && ch != '\n' && ch != '\r');
    if !trimmed.starts_with('#') {
        return None;
    }

    let after_hash = &trimmed[1..];
    let keyword_length = after_hash
        .chars()
        .take_while(|ch| ch.is_ascii_alphabetic())
        .count();

    if keyword_length == 0 {
        return Some(Err(directive_error(
            path,
            line_number,
            leading_whitespace + 1,
            "expected preprocessor directive name after '#'",
        )));
    }

    let keyword = after_hash[..keyword_length].to_ascii_lowercase();
    let rest = &after_hash[keyword_length..];
    let directive_column = leading_whitespace + 1;

    Some(match keyword.as_str() {
        "define" => parse_define(path, line_number, directive_column, rest).map(Directive::Define),
        "include" => {
            parse_include(path, line_number, directive_column, rest).map(Directive::Include)
        }
        "command" | "xcommand" => {
            parse_rule(path, line_number, directive_column, rest, RuleKind::Command)
                .map(Directive::Rule)
        }
        "translate" | "xtranslate" => parse_rule(
            path,
            line_number,
            directive_column,
            rest,
            RuleKind::Translate,
        )
        .map(Directive::Rule),
        _ => Err(directive_error(
            path,
            line_number,
            directive_column,
            format!("unsupported preprocessor directive '#{}'", keyword),
        )),
    })
}

fn parse_rule(
    path: &Path,
    line_number: usize,
    column: usize,
    rest: &str,
    kind: RuleKind,
) -> Result<RuleDirective, PreprocessError> {
    let rest = trim_line_ending(rest);
    let Some((pattern_text, replacement_text)) = rest.split_once("=>") else {
        return Err(directive_error(
            path,
            line_number,
            column,
            "expected '=>' in preprocessor rule",
        ));
    };

    let pattern = parse_pattern(pattern_text.trim(), path, line_number, column)?;
    if pattern.is_empty() {
        return Err(directive_error(
            path,
            line_number,
            column,
            "expected non-empty preprocessor rule pattern",
        ));
    }

    let replacement = parse_result(
        trim_inline_whitespace(replacement_text),
        path,
        line_number,
        column,
    )?;

    Ok(RuleDirective {
        kind,
        pattern,
        replacement,
        source_path: path.to_path_buf(),
        location: SourceLocation {
            line: line_number,
            column,
        },
    })
}

fn parse_define(
    path: &Path,
    line_number: usize,
    column: usize,
    rest: &str,
) -> Result<DefineDirective, PreprocessError> {
    let rest = trim_inline_whitespace(rest);
    if rest.is_empty() {
        return Err(directive_error(
            path,
            line_number,
            column,
            "expected define name after '#define'",
        ));
    }

    let name_length = identifier_length(rest);
    if name_length == 0 {
        return Err(directive_error(
            path,
            line_number,
            column,
            "expected valid define name after '#define'",
        ));
    }

    let name = rest[..name_length].to_owned();
    let mut tail = &rest[name_length..];
    let mut parameters = None;

    if tail.starts_with('(') {
        let Some(close_index) = tail.find(')') else {
            return Err(directive_error(
                path,
                line_number,
                column,
                format!("unterminated parameter list in define '{}'", name),
            ));
        };

        let parameter_text = &tail[1..close_index];
        let parsed_parameters =
            parse_define_parameters(path, line_number, column, &name, parameter_text)?;
        parameters = Some(parsed_parameters);
        tail = &tail[close_index + 1..];
    } else if tail
        .chars()
        .next()
        .is_some_and(|ch| !ch.is_whitespace() && ch != '\r' && ch != '\n')
    {
        return Err(directive_error(
            path,
            line_number,
            column,
            format!("expected whitespace after define name '{}'", name),
        ));
    }

    let replacement = trim_line_ending(trim_inline_whitespace(tail)).to_owned();
    Ok(DefineDirective {
        normalized_name: name.to_ascii_uppercase(),
        name,
        parameters,
        replacement,
        source_path: path.to_path_buf(),
        location: SourceLocation {
            line: line_number,
            column,
        },
    })
}

fn parse_define_parameters(
    path: &Path,
    line_number: usize,
    column: usize,
    define_name: &str,
    parameter_text: &str,
) -> Result<Vec<String>, PreprocessError> {
    if parameter_text.trim().is_empty() {
        return Ok(Vec::new());
    }

    parameter_text
        .split(',')
        .map(|entry| {
            let parameter = entry.trim();
            if identifier_length(parameter) != parameter.len() {
                Err(directive_error(
                    path,
                    line_number,
                    column,
                    format!("expected valid parameter name in define '{}'", define_name),
                ))
            } else {
                Ok(parameter.to_owned())
            }
        })
        .collect()
}

fn parse_include(
    path: &Path,
    line_number: usize,
    column: usize,
    rest: &str,
) -> Result<IncludeDirective, PreprocessError> {
    let rest = trim_inline_whitespace(trim_line_ending(rest));
    if rest.is_empty() {
        return Err(directive_error(
            path,
            line_number,
            column,
            "expected include target after '#include'",
        ));
    }

    let (delimiter, closing) = match rest.chars().next().unwrap_or_default() {
        '"' => (IncludeDelimiter::Quotes, '"'),
        '<' => (IncludeDelimiter::Angles, '>'),
        _ => {
            return Err(directive_error(
                path,
                line_number,
                column,
                "expected quoted or angle-bracket include target",
            ));
        }
    };

    let body = &rest[1..];
    let Some(closing_index) = body.find(closing) else {
        return Err(directive_error(
            path,
            line_number,
            column,
            "unterminated include target",
        ));
    };

    let target = body[..closing_index].to_owned();
    let trailing = trim_inline_whitespace(&body[closing_index + 1..]);
    if !trailing.is_empty() {
        return Err(directive_error(
            path,
            line_number,
            column,
            "unexpected trailing tokens after include target",
        ));
    }

    Ok(IncludeDirective {
        target,
        delimiter,
        source_path: path.to_path_buf(),
        location: SourceLocation {
            line: line_number,
            column,
        },
    })
}

fn parse_pattern(
    text: &str,
    path: &Path,
    line_number: usize,
    column: usize,
) -> Result<Vec<PatternPart>, PreprocessError> {
    parse_pattern_until(text, path, line_number, column, None).and_then(|(parts, consumed)| {
        if consumed != text.len() {
            Err(directive_error(
                path,
                line_number,
                column,
                "unexpected trailing tokens in pattern",
            ))
        } else {
            Ok(parts)
        }
    })
}

fn parse_pattern_until(
    text: &str,
    path: &Path,
    line_number: usize,
    column: usize,
    until: Option<char>,
) -> Result<(Vec<PatternPart>, usize), PreprocessError> {
    let mut parts = Vec::new();
    let mut cursor = 0;

    while cursor < text.len() {
        cursor = skip_inline_spaces(text, cursor);
        if cursor >= text.len() {
            break;
        }

        let ch = char_at(text, cursor);
        if Some(ch) == until {
            return Ok((parts, cursor + ch.len_utf8()));
        }

        match ch {
            '[' => {
                let (nested, consumed) =
                    parse_pattern_until(&text[cursor + 1..], path, line_number, column, Some(']'))?;
                parts.push(PatternPart::Optional(nested));
                cursor += 1 + consumed;
            }
            '<' => {
                let Some(close_offset) = text[cursor + 1..].find('>') else {
                    return Err(directive_error(
                        path,
                        line_number,
                        column,
                        "unterminated rule marker in pattern",
                    ));
                };
                let marker_text = &text[cursor + 1..cursor + 1 + close_offset];
                parts.push(PatternPart::Marker(parse_marker(
                    marker_text,
                    path,
                    line_number,
                    column,
                )?));
                cursor += close_offset + 2;
            }
            _ => {
                let (literal, next_cursor) = parse_literal_token(text, cursor);
                parts.push(PatternPart::Literal(literal));
                cursor = next_cursor;
            }
        }
    }

    if until.is_some() {
        Err(directive_error(
            path,
            line_number,
            column,
            "unterminated optional clause in pattern",
        ))
    } else {
        Ok((parts, cursor))
    }
}

fn parse_marker(
    text: &str,
    path: &Path,
    line_number: usize,
    column: usize,
) -> Result<PatternMarker, PreprocessError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(directive_error(
            path,
            line_number,
            column,
            "expected marker name inside '<...>'",
        ));
    }

    if let Some(base_name) = text.strip_suffix(",...") {
        return Ok(PatternMarker {
            name: validate_marker_name(base_name, path, line_number, column)?,
            kind: MarkerKind::List,
        });
    }

    if let Some((name, restriction_text)) = text.split_once(':') {
        let name = validate_marker_name(name, path, line_number, column)?;
        let allowed = restriction_text
            .split(',')
            .map(str::trim)
            .filter(|entry| !entry.is_empty())
            .map(|entry| entry.to_owned())
            .collect::<Vec<_>>();
        if allowed.is_empty() {
            return Err(directive_error(
                path,
                line_number,
                column,
                format!("expected at least one restriction for marker '{name}'"),
            ));
        }
        return Ok(PatternMarker {
            name,
            kind: MarkerKind::Restricted(allowed),
        });
    }

    Ok(PatternMarker {
        name: validate_marker_name(text, path, line_number, column)?,
        kind: MarkerKind::Regular,
    })
}

fn validate_marker_name(
    text: &str,
    path: &Path,
    line_number: usize,
    column: usize,
) -> Result<String, PreprocessError> {
    let name = text.trim();
    if identifier_length(name) != name.len() {
        return Err(directive_error(
            path,
            line_number,
            column,
            format!("expected valid marker name, got '{name}'"),
        ));
    }
    Ok(name.to_owned())
}

fn parse_result(
    text: &str,
    path: &Path,
    line_number: usize,
    column: usize,
) -> Result<Vec<ResultPart>, PreprocessError> {
    parse_result_until(text, path, line_number, column, None).and_then(|(parts, consumed)| {
        if consumed != text.len() {
            Err(directive_error(
                path,
                line_number,
                column,
                "unexpected trailing tokens in replacement",
            ))
        } else {
            Ok(parts)
        }
    })
}

fn parse_result_until(
    text: &str,
    path: &Path,
    line_number: usize,
    column: usize,
    until: Option<char>,
) -> Result<(Vec<ResultPart>, usize), PreprocessError> {
    let mut parts = Vec::new();
    let mut cursor = 0;
    let mut literal = String::new();

    while cursor < text.len() {
        let ch = char_at(text, cursor);
        if Some(ch) == until {
            if !literal.is_empty() {
                parts.push(ResultPart::Literal(literal));
            }
            return Ok((parts, cursor + ch.len_utf8()));
        }

        if ch == '\\' {
            let next_cursor = cursor + ch.len_utf8();
            if next_cursor < text.len() {
                let next = char_at(text, next_cursor);
                if is_result_escape_target(next) {
                    literal.push(next);
                    cursor = next_cursor + next.len_utf8();
                    continue;
                }
            }
        }

        if ch == '[' {
            if !literal.is_empty() {
                parts.push(ResultPart::Literal(std::mem::take(&mut literal)));
            }
            let (nested, consumed) =
                parse_result_until(&text[cursor + 1..], path, line_number, column, Some(']'))?;
            parts.push(ResultPart::Optional(nested));
            cursor += 1 + consumed;
            continue;
        }

        if ch == '#' && text[cursor + 1..].starts_with('<') {
            if !literal.is_empty() {
                parts.push(ResultPart::Literal(std::mem::take(&mut literal)));
            }
            let Some(close_offset) = text[cursor + 2..].find('>') else {
                return Err(directive_error(
                    path,
                    line_number,
                    column,
                    "unterminated stringify marker in replacement",
                ));
            };
            let marker = &text[cursor + 2..cursor + 2 + close_offset];
            parts.push(ResultPart::Stringify(validate_marker_name(
                marker,
                path,
                line_number,
                column,
            )?));
            cursor += close_offset + 3;
            continue;
        }

        if ch == '<' {
            if !literal.is_empty() {
                parts.push(ResultPart::Literal(std::mem::take(&mut literal)));
            }
            let Some(close_offset) = text[cursor + 1..].find('>') else {
                return Err(directive_error(
                    path,
                    line_number,
                    column,
                    "unterminated marker in replacement",
                ));
            };
            let marker = &text[cursor + 1..cursor + 1 + close_offset];
            parts.push(ResultPart::Marker(validate_marker_name(
                marker,
                path,
                line_number,
                column,
            )?));
            cursor += close_offset + 2;
            continue;
        }

        literal.push(ch);
        cursor += ch.len_utf8();
    }

    if until.is_some() {
        Err(directive_error(
            path,
            line_number,
            column,
            "unterminated optional clause in replacement",
        ))
    } else {
        if !literal.is_empty() {
            parts.push(ResultPart::Literal(literal));
        }
        Ok((parts, cursor))
    }
}

fn skip_inline_spaces(text: &str, mut cursor: usize) -> usize {
    while cursor < text.len() {
        let ch = char_at(text, cursor);
        if !matches!(ch, ' ' | '\t') {
            break;
        }
        cursor += ch.len_utf8();
    }
    cursor
}

fn is_result_escape_target(ch: char) -> bool {
    matches!(ch, '[' | ']' | '<' | '>' | '#' | '\\')
}

fn parse_literal_token(text: &str, start: usize) -> (String, usize) {
    let ch = char_at(text, start);
    if is_string_delimiter(ch) {
        let mut token = String::new();
        let end = copy_string_literal(text, start, ch, &mut token);
        return (token, end);
    }

    if is_symbol_like(ch) {
        return (ch.to_string(), start + ch.len_utf8());
    }

    let mut cursor = start;
    let mut token = String::new();
    while cursor < text.len() {
        let current = char_at(text, cursor);
        if matches!(current, ' ' | '\t' | '[' | ']' | '<' | '>') || is_symbol_like(current) {
            break;
        }
        token.push(current);
        cursor += current.len_utf8();
    }
    (token, cursor)
}

fn identifier_length(text: &str) -> usize {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return 0;
    };

    if !is_identifier_start(first) {
        return 0;
    }

    first.len_utf8()
        + chars
            .take_while(|ch| is_identifier_continue(*ch))
            .map(char::len_utf8)
            .sum::<usize>()
}

fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

fn trim_inline_whitespace(text: &str) -> &str {
    text.trim_start_matches([' ', '\t'])
}

fn trim_line_ending(text: &str) -> &str {
    text.trim_end_matches(['\r', '\n'])
}

fn directive_error(
    path: &Path,
    line: usize,
    column: usize,
    message: impl Into<String>,
) -> PreprocessError {
    PreprocessError {
        message: message.into(),
        path: path.to_path_buf(),
        location: SourceLocation { line, column },
    }
}

fn display_path(path: &Path) -> String {
    if path.as_os_str().is_empty() {
        "<memory>".to_owned()
    } else {
        path.display().to_string()
    }
}

fn is_directive_start(line: &str) -> bool {
    trim_inline_whitespace(trim_line_ending(line)).starts_with('#')
}

fn collect_directive_line(lines: &[&str], start_index: usize) -> (String, usize) {
    let mut logical = trim_line_ending(lines[start_index]).to_owned();
    let mut index = start_index;

    while trim_inline_whitespace(logical.trim_end()).ends_with(';') && index + 1 < lines.len() {
        logical = trim_inline_whitespace(logical.trim_end())
            .trim_end_matches(';')
            .trim_end()
            .to_owned();
        index += 1;
        logical.push(' ');
        logical.push_str(trim_inline_whitespace(trim_line_ending(lines[index])));
    }

    (logical, index)
}

fn preprocess_normal_line(
    line: &str,
    defines: &[DefineDirective],
    rules: &[RuleDirective],
    path: &Path,
    line_number: usize,
) -> (String, Vec<PreprocessError>) {
    let (defined_once, mut errors) = expand_object_like_defines(line, defines, path, line_number);
    let (rule_expanded, mut rule_errors) =
        expand_rule_directives(&defined_once, rules, path, line_number);
    errors.append(&mut rule_errors);
    if rule_expanded == defined_once {
        return (rule_expanded, errors);
    }
    let (defined_twice, mut final_errors) =
        expand_object_like_defines(&rule_expanded, defines, path, line_number);
    errors.append(&mut final_errors);
    (defined_twice, errors)
}

fn expand_rule_directives(
    line: &str,
    rules: &[RuleDirective],
    path: &Path,
    line_number: usize,
) -> (String, Vec<PreprocessError>) {
    let mut current = line.to_owned();
    let mut errors = Vec::new();
    let mut passes = 0usize;

    while passes < 16 {
        passes += 1;
        let mut changed = false;

        if let Some(expanded) = apply_command_rules(&current, rules)
            && expanded != current
        {
            current = expanded;
            changed = true;
        }

        let (translated, translated_changed) = apply_translate_rules(&current, rules);
        if translated_changed {
            current = translated;
            changed = true;
        }

        if !changed {
            break;
        }
    }

    if passes == 16 {
        errors.push(directive_error(
            path,
            line_number,
            1,
            "rule expansion reached iteration limit",
        ));
    }

    (current, errors)
}

fn apply_command_rules(line: &str, rules: &[RuleDirective]) -> Option<String> {
    let content_start = line
        .char_indices()
        .find_map(|(index, ch)| (!matches!(ch, ' ' | '\t')).then_some(index))
        .unwrap_or(0);
    let content_end = line.trim_end_matches([' ', '\t']).len();
    let leading = &line[..content_start];
    let trailing = &line[content_end..];
    let content = &line[content_start..content_end];
    let tokens = tokenize_source_line(content);
    if tokens.is_empty() {
        return None;
    }

    for rule in rules
        .iter()
        .rev()
        .filter(|rule| rule.kind == RuleKind::Command)
    {
        if let Some(captures) = match_pattern(&rule.pattern, &tokens, content, 0, true) {
            let mut rendered = leading.to_owned();
            rendered.push_str(&render_result(&rule.replacement, &captures));
            rendered.push_str(trailing);
            return Some(rendered);
        }
    }

    None
}

fn apply_translate_rules(line: &str, rules: &[RuleDirective]) -> (String, bool) {
    let mut current = line.to_owned();
    let mut changed = false;

    loop {
        let tokens = tokenize_source_line(&current);
        let mut replaced = None;

        'outer: for start in 0..tokens.len() {
            for rule in rules
                .iter()
                .rev()
                .filter(|rule| rule.kind == RuleKind::Translate)
            {
                if let Some((end, captures)) =
                    match_pattern_with_end(&rule.pattern, &tokens, &current, start)
                {
                    let start_offset = tokens[start].start;
                    let end_offset = tokens[end - 1].end;
                    let mut next = String::new();
                    next.push_str(&current[..start_offset]);
                    next.push_str(&render_result(&rule.replacement, &captures));
                    next.push_str(&current[end_offset..]);
                    replaced = Some(next);
                    break 'outer;
                }
            }
        }

        if let Some(next) = replaced {
            changed = true;
            current = next;
            continue;
        }

        break;
    }

    (current, changed)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceToken {
    text: String,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct MatchCaptures {
    values: BTreeMap<String, CaptureValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CaptureValue {
    raw: String,
    list_items: Option<Vec<String>>,
}

fn tokenize_source_line(text: &str) -> Vec<SourceToken> {
    let mut tokens = Vec::new();
    let mut cursor = 0;

    while cursor < text.len() {
        let ch = char_at(text, cursor);
        if matches!(ch, ' ' | '\t' | '\r' | '\n') {
            cursor += ch.len_utf8();
            continue;
        }

        if is_string_delimiter(ch) {
            let start = cursor;
            let mut token = String::new();
            cursor = copy_string_literal(text, cursor, ch, &mut token);
            tokens.push(SourceToken {
                text: token,
                start,
                end: cursor,
            });
            continue;
        }

        let start = cursor;
        if is_symbol_like(ch) {
            cursor += ch.len_utf8();
        } else {
            while cursor < text.len() {
                let current = char_at(text, cursor);
                if matches!(current, ' ' | '\t' | '\r' | '\n') || is_symbol_like(current) {
                    break;
                }
                cursor += current.len_utf8();
            }
        }
        tokens.push(SourceToken {
            text: text[start..cursor].to_owned(),
            start,
            end: cursor,
        });
    }

    tokens
}

fn is_symbol_like(ch: char) -> bool {
    matches!(
        ch,
        '(' | ')'
            | '['
            | ']'
            | '{'
            | '}'
            | ','
            | ';'
            | ':'
            | '+'
            | '-'
            | '*'
            | '/'
            | '='
            | '<'
            | '>'
            | '#'
            | '&'
    )
}

fn match_pattern(
    pattern: &[PatternPart],
    tokens: &[SourceToken],
    source: &str,
    start_index: usize,
    require_full: bool,
) -> Option<MatchCaptures> {
    let (end, captures) = match_pattern_recursive(
        pattern,
        0,
        tokens,
        source,
        start_index,
        require_full.then_some(tokens.len()),
        MatchCaptures::default(),
    )?;
    if require_full && end != tokens.len() {
        None
    } else {
        Some(captures)
    }
}

fn match_pattern_with_end(
    pattern: &[PatternPart],
    tokens: &[SourceToken],
    source: &str,
    start_index: usize,
) -> Option<(usize, MatchCaptures)> {
    match_pattern_recursive(
        pattern,
        0,
        tokens,
        source,
        start_index,
        None,
        MatchCaptures::default(),
    )
}

fn match_pattern_recursive(
    pattern: &[PatternPart],
    pattern_index: usize,
    tokens: &[SourceToken],
    source: &str,
    token_index: usize,
    required_end: Option<usize>,
    captures: MatchCaptures,
) -> Option<(usize, MatchCaptures)> {
    if pattern_index == pattern.len() {
        if let Some(required_end) = required_end
            && token_index != required_end
        {
            return None;
        }
        return Some((token_index, captures));
    }

    if matches!(pattern[pattern_index], PatternPart::Optional(_)) {
        let group_end = optional_group_end(pattern, pattern_index);
        let optional_indices = (pattern_index..group_end).collect::<Vec<_>>();
        let matcher = OptionalGroupMatcher {
            pattern,
            tokens,
            source,
            rest_index: group_end,
            required_end,
        };
        return match_optional_group(&matcher, &optional_indices, token_index, captures);
    }

    match &pattern[pattern_index] {
        PatternPart::Literal(literal) => {
            let token = tokens.get(token_index)?;
            if token_matches_literal(token, literal) {
                match_pattern_recursive(
                    pattern,
                    pattern_index + 1,
                    tokens,
                    source,
                    token_index + 1,
                    required_end,
                    captures,
                )
            } else {
                None
            }
        }
        PatternPart::Optional(_) => unreachable!("optional groups are handled before dispatch"),
        PatternPart::Marker(marker) => match marker.kind {
            MarkerKind::Restricted(ref allowed) => {
                let token = tokens.get(token_index)?;
                if !allowed
                    .iter()
                    .any(|entry| token_matches_text(&token.text, entry))
                {
                    return None;
                }
                let capture = CaptureValue {
                    raw: token.text.clone(),
                    list_items: None,
                };
                let captures = merge_capture(captures, &marker.name, capture)?;
                match_pattern_recursive(
                    pattern,
                    pattern_index + 1,
                    tokens,
                    source,
                    token_index + 1,
                    required_end,
                    captures,
                )
            }
            MarkerKind::Regular | MarkerKind::List => {
                let minimum_end = token_index + 1;
                for end in (minimum_end..=tokens.len()).rev() {
                    let capture =
                        match build_capture(&marker.kind, tokens, source, token_index, end) {
                            Some(capture) => capture,
                            None => continue,
                        };
                    let next_captures = merge_capture(captures.clone(), &marker.name, capture)?;
                    if let Some(matched) = match_pattern_recursive(
                        pattern,
                        pattern_index + 1,
                        tokens,
                        source,
                        end,
                        required_end,
                        next_captures,
                    ) {
                        return Some(matched);
                    }
                }
                None
            }
        },
    }
}

fn optional_group_end(pattern: &[PatternPart], start: usize) -> usize {
    let mut index = start;
    while index < pattern.len() {
        if !matches!(pattern[index], PatternPart::Optional(_)) {
            break;
        }
        index += 1;
    }
    index
}

struct OptionalGroupMatcher<'a> {
    pattern: &'a [PatternPart],
    tokens: &'a [SourceToken],
    source: &'a str,
    rest_index: usize,
    required_end: Option<usize>,
}

fn match_optional_group(
    matcher: &OptionalGroupMatcher<'_>,
    optional_indices: &[usize],
    token_index: usize,
    captures: MatchCaptures,
) -> Option<(usize, MatchCaptures)> {
    let mut ordered_indices = optional_indices
        .iter()
        .copied()
        .map(|index| (index, optional_clause_priority(&matcher.pattern[index])))
        .collect::<Vec<_>>();
    ordered_indices.sort_by_key(|(_, priority)| *priority);

    for (optional_index, _) in ordered_indices {
        let position = optional_indices
            .iter()
            .position(|index| *index == optional_index)
            .expect("optional index is always part of the group");
        let PatternPart::Optional(optional) = &matcher.pattern[optional_index] else {
            continue;
        };
        for (next_index, next_captures) in collect_optional_matches(
            optional,
            matcher.tokens,
            matcher.source,
            token_index,
            &captures,
        ) {
            let mut remaining = optional_indices.to_vec();
            remaining.remove(position);
            if let Some(matched) =
                match_optional_group(matcher, &remaining, next_index, next_captures)
            {
                return Some(matched);
            }
        }
    }

    match_pattern_recursive(
        matcher.pattern,
        matcher.rest_index,
        matcher.tokens,
        matcher.source,
        token_index,
        matcher.required_end,
        captures,
    )
}

fn optional_clause_priority(part: &PatternPart) -> usize {
    match part {
        PatternPart::Optional(parts) => optional_parts_priority(parts),
        PatternPart::Literal(_) => 0,
        PatternPart::Marker(PatternMarker {
            kind: MarkerKind::Restricted(_),
            ..
        }) => 1,
        PatternPart::Marker(_) => 2,
    }
}

fn optional_parts_priority(parts: &[PatternPart]) -> usize {
    parts.first().map(optional_clause_priority).unwrap_or(3)
}

fn collect_optional_matches(
    optional: &[PatternPart],
    tokens: &[SourceToken],
    source: &str,
    token_index: usize,
    captures: &MatchCaptures,
) -> Vec<(usize, MatchCaptures)> {
    let mut matches = Vec::new();

    for end in token_index + 1..=tokens.len() {
        if let Some((next_index, next_captures)) = match_pattern_recursive(
            optional,
            0,
            tokens,
            source,
            token_index,
            Some(end),
            captures.clone(),
        ) {
            matches.push((next_index, next_captures));
        }
    }

    matches
}

fn token_matches_literal(token: &SourceToken, literal: &str) -> bool {
    token_matches_text(&token.text, literal)
}

fn token_matches_text(token_text: &str, literal: &str) -> bool {
    if token_text
        .chars()
        .all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
        && literal
            .chars()
            .all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
    {
        token_text.eq_ignore_ascii_case(literal)
    } else {
        token_text == literal
    }
}

fn build_capture(
    kind: &MarkerKind,
    tokens: &[SourceToken],
    source: &str,
    start: usize,
    end: usize,
) -> Option<CaptureValue> {
    if start >= end {
        return None;
    }
    let raw = source[tokens[start].start..tokens[end - 1].end].to_owned();
    match kind {
        MarkerKind::Regular => Some(CaptureValue {
            raw,
            list_items: None,
        }),
        MarkerKind::List => {
            split_list_capture(tokens, source, start, end).map(|entries| CaptureValue {
                raw,
                list_items: Some(entries),
            })
        }
        MarkerKind::Restricted(_) => None,
    }
}

fn split_list_capture(
    tokens: &[SourceToken],
    source: &str,
    start: usize,
    end: usize,
) -> Option<Vec<String>> {
    let mut entries = Vec::new();
    let mut depth = 0i32;
    let mut entry_start = start;

    for index in start..end {
        match tokens[index].text.as_str() {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => depth -= 1,
            "," if depth == 0 => {
                if entry_start == index {
                    return None;
                }
                entries.push(source[tokens[entry_start].start..tokens[index - 1].end].to_owned());
                entry_start = index + 1;
            }
            _ => {}
        }
    }

    if entry_start >= end {
        return None;
    }
    entries.push(source[tokens[entry_start].start..tokens[end - 1].end].to_owned());
    Some(entries)
}

fn merge_capture(
    mut captures: MatchCaptures,
    name: &str,
    value: CaptureValue,
) -> Option<MatchCaptures> {
    if let Some(existing) = captures.values.get(name) {
        if existing != &value {
            return None;
        }
        return Some(captures);
    }
    captures.values.insert(name.to_owned(), value);
    Some(captures)
}

fn render_result(parts: &[ResultPart], captures: &MatchCaptures) -> String {
    let mut output = String::new();
    for part in parts {
        render_result_part(part, captures, &mut output);
    }
    output
}

fn render_result_part(part: &ResultPart, captures: &MatchCaptures, output: &mut String) {
    match part {
        ResultPart::Literal(text) => output.push_str(text),
        ResultPart::Marker(name) => {
            if let Some(value) = captures.values.get(name) {
                output.push_str(&value.raw);
            }
        }
        ResultPart::Stringify(name) => {
            if let Some(value) = captures.values.get(name) {
                output.push('"');
                output.push_str(&escape_string_literal(&value.raw));
                output.push('"');
            }
        }
        ResultPart::Optional(parts) => {
            if result_parts_have_value(parts, captures) {
                for nested in parts {
                    render_result_part(nested, captures, output);
                }
            }
        }
    }
}

fn result_parts_have_value(parts: &[ResultPart], captures: &MatchCaptures) -> bool {
    parts.iter().any(|part| match part {
        ResultPart::Literal(_) => false,
        ResultPart::Marker(name) | ResultPart::Stringify(name) => {
            captures.values.contains_key(name)
        }
        ResultPart::Optional(parts) => result_parts_have_value(parts, captures),
    })
}

fn escape_string_literal(text: &str) -> String {
    text.chars().flat_map(char::escape_default).collect()
}

fn expand_object_like_defines(
    line: &str,
    defines: &[DefineDirective],
    path: &Path,
    line_number: usize,
) -> (String, Vec<PreprocessError>) {
    let object_like_defines = defines
        .iter()
        .filter(|define| define.parameters.is_none())
        .map(|define| (define.normalized_name.clone(), define))
        .collect::<BTreeMap<_, _>>();

    if object_like_defines.is_empty() {
        return (line.to_owned(), Vec::new());
    }

    let mut errors = Vec::new();
    let output = expand_text_segment(
        line,
        &object_like_defines,
        path,
        line_number,
        1,
        &mut Vec::new(),
        &mut errors,
    );

    (output, errors)
}

fn expand_text_segment(
    text: &str,
    defines: &BTreeMap<String, &DefineDirective>,
    path: &Path,
    line_number: usize,
    column_base: usize,
    stack: &mut Vec<String>,
    errors: &mut Vec<PreprocessError>,
) -> String {
    let mut output = String::with_capacity(text.len());
    let mut cursor = 0;

    while cursor < text.len() {
        let ch = char_at(text, cursor);
        if is_string_delimiter(ch) {
            cursor = copy_string_literal(text, cursor, ch, &mut output);
            continue;
        }

        if starts_line_comment(text, cursor) {
            output.push_str(&text[cursor..]);
            break;
        }

        if is_identifier_start(ch) {
            let end = advance_identifier(text, cursor);
            let identifier = &text[cursor..end];
            let normalized = identifier.to_ascii_uppercase();
            let identifier_column = column_base + text[..cursor].chars().count();

            if let Some(define) = defines.get(&normalized) {
                if let Some(position) = stack.iter().position(|name| name == &normalized) {
                    let mut cycle = stack[position..].to_vec();
                    cycle.push(normalized.clone());
                    errors.push(directive_error(
                        path,
                        line_number,
                        identifier_column,
                        format!("cyclic define expansion detected: {}", cycle.join(" -> ")),
                    ));
                    output.push_str(identifier);
                } else {
                    stack.push(normalized);
                    let expanded = expand_text_segment(
                        &define.replacement,
                        defines,
                        path,
                        line_number,
                        identifier_column,
                        stack,
                        errors,
                    );
                    stack.pop();
                    output.push_str(&expanded);
                }
            } else {
                output.push_str(identifier);
            }

            cursor = end;
            continue;
        }

        output.push(ch);
        cursor += ch.len_utf8();
    }

    output
}

fn char_at(text: &str, offset: usize) -> char {
    text[offset..].chars().next().expect("valid utf-8 boundary")
}

fn advance_identifier(text: &str, start: usize) -> usize {
    let mut end = start;
    for ch in text[start..].chars() {
        if !is_identifier_continue(ch) {
            break;
        }
        end += ch.len_utf8();
    }
    end
}

fn is_string_delimiter(ch: char) -> bool {
    matches!(ch, '"' | '\'' | '`')
}

fn copy_string_literal(text: &str, start: usize, delimiter: char, output: &mut String) -> usize {
    let mut cursor = start;
    let mut escaped = false;
    let mut saw_opening_delimiter = false;

    while cursor < text.len() {
        let ch = char_at(text, cursor);
        output.push(ch);
        cursor += ch.len_utf8();

        if !saw_opening_delimiter {
            saw_opening_delimiter = true;
            continue;
        }

        if escaped {
            escaped = false;
            continue;
        }

        if ch == '\\' {
            escaped = true;
            continue;
        }

        if ch == delimiter {
            break;
        }
    }

    cursor
}

fn starts_line_comment(text: &str, start: usize) -> bool {
    let tail = &text[start..];
    tail.starts_with("//") || tail.starts_with("&&")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Debug, Default)]
    struct MapIncludeResolver {
        files: BTreeMap<PathBuf, SourceFile>,
    }

    impl MapIncludeResolver {
        fn with_file(mut self, path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
            let path = path.into();
            self.files
                .insert(path.clone(), SourceFile::new(path, text.into()));
            self
        }
    }

    impl IncludeResolver for MapIncludeResolver {
        fn resolve_include(
            &self,
            including_source: &SourceFile,
            directive: &IncludeDirective,
        ) -> Result<SourceFile, IncludeResolutionError> {
            let base = including_source
                .path
                .parent()
                .unwrap_or_else(|| Path::new("."));
            let path = base.join(&directive.target);
            self.files.get(&path).cloned().ok_or_else(|| {
                IncludeResolutionError::new(format!("missing test include {}", path.display()))
            })
        }
    }

    #[test]
    fn collects_simple_defines_without_emitting_directive_lines() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#define APP_NAME \"harbour-rust\"\nPROCEDURE Main()\nRETURN\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert_eq!(output.text, "PROCEDURE Main()\nRETURN\n");
        assert!(output.errors.is_empty());
        assert_eq!(output.defines.len(), 1);
        assert_eq!(output.defines[0].name, "APP_NAME");
        assert_eq!(output.defines[0].normalized_name, "APP_NAME");
        assert_eq!(output.defines[0].replacement, "\"harbour-rust\"");
        assert_eq!(output.line_origins.len(), 2);
        assert_eq!(output.line_origins[0].source_line, 2);
        assert_eq!(output.line_origins[1].source_line, 3);
    }

    #[test]
    fn resolves_includes_and_tracks_line_origins() {
        let resolver = MapIncludeResolver::default().with_file(
            PathBuf::from("fixtures/shared.ch"),
            "#define GREETING \"hello\"\n? GREETING\n",
        );
        let source = SourceFile::new(
            PathBuf::from("fixtures/main.prg"),
            "#define APP_NAME \"harbour-rust\"\n#include \"shared.ch\"\nPROCEDURE Main()\nRETURN\n",
        );

        let output = Preprocessor::new(resolver).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "? \"hello\"\nPROCEDURE Main()\nRETURN\n");
        assert_eq!(output.defines.len(), 2);
        assert_eq!(output.defines[0].name, "APP_NAME");
        assert_eq!(output.defines[1].name, "GREETING");
        assert_eq!(
            output
                .line_origins
                .iter()
                .map(|origin| (origin.source_path.display().to_string(), origin.source_line))
                .collect::<Vec<_>>(),
            vec![
                ("fixtures/shared.ch".to_owned(), 2),
                ("fixtures/main.prg".to_owned(), 3),
                ("fixtures/main.prg".to_owned(), 4),
            ]
        );
    }

    #[test]
    fn reports_missing_include_as_preprocess_error() {
        let source = SourceFile::new(
            PathBuf::from("fixtures/main.prg"),
            "#include \"missing.ch\"\nPROCEDURE Main()\nRETURN\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert_eq!(output.text, "PROCEDURE Main()\nRETURN\n");
        assert_eq!(output.errors.len(), 1);
        assert!(
            output.errors[0]
                .message
                .contains("failed to resolve include")
        );
    }

    #[test]
    fn reports_invalid_define_names() {
        let source = SourceFile::new(PathBuf::from("main.prg"), "#define 1NAME value\n");

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert_eq!(output.errors.len(), 1);
        assert_eq!(
            output.errors[0].message,
            "expected valid define name after '#define'"
        );
    }

    #[test]
    fn expands_object_like_defines_case_insensitively_in_normal_lines() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#define APP_NAME \"harbour-rust\"\n#DEFINE GREETING \"hello\"\n? app_name\n? GREETING\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "? \"harbour-rust\"\n? \"hello\"\n");
    }

    #[test]
    fn expands_recursive_object_like_define_chains() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#define APP_NAME GREETING\n#define GREETING \"hello\"\n? APP_NAME\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "? \"hello\"\n");
    }

    #[test]
    fn does_not_expand_defines_inside_strings_or_comments() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#define APP_NAME \"harbour-rust\"\n? \"APP_NAME\"\n&& APP_NAME\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "? \"APP_NAME\"\n&& APP_NAME\n");
    }

    #[test]
    fn does_not_expand_function_like_defines_in_normal_lines() {
        let source = SourceFile::new(PathBuf::from("main.prg"), "#define WRAP(x) x\n? WRAP\n");

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "? WRAP\n");
    }

    #[test]
    fn reports_cycles_in_recursive_object_like_define_expansion() {
        let source = SourceFile::new(PathBuf::from("main.prg"), "#define A B\n#define B A\n? A\n");

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert_eq!(output.text, "? A\n");
        assert_eq!(output.errors.len(), 1);
        assert_eq!(
            output.errors[0].message,
            "cyclic define expansion detected: A -> B -> A"
        );
        assert_eq!(output.errors[0].line(), 3);
        assert_eq!(output.errors[0].column(), 3);
    }

    #[test]
    fn expands_translate_rule_inside_a_normal_source_line() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#translate DOUBLE(<value>) => <value> + <value>\nPROCEDURE Main()\n   LOCAL n := DOUBLE(3)\nRETURN\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(
            output.text,
            "PROCEDURE Main()\n   LOCAL n := 3 + 3\nRETURN\n"
        );
    }

    #[test]
    fn expands_command_rule_for_a_full_statement_line() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command EMIT <value> => ? <value>\nPROCEDURE Main()\n   EMIT n\nRETURN\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.text, "PROCEDURE Main()\n   ? n\nRETURN\n");
    }
}
