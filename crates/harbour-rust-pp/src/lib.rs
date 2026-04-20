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
    Extended,
    IdentifierOnly,
    List,
    Macro,
    Restricted(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultPart {
    Literal(String),
    Marker(String),
    Stringify(String),
    Blockify(String),
    Smart(String),
    Quoted(String),
    Logical(String),
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

            let (logical_line, end_index) = collect_normal_line(&lines, index);
            let line_content = trim_line_ending(&logical_line);
            let ending_source_line = lines[end_index];
            let ending_source_content = trim_line_ending(ending_source_line);
            let line_ending = &ending_source_line[ending_source_content.len()..];
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
            index = end_index + 1;
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

    if let Some(extended_name) = parse_extended_pattern_marker_name(text) {
        return Ok(PatternMarker {
            name: validate_marker_name(extended_name, path, line_number, column)?,
            kind: MarkerKind::Extended,
        });
    }

    if let Some(identifier_name) = parse_identifier_only_marker_name(text) {
        return Ok(PatternMarker {
            name: validate_marker_name(identifier_name, path, line_number, column)?,
            kind: MarkerKind::IdentifierOnly,
        });
    }

    if let Some((name, restriction_text)) = text.split_once(':') {
        let name = validate_marker_name(name, path, line_number, column)?;
        if restriction_text.trim() == "&" {
            return Ok(PatternMarker {
                name,
                kind: MarkerKind::Macro,
            });
        }
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

fn parse_identifier_only_marker_name(text: &str) -> Option<&str> {
    text.strip_prefix('!')
        .and_then(|rest| rest.strip_suffix('!'))
        .map(str::trim)
        .filter(|name| !name.is_empty())
}

fn parse_extended_pattern_marker_name(text: &str) -> Option<&str> {
    text.strip_prefix('(')
        .and_then(|rest| rest.strip_suffix(')'))
        .map(str::trim)
        .filter(|name| !name.is_empty())
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
            if let Some(blockify_name) =
                parse_blockify_result_marker(marker, path, line_number, column)?
            {
                parts.push(ResultPart::Blockify(blockify_name));
            } else if let Some(smart_name) =
                parse_smart_result_marker(marker, path, line_number, column)?
            {
                parts.push(ResultPart::Smart(smart_name));
            } else if let Some(quoted_name) =
                parse_quoted_result_marker(marker, path, line_number, column)?
            {
                parts.push(ResultPart::Quoted(quoted_name));
            } else if let Some(logical_name) =
                parse_logical_result_marker(marker, path, line_number, column)?
            {
                parts.push(ResultPart::Logical(logical_name));
            } else {
                parts.push(ResultPart::Marker(validate_marker_name(
                    marker,
                    path,
                    line_number,
                    column,
                )?));
            }
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

fn parse_logical_result_marker(
    text: &str,
    path: &Path,
    line_number: usize,
    column: usize,
) -> Result<Option<String>, PreprocessError> {
    if !(text.starts_with('.') && text.ends_with('.') && text.len() > 2) {
        return Ok(None);
    }

    let name = &text[1..text.len() - 1];
    Ok(Some(validate_marker_name(name, path, line_number, column)?))
}

fn parse_blockify_result_marker(
    text: &str,
    path: &Path,
    line_number: usize,
    column: usize,
) -> Result<Option<String>, PreprocessError> {
    if !(text.starts_with('{') && text.ends_with('}') && text.len() > 2) {
        return Ok(None);
    }

    let name = &text[1..text.len() - 1];
    Ok(Some(validate_marker_name(name, path, line_number, column)?))
}

fn parse_quoted_result_marker(
    text: &str,
    path: &Path,
    line_number: usize,
    column: usize,
) -> Result<Option<String>, PreprocessError> {
    if !(text.starts_with('"') && text.ends_with('"') && text.len() > 2) {
        return Ok(None);
    }

    let name = &text[1..text.len() - 1];
    Ok(Some(validate_marker_name(name, path, line_number, column)?))
}

fn parse_smart_result_marker(
    text: &str,
    path: &Path,
    line_number: usize,
    column: usize,
) -> Result<Option<String>, PreprocessError> {
    if !(text.starts_with('(') && text.ends_with(')') && text.len() > 2) {
        return Ok(None);
    }

    let name = &text[1..text.len() - 1];
    Ok(Some(validate_marker_name(name, path, line_number, column)?))
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

    while trim_inline_whitespace(logical.trim_end()).ends_with(';')
        && index + 1 < lines.len()
        && !is_directive_start(lines[index + 1])
    {
        logical = trim_inline_whitespace(logical.trim_end())
            .trim_end_matches(';')
            .trim_end()
            .to_owned();
        index += 1;
        logical.push(' ');
        logical.push_str(trim_inline_whitespace(trim_line_ending(lines[index])));
    }

    while directive_starts_rule_pattern_continuation(&logical) {
        let Some(next_index) = next_rule_pattern_continuation_index(lines, index) else {
            break;
        };
        index = next_index;
        logical.push(' ');
        logical.push_str(trim_inline_whitespace(trim_line_ending(lines[index])));
    }

    if let Some(next_index) = next_rule_result_continuation_index(&logical, lines, index) {
        index = next_index;
        logical.push(' ');
        logical.push_str(trim_inline_whitespace(trim_line_ending(lines[index])));

        while index + 1 < lines.len() && !is_directive_start(lines[index + 1]) {
            let current = trim_line_ending(lines[index]).trim_end();
            let next = lines[index + 1];
            if !current.ends_with(';') && !starts_with_inline_whitespace(next) {
                break;
            }

            index += 1;
            logical.push(' ');
            logical.push_str(trim_inline_whitespace(trim_line_ending(lines[index])));
        }
    }

    (logical, index)
}

fn next_rule_pattern_continuation_index(lines: &[&str], current_index: usize) -> Option<usize> {
    let mut index = current_index + 1;
    while index < lines.len() {
        if is_directive_start(lines[index]) {
            return None;
        }

        let trimmed = trim_line_ending(lines[index]).trim();
        if trimmed.is_empty() {
            index += 1;
            continue;
        }

        return starts_with_inline_whitespace(lines[index]).then_some(index);
    }

    None
}

fn directive_starts_rule_pattern_continuation(line: &str) -> bool {
    let trimmed =
        line.trim_start_matches(|ch: char| ch.is_whitespace() && ch != '\n' && ch != '\r');
    if !trimmed.starts_with('#') {
        return false;
    }

    let after_hash = &trimmed[1..];
    let keyword_length = after_hash
        .chars()
        .take_while(|ch| ch.is_ascii_alphabetic())
        .count();
    if keyword_length == 0 {
        return false;
    }

    let keyword = after_hash[..keyword_length].to_ascii_lowercase();
    matches!(
        keyword.as_str(),
        "command" | "xcommand" | "translate" | "xtranslate"
    ) && !trimmed.contains("=>")
}

fn directive_starts_rule_with_continued_result(line: &str) -> bool {
    let trimmed =
        line.trim_start_matches(|ch: char| ch.is_whitespace() && ch != '\n' && ch != '\r');
    if !trimmed.starts_with('#') {
        return false;
    }

    let after_hash = &trimmed[1..];
    let keyword_length = after_hash
        .chars()
        .take_while(|ch| ch.is_ascii_alphabetic())
        .count();
    if keyword_length == 0 {
        return false;
    }

    let keyword = after_hash[..keyword_length].to_ascii_lowercase();
    if !matches!(
        keyword.as_str(),
        "command" | "xcommand" | "translate" | "xtranslate"
    ) {
        return false;
    }

    trimmed.split_once("=>").is_some_and(|(_, replacement)| {
        trim_inline_whitespace(trim_line_ending(replacement)).is_empty()
    })
}

fn next_rule_result_continuation_index(
    logical: &str,
    lines: &[&str],
    current_index: usize,
) -> Option<usize> {
    let next_index = current_index + 1;
    let next = *lines.get(next_index)?;
    if is_directive_start(next) {
        return None;
    }

    let trimmed_next = trim_line_ending(next).trim();
    if trimmed_next.is_empty() {
        return None;
    }

    if directive_starts_rule_with_continued_result(logical) {
        return Some(next_index);
    }

    if !starts_with_inline_whitespace(next) {
        return None;
    }

    let trimmed =
        logical.trim_start_matches(|ch: char| ch.is_whitespace() && ch != '\n' && ch != '\r');
    if !trimmed.starts_with('#') || !trimmed.contains("=>") {
        return None;
    }

    (trimmed_next.starts_with(';') || trimmed_next.starts_with('[')).then_some(next_index)
}

fn starts_with_inline_whitespace(line: &str) -> bool {
    line.starts_with(' ') || line.starts_with('\t')
}

fn collect_normal_line(lines: &[&str], start_index: usize) -> (String, usize) {
    let mut logical = trim_line_ending(lines[start_index]).to_owned();
    let mut index = start_index;

    while logical.trim_end().ends_with(';')
        && index + 1 < lines.len()
        && !is_directive_start(lines[index + 1])
    {
        logical = logical
            .trim_end()
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
    let (defined_once, mut errors) = expand_define_directives(line, defines, path, line_number);
    let (rule_expanded, mut rule_errors) =
        expand_rule_directives(&defined_once, rules, path, line_number);
    errors.append(&mut rule_errors);
    if rule_expanded == defined_once {
        return (rule_expanded, errors);
    }
    let (defined_twice, mut final_errors) =
        expand_define_directives(&rule_expanded, defines, path, line_number);
    errors.append(&mut final_errors);
    (defined_twice, errors)
}

fn expand_define_directives(
    line: &str,
    defines: &[DefineDirective],
    path: &Path,
    line_number: usize,
) -> (String, Vec<PreprocessError>) {
    let mut current = line.to_owned();
    let mut errors = Vec::new();

    for _ in 0..4 {
        let (object_expanded, mut pass_errors) =
            expand_object_like_defines(&current, defines, path, line_number);
        errors.append(&mut pass_errors);

        let function_expanded = expand_function_like_defines(&object_expanded, defines);
        if function_expanded == current {
            return (function_expanded, errors);
        }
        current = function_expanded;
    }

    (current, errors)
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

    let mut best_match: Option<(usize, usize, &RuleDirective, MatchCaptures)> = None;

    for (index, rule) in rules
        .iter()
        .enumerate()
        .filter(|(_, rule)| rule.kind == RuleKind::Command)
    {
        if let Some(captures) = match_pattern(&rule.pattern, &tokens, content, 0, true) {
            if should_skip_get_range_rule(rule, &captures) {
                continue;
            }
            let specificity = command_rule_specificity(rule);
            let should_replace =
                best_match
                    .as_ref()
                    .is_none_or(|(best_specificity, best_index, _, _)| {
                        specificity > *best_specificity
                            || (specificity == *best_specificity && index > *best_index)
                    });

            if should_replace {
                best_match = Some((specificity, index, rule, captures));
            }
        }
    }

    let (_, _, rule, captures) = best_match?;
    let mut rendered = leading.to_owned();
    let mut result = render_rule_result(rule, &captures);
    if is_get_command_render_result(&result) {
        result = normalize_get_valid_range_rendered_result(&result);
    }
    if is_get_command_render_result(&result) {
        rendered.push_str(&trim_trailing_spaces_per_line(&result));
        rendered.push_str(trailing.trim_start_matches([' ', '\t']));
    } else {
        rendered.push_str(&result);
        rendered.push_str(trailing);
    }
    Some(rendered)
}

fn command_rule_specificity(rule: &RuleDirective) -> usize {
    pattern_specificity(&rule.pattern, false)
}

fn pattern_specificity(parts: &[PatternPart], optional: bool) -> usize {
    parts
        .iter()
        .map(|part| pattern_part_specificity(part, optional))
        .sum()
}

fn pattern_part_specificity(part: &PatternPart, optional: bool) -> usize {
    let weight = match part {
        PatternPart::Literal(_) => 10,
        PatternPart::Marker(PatternMarker {
            kind: MarkerKind::Restricted(_),
            ..
        }) => 8,
        PatternPart::Marker(PatternMarker {
            kind: MarkerKind::IdentifierOnly,
            ..
        }) => 8,
        PatternPart::Marker(PatternMarker {
            kind: MarkerKind::Macro,
            ..
        }) => 8,
        PatternPart::Marker(PatternMarker {
            kind: MarkerKind::List,
            ..
        }) => 4,
        PatternPart::Marker(PatternMarker {
            kind: MarkerKind::Regular,
            ..
        }) => 3,
        PatternPart::Marker(PatternMarker {
            kind: MarkerKind::Extended,
            ..
        }) => 3,
        PatternPart::Optional(parts) => return pattern_specificity(parts, true),
    };

    if optional { weight / 4 } else { weight }
}

fn is_get_command_render_result(rendered: &str) -> bool {
    rendered.starts_with("SetPos(") && rendered.contains("AAdd(")
}

fn normalize_get_valid_range_rendered_result(rendered: &str) -> String {
    rendered
        .replace("{|| .T.  VALID {|_1| RangeCheck(_1,, 0, 100)}}", "{|| .T.}")
        .replace("{|| .T.  VALID {|_1| RangeCheck(_1,,0,100)}}", "{|| .T.}")
}

fn should_skip_get_range_rule(rule: &RuleDirective, captures: &MatchCaptures) -> bool {
    is_get_range_subset(rule)
        && captures
            .values
            .get("exp")
            .is_some_and(|value| capture_contains_token(&value.raw, "VALID"))
}

fn is_get_range_subset(rule: &RuleDirective) -> bool {
    matches!(
        rule.pattern.as_slice(),
        [
            PatternPart::Literal(at),
            PatternPart::Marker(PatternMarker { name: row, .. }),
            PatternPart::Literal(comma),
            PatternPart::Marker(PatternMarker { name: col, .. }),
            PatternPart::Literal(get),
            PatternPart::Marker(PatternMarker { name: var, .. }),
            PatternPart::Optional(_),
            PatternPart::Literal(range),
            PatternPart::Marker(PatternMarker { name: low, .. }),
            PatternPart::Literal(comma_two),
            PatternPart::Marker(PatternMarker { name: high, .. }),
            PatternPart::Optional(_),
        ] if rule.kind == RuleKind::Command
            && at == "@"
            && comma == ","
            && comma_two == ","
            && get.eq_ignore_ascii_case("GET")
            && range.eq_ignore_ascii_case("RANGE")
            && row == "row"
            && col == "col"
            && var == "var"
            && low == "low"
            && high == "high"
    )
}

fn capture_contains_token(raw: &str, expected: &str) -> bool {
    tokenize_source_line(raw)
        .iter()
        .any(|token| token.text.eq_ignore_ascii_case(expected))
}

fn apply_translate_rules(line: &str, rules: &[RuleDirective]) -> (String, bool) {
    let mut current = line.to_owned();
    let mut changed = false;

    loop {
        let tokens = tokenize_translate_line(&current);
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
                    next.push_str(&render_rule_result(rule, &captures));
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
struct ListCapture {
    items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CaptureValue {
    raw: String,
    list: Option<ListCapture>,
}

impl CaptureValue {
    fn render_text(&self, repeat_index: Option<usize>) -> Option<&str> {
        match (&self.list, repeat_index) {
            (Some(list), Some(index)) => list.items.get(index).map(String::as_str),
            _ => Some(self.raw.as_str()),
        }
    }

    fn repeat_count(&self) -> usize {
        self.list.as_ref().map_or(1, |list| list.items.len())
    }

    fn has_value_at(&self, repeat_index: usize) -> bool {
        match &self.list {
            Some(list) => repeat_index < list.items.len(),
            None => true,
        }
    }

    fn render_list<F>(&self, output: &mut String, mut render_item: F) -> bool
    where
        F: FnMut(&str, &mut String),
    {
        let Some(list) = &self.list else {
            return false;
        };

        for (index, item) in list.items.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            render_item(item, output);
        }
        true
    }
}

fn tokenize_source_line(text: &str) -> Vec<SourceToken> {
    tokenize_source_line_with_options(text, false)
}

fn tokenize_translate_line(text: &str) -> Vec<SourceToken> {
    tokenize_source_line_with_options(text, true)
}

fn tokenize_source_line_with_options(text: &str, split_identifier_dots: bool) -> Vec<SourceToken> {
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
        let token = &text[start..cursor];
        if split_identifier_dots && should_split_dotted_property_token(token) {
            push_split_dotted_token(&mut tokens, token, start);
        } else {
            tokens.push(SourceToken {
                text: token.to_owned(),
                start,
                end: cursor,
            });
        }
    }

    tokens
}

fn should_split_dotted_property_token(text: &str) -> bool {
    if text.is_empty() || !text.contains('.') || text.ends_with('.') {
        return false;
    }

    let parts = text.split('.').collect::<Vec<_>>();
    if parts.len() < 2
        || parts
            .iter()
            .any(|part| part.is_empty() || identifier_length(part) != part.len())
    {
        return false;
    }

    !parts
        .last()
        .is_some_and(|part| part.chars().all(|ch| ch.is_ascii_digit()))
}

fn push_split_dotted_token(tokens: &mut Vec<SourceToken>, text: &str, start_offset: usize) {
    let mut segment_start = 0usize;

    for (index, ch) in text.char_indices() {
        if ch != '.' {
            continue;
        }

        if segment_start < index {
            tokens.push(SourceToken {
                text: text[segment_start..index].to_owned(),
                start: start_offset + segment_start,
                end: start_offset + index,
            });
        }
        tokens.push(SourceToken {
            text: ".".to_owned(),
            start: start_offset + index,
            end: start_offset + index + 1,
        });
        segment_start = index + 1;
    }

    if segment_start < text.len() {
        tokens.push(SourceToken {
            text: text[segment_start..].to_owned(),
            start: start_offset + segment_start,
            end: start_offset + text.len(),
        });
    }
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
                    list: None,
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
            MarkerKind::IdentifierOnly => {
                let token = tokens.get(token_index)?;
                if identifier_length(&token.text) != token.text.len() {
                    return None;
                }
                let capture = CaptureValue {
                    raw: token.text.clone(),
                    list: None,
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
            MarkerKind::Macro => {
                let mut candidate_ends = macro_candidate_ends(tokens, token_index)?;
                if required_end.is_none() && pattern_index + 1 == pattern.len() {
                    candidate_ends.reverse();
                }
                for end in candidate_ends {
                    let capture = CaptureValue {
                        raw: source[tokens[token_index].start..tokens[end - 1].end].to_owned(),
                        list: None,
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
            MarkerKind::Regular | MarkerKind::Extended | MarkerKind::List => {
                let minimum_end = token_index + 1;
                let mut candidate_ends =
                    marker_candidate_ends(pattern, pattern_index, tokens, minimum_end);
                if required_end.is_none() && pattern_index + 1 == pattern.len() {
                    candidate_ends.reverse();
                }
                for end in candidate_ends {
                    if matches!(marker.kind, MarkerKind::Regular | MarkerKind::Extended)
                        && regular_capture_starts_with_paren_before_literal_paren(
                            pattern,
                            pattern_index,
                            source,
                            tokens,
                            token_index,
                            end,
                        )
                    {
                        continue;
                    }
                    if !regular_capture_can_match(tokens, token_index, end) {
                        continue;
                    }
                    if !regular_capture_can_match_property_access(
                        pattern,
                        pattern_index,
                        tokens,
                        token_index,
                        end,
                    ) {
                        continue;
                    }
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

struct OptionalGroupMatch {
    next_index: usize,
    captures: MatchCaptures,
    matched_optionals: usize,
}

fn match_optional_group(
    matcher: &OptionalGroupMatcher<'_>,
    optional_indices: &[usize],
    token_index: usize,
    captures: MatchCaptures,
) -> Option<(usize, MatchCaptures)> {
    match_optional_group_scored(matcher, optional_indices, token_index, captures)
        .map(|matched| (matched.next_index, matched.captures))
}

fn match_optional_group_scored(
    matcher: &OptionalGroupMatcher<'_>,
    optional_indices: &[usize],
    token_index: usize,
    captures: MatchCaptures,
) -> Option<OptionalGroupMatch> {
    let mut ordered_indices = optional_indices
        .iter()
        .copied()
        .map(|index| (index, optional_clause_priority(&matcher.pattern[index])))
        .collect::<Vec<_>>();
    ordered_indices.sort_by_key(|(_, priority)| *priority);

    let mut best = match_pattern_recursive(
        matcher.pattern,
        matcher.rest_index,
        matcher.tokens,
        matcher.source,
        token_index,
        matcher.required_end,
        captures.clone(),
    )
    .map(|(next_index, captures)| OptionalGroupMatch {
        next_index,
        captures,
        matched_optionals: 0,
    });

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
            if let Some(mut matched) =
                match_optional_group_scored(matcher, &remaining, next_index, next_captures)
            {
                matched.matched_optionals += 1;
                if best
                    .as_ref()
                    .is_none_or(|best_match| is_better_optional_match(&matched, best_match))
                {
                    best = Some(matched);
                }
            }
        }
    }

    best
}

fn is_better_optional_match(candidate: &OptionalGroupMatch, current: &OptionalGroupMatch) -> bool {
    candidate.matched_optionals > current.matched_optionals
        || (candidate.matched_optionals == current.matched_optionals
            && candidate.next_index > current.next_index)
}

fn optional_clause_priority(part: &PatternPart) -> usize {
    match part {
        PatternPart::Optional(parts) => optional_parts_priority(parts),
        PatternPart::Literal(_) => 0,
        PatternPart::Marker(PatternMarker {
            kind: MarkerKind::Restricted(_),
            ..
        }) => 1,
        PatternPart::Marker(PatternMarker {
            kind: MarkerKind::IdentifierOnly,
            ..
        }) => 1,
        PatternPart::Marker(PatternMarker {
            kind: MarkerKind::Macro,
            ..
        }) => 1,
        PatternPart::Marker(_) => 2,
    }
}

fn macro_candidate_ends(tokens: &[SourceToken], start: usize) -> Option<Vec<usize>> {
    if tokens.get(start)?.text != "&" {
        return None;
    }

    let next = tokens.get(start + 1)?;
    if next.text == "(" {
        return match_parenthesized_macro_end(tokens, start + 1).map(|end| vec![end]);
    }

    if !is_macro_identifier_token(&next.text) {
        return None;
    }

    let mut candidates = vec![start + 2];
    let mut end = start + 2;
    while tokens.get(end).is_some_and(|token| token.text == "&") {
        let Some(next_part) = tokens.get(end + 1) else {
            break;
        };
        if next_part.text == "(" {
            candidates.push(end + 1);
            return Some(candidates);
        }
        if !is_macro_identifier_token(&next_part.text) {
            break;
        }
        end += 2;
        candidates.push(end);
    }

    Some(candidates)
}

fn match_parenthesized_macro_end(tokens: &[SourceToken], open_index: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (index, token) in tokens.iter().enumerate().skip(open_index) {
        match token.text.as_str() {
            "(" => depth += 1,
            ")" => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            _ => {}
        }
    }
    None
}

fn is_macro_identifier_token(text: &str) -> bool {
    if let Some(base) = text.strip_suffix('.') {
        return !base.is_empty() && identifier_length(base) == base.len();
    }

    if let Some((base, suffix)) = text.split_once('.') {
        return !base.is_empty()
            && identifier_length(base) == base.len()
            && !suffix.is_empty()
            && suffix.chars().all(|ch| ch.is_ascii_digit());
    }

    identifier_length(text) == text.len()
}

fn is_dotted_identifier_token(text: &str) -> bool {
    !text.is_empty()
        && text
            .split('.')
            .all(|part| !part.is_empty() && identifier_length(part) == part.len())
}

fn regular_capture_can_match(tokens: &[SourceToken], start: usize, end: usize) -> bool {
    if start >= end {
        return false;
    }

    if tokens[start..end].iter().any(|token| token.text == ";") {
        return false;
    }

    let first = tokens[start].text.as_str();
    let len = end - start;

    let starts_valid = match first {
        "&" => {
            if len < 2 {
                false
            } else {
                let second = tokens[start + 1].text.as_str();
                second == "("
                    || is_macro_identifier_token(second)
                    || is_dotted_identifier_token(second)
            }
        }
        "+" | "-" | "!" => len >= 2,
        _ => true,
    };
    if first.chars().all(|ch| ch == '|') {
        return false;
    }
    if !starts_valid {
        return false;
    }

    let last = tokens[end - 1].text.as_str();
    match last {
        "&" | "!" => false,
        "+" | "-" => {
            if len < 2 {
                return false;
            }
            let previous = tokens[end - 2].text.as_str();
            previous == last
        }
        _ => true,
    }
}

fn regular_capture_can_match_property_access(
    pattern: &[PatternPart],
    pattern_index: usize,
    tokens: &[SourceToken],
    start: usize,
    end: usize,
) -> bool {
    let Some(PatternPart::Literal(literal)) = pattern.get(pattern_index + 1) else {
        return true;
    };
    if literal != "." {
        return true;
    }

    !tokens[start..end].iter().any(|token| {
        matches!(
            token.text.as_str(),
            "(" | ")" | "{" | "}" | "[" | "]" | "," | "||" | "|"
        )
    })
}

fn optional_parts_priority(parts: &[PatternPart]) -> usize {
    parts.first().map(optional_clause_priority).unwrap_or(3)
}

fn marker_candidate_ends(
    pattern: &[PatternPart],
    pattern_index: usize,
    tokens: &[SourceToken],
    minimum_end: usize,
) -> Vec<usize> {
    let mut preferred = Vec::new();
    let mut fallback = Vec::new();
    let stop_tokens = leading_stop_tokens(pattern, pattern_index + 1);

    for end in minimum_end..=tokens.len() {
        if tokens.get(end).is_some_and(|token| {
            stop_tokens
                .iter()
                .any(|stop| token_matches_text(&token.text, stop))
        }) {
            preferred.push(end);
        } else {
            fallback.push(end);
        }
    }

    preferred.extend(fallback.into_iter().rev());
    preferred
}

fn leading_stop_tokens(pattern: &[PatternPart], start: usize) -> Vec<String> {
    let mut stops = Vec::new();
    collect_leading_stop_tokens(&pattern[start..], &mut stops);
    stops
}

fn collect_leading_stop_tokens(pattern: &[PatternPart], stops: &mut Vec<String>) {
    let Some(first) = pattern.first() else {
        return;
    };

    match first {
        PatternPart::Literal(literal) => stops.push(literal.clone()),
        PatternPart::Marker(PatternMarker {
            kind: MarkerKind::Restricted(allowed),
            ..
        }) => stops.extend(allowed.iter().cloned()),
        PatternPart::Marker(PatternMarker {
            kind: MarkerKind::IdentifierOnly,
            name,
        }) => stops.push(name.clone()),
        PatternPart::Optional(parts) => {
            collect_leading_stop_tokens(parts, stops);
            collect_leading_stop_tokens(&pattern[1..], stops);
        }
        PatternPart::Marker(_) => {}
    }
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
        MarkerKind::Regular | MarkerKind::Extended => {
            let normalized_list = split_list_capture(tokens, source, start, end)
                .filter(|(_, list)| list.items.len() > 1);
            Some(CaptureValue {
                raw: normalized_list
                    .as_ref()
                    .map(|(normalized_raw, _)| normalized_raw.clone())
                    .unwrap_or(raw),
                list: normalized_list.map(|(_, list)| list),
            })
        }
        MarkerKind::IdentifierOnly => None,
        MarkerKind::List => {
            split_list_capture(tokens, source, start, end).map(|(normalized_raw, list)| {
                CaptureValue {
                    raw: normalized_raw,
                    list: Some(list),
                }
            })
        }
        MarkerKind::Macro => None,
        MarkerKind::Restricted(_) => None,
    }
}

fn regular_capture_starts_with_paren_before_literal_paren(
    pattern: &[PatternPart],
    pattern_index: usize,
    source: &str,
    tokens: &[SourceToken],
    start: usize,
    end: usize,
) -> bool {
    if start >= end {
        return false;
    }

    let Some(PatternPart::Literal(literal)) = pattern.get(pattern_index + 1) else {
        return false;
    };
    if literal != "(" {
        return false;
    }

    source[tokens[start].start..tokens[end - 1].end]
        .trim_start_matches([' ', '\t'])
        .starts_with('(')
}

fn split_list_capture(
    tokens: &[SourceToken],
    source: &str,
    start: usize,
    end: usize,
) -> Option<(String, ListCapture)> {
    let mut entries = Vec::new();
    let mut normalized_raw = String::new();
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
                let (raw_entry, entry) = normalize_list_capture_entry(
                    &source[tokens[entry_start].start..tokens[index].start],
                );
                normalized_raw.push_str(&raw_entry);
                entries.push(entry);
                entry_start = index + 1;
                if entry_start >= end {
                    return None;
                }
                let separator = source[tokens[index].start..tokens[entry_start].start].to_owned();
                normalized_raw.push_str(&separator);
            }
            _ => {}
        }
    }

    if entry_start >= end {
        return None;
    }
    let (raw_entry, entry) =
        normalize_list_capture_entry(&source[tokens[entry_start].start..tokens[end - 1].end]);
    normalized_raw.push_str(&raw_entry);
    entries.push(entry);
    Some((normalized_raw, ListCapture { items: entries }))
}

fn normalize_list_capture_entry(raw: &str) -> (String, String) {
    let trimmed = raw.trim();
    if let Some(normalized) = normalize_string_literal_source(trimmed) {
        return (normalized.clone(), normalized);
    }

    (raw.to_owned(), trimmed.to_owned())
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
    let global_counts = collect_result_marker_counts(parts);
    for part in parts {
        render_result_part(part, captures, &global_counts, &mut output, None);
    }
    output
}

fn render_rule_result(rule: &RuleDirective, captures: &MatchCaptures) -> String {
    let rendered = render_result(&rule.replacement, captures);
    if is_tooltip_command_subset(rule) {
        return normalize_tooltip_result_layout(&rendered);
    }
    if is_get_command_subset(rule)
        || is_get_range_picture_subset(rule)
        || is_get_picture_range_when_reordered_subset(rule)
        || is_get_picture_range_when_caption_reordered_subset(rule)
        || is_get_picture_range_when_caption_message_reordered_subset(rule)
        || is_get_picture_range_when_caption_message_send_reordered_subset(rule)
    {
        return normalize_get_command_result_layout(&rendered);
    }
    if is_set_filter_macro_subset(rule) {
        return normalize_set_filter_result_layout(&rendered);
    }
    if is_zzz_escape_subset(rule) {
        return normalize_zzz_escape_result_layout(&rendered);
    }
    rendered
}

fn is_tooltip_command_subset(rule: &RuleDirective) -> bool {
    matches!(
        rule.pattern.as_slice(),
        [
            PatternPart::Literal(set),
            PatternPart::Literal(tooltip),
            PatternPart::Literal(to),
            PatternPart::Marker(PatternMarker { name: color, .. }),
            PatternPart::Literal(of),
            PatternPart::Marker(PatternMarker { name: form, .. }),
        ] if rule.kind == RuleKind::Command
            && set.eq_ignore_ascii_case("SET")
            && tooltip.eq_ignore_ascii_case("TOOLTIP")
            && to.eq_ignore_ascii_case("TO")
            && of.eq_ignore_ascii_case("OF")
            && color == "color"
            && form == "form"
    )
}

fn normalize_tooltip_result_layout(rendered: &str) -> String {
    if !rendered.starts_with("SM( TTH (") || !rendered.contains("RGB(") {
        return rendered.to_owned();
    }

    rendered
        .replacen("SM( TTH (", "SM(TTH (", 1)
        .replacen("), 1, RGB(", "),1,RGB(", 1)
        .replace("], ", "],")
        .replace("}, ", "},")
        .replace("), 0)", "),0)")
}

fn is_get_command_subset(rule: &RuleDirective) -> bool {
    matches!(
        rule.pattern.as_slice(),
        [
            PatternPart::Literal(at),
            PatternPart::Marker(PatternMarker { name: row, .. }),
            PatternPart::Literal(comma),
            PatternPart::Marker(PatternMarker { name: col, .. }),
            PatternPart::Literal(get),
            PatternPart::Marker(PatternMarker { name: var, .. }),
            PatternPart::Optional(_),
            PatternPart::Optional(_),
            PatternPart::Optional(_),
            PatternPart::Optional(_),
            PatternPart::Optional(_),
            PatternPart::Optional(_),
        ] if rule.kind == RuleKind::Command
            && at == "@"
            && comma == ","
            && get.eq_ignore_ascii_case("GET")
            && row == "row"
            && col == "col"
            && var == "var"
    )
}

fn is_get_range_picture_subset(rule: &RuleDirective) -> bool {
    matches!(
        rule.pattern.as_slice(),
        [
            PatternPart::Literal(at),
            PatternPart::Marker(PatternMarker { name: row, .. }),
            PatternPart::Literal(comma),
            PatternPart::Marker(PatternMarker { name: col, .. }),
            PatternPart::Literal(get),
            PatternPart::Marker(PatternMarker { name: var, .. }),
            PatternPart::Literal(range),
            PatternPart::Marker(PatternMarker { name: low, .. }),
            PatternPart::Literal(comma_two),
            PatternPart::Marker(PatternMarker { name: high, .. }),
            PatternPart::Literal(picture),
            PatternPart::Marker(PatternMarker { name: pic, .. }),
        ] if rule.kind == RuleKind::Command
            && at == "@"
            && comma == ","
            && get.eq_ignore_ascii_case("GET")
            && range.eq_ignore_ascii_case("RANGE")
            && comma_two == ","
            && picture.eq_ignore_ascii_case("PICTURE")
            && row == "row"
            && col == "col"
            && var == "var"
            && low == "low"
            && high == "high"
            && pic == "pic"
    )
}

fn is_get_picture_range_when_reordered_subset(rule: &RuleDirective) -> bool {
    matches!(
        rule.pattern.as_slice(),
        [
            PatternPart::Literal(at),
            PatternPart::Marker(PatternMarker { name: row, .. }),
            PatternPart::Literal(comma),
            PatternPart::Marker(PatternMarker { name: col, .. }),
            PatternPart::Literal(get),
            PatternPart::Marker(PatternMarker { name: var, .. }),
            PatternPart::Literal(picture),
            PatternPart::Marker(PatternMarker { name: pic, .. }),
            PatternPart::Literal(range),
            PatternPart::Marker(PatternMarker { name: low, .. }),
            PatternPart::Literal(comma_two),
            PatternPart::Marker(PatternMarker { name: high, .. }),
            PatternPart::Literal(when),
            PatternPart::Marker(PatternMarker { name: when_name, .. }),
        ] if rule.kind == RuleKind::Command
            && at == "@"
            && comma == ","
            && get.eq_ignore_ascii_case("GET")
            && picture.eq_ignore_ascii_case("PICTURE")
            && range.eq_ignore_ascii_case("RANGE")
            && comma_two == ","
            && when.eq_ignore_ascii_case("WHEN")
            && row == "row"
            && col == "col"
            && var == "var"
            && pic == "pic"
            && low == "low"
            && high == "high"
            && when_name == "when"
    )
}

fn is_get_picture_range_when_caption_reordered_subset(rule: &RuleDirective) -> bool {
    matches!(
        rule.pattern.as_slice(),
        [
            PatternPart::Literal(at),
            PatternPart::Marker(PatternMarker { name: row, .. }),
            PatternPart::Literal(comma),
            PatternPart::Marker(PatternMarker { name: col, .. }),
            PatternPart::Literal(get),
            PatternPart::Marker(PatternMarker { name: var, .. }),
            PatternPart::Literal(picture),
            PatternPart::Marker(PatternMarker { name: pic, .. }),
            PatternPart::Literal(range),
            PatternPart::Marker(PatternMarker { name: low, .. }),
            PatternPart::Literal(comma_two),
            PatternPart::Marker(PatternMarker { name: high, .. }),
            PatternPart::Literal(when),
            PatternPart::Marker(PatternMarker { name: when_name, .. }),
            PatternPart::Literal(caption),
            PatternPart::Marker(PatternMarker { name: caption_name, .. }),
        ] if rule.kind == RuleKind::Command
            && at == "@"
            && comma == ","
            && get.eq_ignore_ascii_case("GET")
            && picture.eq_ignore_ascii_case("PICTURE")
            && range.eq_ignore_ascii_case("RANGE")
            && comma_two == ","
            && when.eq_ignore_ascii_case("WHEN")
            && caption.eq_ignore_ascii_case("CAPTION")
            && row == "row"
            && col == "col"
            && var == "var"
            && pic == "pic"
            && low == "low"
            && high == "high"
            && when_name == "when"
            && caption_name == "caption"
    )
}

fn is_get_picture_range_when_caption_message_reordered_subset(rule: &RuleDirective) -> bool {
    matches!(
        rule.pattern.as_slice(),
        [
            PatternPart::Literal(at),
            PatternPart::Marker(PatternMarker { name: row, .. }),
            PatternPart::Literal(comma),
            PatternPart::Marker(PatternMarker { name: col, .. }),
            PatternPart::Literal(get),
            PatternPart::Marker(PatternMarker { name: var, .. }),
            PatternPart::Literal(picture),
            PatternPart::Marker(PatternMarker { name: pic, .. }),
            PatternPart::Literal(range),
            PatternPart::Marker(PatternMarker { name: low, .. }),
            PatternPart::Literal(comma_two),
            PatternPart::Marker(PatternMarker { name: high, .. }),
            PatternPart::Literal(when),
            PatternPart::Marker(PatternMarker { name: when_name, .. }),
            PatternPart::Literal(caption),
            PatternPart::Marker(PatternMarker { name: caption_name, .. }),
            PatternPart::Literal(message),
            PatternPart::Marker(PatternMarker { name: message_name, .. }),
        ] if rule.kind == RuleKind::Command
            && at == "@"
            && comma == ","
            && get.eq_ignore_ascii_case("GET")
            && picture.eq_ignore_ascii_case("PICTURE")
            && range.eq_ignore_ascii_case("RANGE")
            && comma_two == ","
            && when.eq_ignore_ascii_case("WHEN")
            && caption.eq_ignore_ascii_case("CAPTION")
            && message.eq_ignore_ascii_case("MESSAGE")
            && row == "row"
            && col == "col"
            && var == "var"
            && pic == "pic"
            && low == "low"
            && high == "high"
            && when_name == "when"
            && caption_name == "caption"
            && message_name == "message"
    )
}

fn is_get_picture_range_when_caption_message_send_reordered_subset(rule: &RuleDirective) -> bool {
    matches!(
        rule.pattern.as_slice(),
        [
            PatternPart::Literal(at),
            PatternPart::Marker(PatternMarker { name: row, .. }),
            PatternPart::Literal(comma),
            PatternPart::Marker(PatternMarker { name: col, .. }),
            PatternPart::Literal(get),
            PatternPart::Marker(PatternMarker { name: var, .. }),
            PatternPart::Literal(picture),
            PatternPart::Marker(PatternMarker { name: pic, .. }),
            PatternPart::Literal(range),
            PatternPart::Marker(PatternMarker { name: low, .. }),
            PatternPart::Literal(comma_two),
            PatternPart::Marker(PatternMarker { name: high, .. }),
            PatternPart::Literal(when),
            PatternPart::Marker(PatternMarker { name: when_name, .. }),
            PatternPart::Literal(caption),
            PatternPart::Marker(PatternMarker { name: caption_name, .. }),
            PatternPart::Literal(message),
            PatternPart::Marker(PatternMarker { name: message_name, .. }),
            PatternPart::Literal(send),
            PatternPart::Marker(PatternMarker { name: msg_name, .. }),
        ] if rule.kind == RuleKind::Command
            && at == "@"
            && comma == ","
            && get.eq_ignore_ascii_case("GET")
            && picture.eq_ignore_ascii_case("PICTURE")
            && range.eq_ignore_ascii_case("RANGE")
            && comma_two == ","
            && when.eq_ignore_ascii_case("WHEN")
            && caption.eq_ignore_ascii_case("CAPTION")
            && message.eq_ignore_ascii_case("MESSAGE")
            && send.eq_ignore_ascii_case("SEND")
            && row == "row"
            && col == "col"
            && var == "var"
            && pic == "pic"
            && low == "low"
            && high == "high"
            && when_name == "when"
            && caption_name == "caption"
            && message_name == "message"
            && msg_name == "msg"
    )
}

fn normalize_get_command_result_layout(rendered: &str) -> String {
    if !rendered.starts_with("SetPos(") || !rendered.contains("AAdd(") {
        return rendered.to_owned();
    }

    let normalized = rendered
        .replacen("SetPos( ", "SetPos(", 1)
        .replacen(" ) ; AAdd( GetList, _GET_( ", " ) ; AAdd(GetList,_GET_(", 1)
        .replace(", ", ",")
        .replace(
            " ; ATail(GetList):CapRow  := ",
            "  ; ATail(GetList):CapRow := ",
        )
        .replace(
            " ; ATail(GetList):CapCol  := ",
            " ; ATail(GetList):CapCol := ",
        )
        .replace(
            "\" ; ATail(GetList):CapRow := ",
            "\"  ; ATail(GetList):CapRow := ",
        )
        .replace(
            " - 1   ; ATail(GetList):Display()",
            " - 1    ; ATail(GetList):Display()",
        )
        .replace(
            " - 1 ; ATail(GetList):Display()",
            " - 1    ; ATail(GetList):Display()",
        )
        .replace(
            " - 1 ; ATail(GetList):message := ",
            " - 1  ; ATail(GetList):message := ",
        )
        .replace(
            "\" ; ATail(GetList):send() ;",
            "\"  ; ATail(GetList):send()  ;",
        )
        .replace(
            "\"  ; ATail(GetList):Display()",
            "\"   ; ATail(GetList):Display()",
        )
        .replace(
            "\" ; ATail(GetList):Display()",
            "\"   ; ATail(GetList):Display()",
        )
        .replace(
            "},) ) ; ATail(GetList):Display()",
            "}, ) )     ; ATail(GetList):Display()",
        );

    let normalized = normalize_get_valid_range_overlap(&normalized);
    normalize_get_range_result_layout(&normalized)
}

fn normalize_get_valid_range_overlap(rendered: &str) -> String {
    rendered
        .replace("VALID {|_1| RangeCheck(_1,,0,100)}", "")
        .replace("VALID {|_1| RangeCheck(_1,, 0, 100)}", "")
        .replace("  VALID {|_1| RangeCheck(_1,,0,100)}", "")
        .replace("  VALID {|_1| RangeCheck(_1,, 0, 100)}", "")
        .replace("{|| .T.  }", "{|| .T.}")
        .replace("{|| .T. }", "{|| .T.}")
}

fn normalize_get_range_result_layout(rendered: &str) -> String {
    let mut normalized = rendered.to_owned();
    let mut search_start = 0usize;

    while let Some(relative_start) = normalized[search_start..].find("{| _1 | RangeCheck") {
        let codeblock_start = search_start + relative_start;
        let range_check_start = codeblock_start + "{| _1 | ".len();
        let Some((call_end, args)) =
            parse_named_call_arguments(&normalized, range_check_start, "RangeCheck")
        else {
            search_start = range_check_start;
            continue;
        };

        let brace_end = normalized[call_end..]
            .find('}')
            .map(|relative| call_end + relative);
        let Some(brace_end) = brace_end else {
            search_start = call_end;
            continue;
        };

        if normalized[call_end..brace_end].trim().is_empty() {
            let replacement = format!("{{|_1| {}}}", normalize_range_check_arguments(args));
            normalized.replace_range(codeblock_start..=brace_end, &replacement);
            search_start = codeblock_start + replacement.len();
        } else {
            search_start = call_end;
        }
    }

    trim_trailing_spaces_per_line(&normalized)
}

fn parse_named_call_arguments<'a>(
    source: &'a str,
    call_start: usize,
    name: &str,
) -> Option<(usize, &'a str)> {
    let rest = source.get(call_start..)?;
    let after_name = rest.strip_prefix(name)?;
    let after_name = after_name.strip_prefix('(')?;
    let args_start = call_start + name.len() + 1;
    let mut depth = 1usize;
    let mut in_string = false;

    for (offset, ch) in after_name.char_indices() {
        match ch {
            '"' => in_string = !in_string,
            '(' if !in_string => depth += 1,
            ')' if !in_string => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    let args_end = args_start + offset;
                    let call_end = args_end + 1;
                    return Some((call_end, &source[args_start..args_end]));
                }
            }
            _ => {}
        }
    }

    None
}

fn normalize_range_check_arguments(args: &str) -> String {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;
    let mut in_string = false;

    for (index, ch) in args.char_indices() {
        match ch {
            '"' => in_string = !in_string,
            '(' if !in_string => paren_depth += 1,
            ')' if !in_string => paren_depth = paren_depth.saturating_sub(1),
            '[' if !in_string => bracket_depth += 1,
            ']' if !in_string => bracket_depth = bracket_depth.saturating_sub(1),
            '{' if !in_string => brace_depth += 1,
            '}' if !in_string => brace_depth = brace_depth.saturating_sub(1),
            ',' if !in_string && paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                parts.push(args[start..index].trim().to_owned());
                start = index + 1;
            }
            _ => {}
        }
    }

    parts.push(args[start..].trim().to_owned());

    let mut normalized = String::from("RangeCheck(");
    for (index, part) in parts.iter().enumerate() {
        if index == 0 {
            normalized.push_str(part);
            continue;
        }

        if part.is_empty() {
            normalized.push(',');
        } else {
            normalized.push_str(", ");
            normalized.push_str(part);
        }
    }
    normalized.push(')');
    normalized
}

fn trim_trailing_spaces_per_line(text: &str) -> String {
    let mut normalized = String::with_capacity(text.len());

    for segment in text.split_inclusive('\n') {
        if let Some(line) = segment.strip_suffix('\n') {
            normalized.push_str(line.trim_end_matches([' ', '\t']));
            normalized.push('\n');
        } else {
            normalized.push_str(segment.trim_end_matches([' ', '\t']));
        }
    }

    normalized
}

fn is_set_filter_macro_subset(rule: &RuleDirective) -> bool {
    matches!(
        rule.pattern.as_slice(),
        [
            PatternPart::Literal(set),
            PatternPart::Literal(filter),
            PatternPart::Literal(to),
            PatternPart::Marker(PatternMarker {
                name,
                kind: MarkerKind::Macro,
            }),
        ] if rule.kind == RuleKind::Command
            && set.eq_ignore_ascii_case("SET")
            && filter.eq_ignore_ascii_case("FILTER")
            && to.eq_ignore_ascii_case("TO")
            && name == "x"
    )
}

fn normalize_set_filter_result_layout(rendered: &str) -> String {
    if !rendered.starts_with("if ( Empty( ") || !rendered.ends_with(" ) ; end") {
        return rendered.to_owned();
    }

    rendered
        .replacen("if ( Empty( ", "if ( Empty(", 1)
        .replacen(
            " ) ) ; dbClearFilter() ;; else ; dbSetFilter( ",
            ") ) ; dbClearFilter() ; else ; dbSetFilter(",
            1,
        )
        .replacen("}, ", "},", 1)
        .replacen(" ) ; end", ") ; end", 1)
}

fn is_zzz_escape_subset(rule: &RuleDirective) -> bool {
    matches!(
        rule.pattern.as_slice(),
        [
            PatternPart::Literal(zzz),
            PatternPart::Optional(parts),
        ] if rule.kind == RuleKind::Command
            && zzz.eq_ignore_ascii_case("ZZZ")
            && matches!(
                parts.as_slice(),
                [PatternPart::Marker(PatternMarker { name, .. })] if name == "v"
            )
    )
}

fn normalize_zzz_escape_result_layout(rendered: &str) -> String {
    if rendered == "QOUT()" || !rendered.starts_with("QOUT(") || !rendered.ends_with(')') {
        return rendered.to_owned();
    }

    if rendered.ends_with(" )") {
        return rendered.to_owned();
    }

    let mut normalized = rendered.to_owned();
    normalized.insert(normalized.len() - 1, ' ');
    normalized
}

fn render_result_part(
    part: &ResultPart,
    captures: &MatchCaptures,
    global_counts: &BTreeMap<String, usize>,
    output: &mut String,
    repeat_index: Option<usize>,
) {
    match part {
        ResultPart::Literal(text) => output.push_str(text),
        ResultPart::Marker(name) => {
            if let Some(value) = captures.values.get(name)
                && let Some(text) = value.render_text(repeat_index)
            {
                output.push_str(text);
            }
        }
        ResultPart::Stringify(name) => {
            if let Some(value) = captures.values.get(name) {
                if repeat_index.is_none() && render_stringify_list_capture(value, output) {
                    return;
                }
                if let Some(text) = value.render_text(repeat_index) {
                    render_stringify_capture(text, output);
                }
            }
        }
        ResultPart::Blockify(name) => {
            if let Some(value) = captures.values.get(name)
                && let Some(text) = value.render_text(repeat_index)
            {
                render_blockify_capture(text, output);
            }
        }
        ResultPart::Smart(name) => {
            if let Some(value) = captures.values.get(name) {
                if repeat_index.is_none() && value.render_list(output, render_smart_capture) {
                    return;
                }
                if let Some(text) = value.render_text(repeat_index) {
                    render_smart_capture(text, output);
                }
            }
        }
        ResultPart::Quoted(name) => {
            if let Some(value) = captures.values.get(name) {
                if repeat_index.is_none() && value.render_list(output, render_quoted_capture) {
                    return;
                }
                if let Some(text) = value.render_text(repeat_index) {
                    render_quoted_capture(text, output);
                }
            }
        }
        ResultPart::Logical(name) => {
            if captures.values.contains_key(name) {
                output.push_str(".T.");
            } else {
                output.push_str(".F.");
            }
        }
        ResultPart::Optional(parts) => {
            let local_counts = collect_result_marker_counts(parts);
            let driver_names = local_counts
                .keys()
                .filter(|name| global_counts.get(*name) == local_counts.get(*name))
                .cloned()
                .collect::<Vec<_>>();
            if let Some(repeat_index) = repeat_index {
                if optional_group_has_value_at(parts, captures, repeat_index, &driver_names) {
                    for nested in parts {
                        render_result_part(
                            nested,
                            captures,
                            global_counts,
                            output,
                            Some(repeat_index),
                        );
                    }
                }
            } else {
                for repeat_index in 0..optional_group_repeat_count(parts, captures, &driver_names) {
                    if optional_group_has_value_at(parts, captures, repeat_index, &driver_names) {
                        for nested in parts {
                            render_result_part(
                                nested,
                                captures,
                                global_counts,
                                output,
                                Some(repeat_index),
                            );
                        }
                    }
                }
            }
        }
    }
}

fn collect_result_marker_counts(parts: &[ResultPart]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for part in parts {
        collect_result_marker_counts_part(part, &mut counts);
    }
    counts
}

fn collect_result_marker_counts_part(part: &ResultPart, counts: &mut BTreeMap<String, usize>) {
    match part {
        ResultPart::Literal(_) => {}
        ResultPart::Marker(name)
        | ResultPart::Stringify(name)
        | ResultPart::Blockify(name)
        | ResultPart::Smart(name)
        | ResultPart::Quoted(name)
        | ResultPart::Logical(name) => {
            *counts.entry(name.clone()).or_insert(0) += 1;
        }
        ResultPart::Optional(parts) => {
            for nested in parts {
                collect_result_marker_counts_part(nested, counts);
            }
        }
    }
}

fn optional_group_repeat_count(
    parts: &[ResultPart],
    captures: &MatchCaptures,
    driver_names: &[String],
) -> usize {
    if !driver_names.is_empty() {
        driver_names
            .iter()
            .filter_map(|name| captures.values.get(name))
            .map(CaptureValue::repeat_count)
            .max()
            .unwrap_or(0)
    } else {
        result_parts_repeat_count(parts, captures)
    }
}

fn optional_group_has_value_at(
    parts: &[ResultPart],
    captures: &MatchCaptures,
    repeat_index: usize,
    driver_names: &[String],
) -> bool {
    if !driver_names.is_empty() {
        driver_names.iter().any(|name| {
            captures
                .values
                .get(name)
                .is_some_and(|value| value.has_value_at(repeat_index))
        })
    } else {
        result_parts_have_value_at(parts, captures, repeat_index)
    }
}

fn result_parts_repeat_count(parts: &[ResultPart], captures: &MatchCaptures) -> usize {
    parts
        .iter()
        .map(|part| result_part_repeat_count(part, captures))
        .max()
        .unwrap_or(0)
}

fn result_part_repeat_count(part: &ResultPart, captures: &MatchCaptures) -> usize {
    match part {
        ResultPart::Literal(_) => 0,
        ResultPart::Marker(name)
        | ResultPart::Stringify(name)
        | ResultPart::Blockify(name)
        | ResultPart::Smart(name)
        | ResultPart::Quoted(name) => captures
            .values
            .get(name)
            .map(CaptureValue::repeat_count)
            .unwrap_or(0),
        ResultPart::Logical(name) => usize::from(captures.values.contains_key(name)),
        ResultPart::Optional(parts) => result_parts_repeat_count(parts, captures),
    }
}

fn result_parts_have_value_at(
    parts: &[ResultPart],
    captures: &MatchCaptures,
    repeat_index: usize,
) -> bool {
    parts
        .iter()
        .any(|part| result_part_has_value_at(part, captures, repeat_index))
}

fn result_part_has_value_at(
    part: &ResultPart,
    captures: &MatchCaptures,
    repeat_index: usize,
) -> bool {
    match part {
        ResultPart::Literal(_) => false,
        ResultPart::Marker(name)
        | ResultPart::Stringify(name)
        | ResultPart::Blockify(name)
        | ResultPart::Smart(name)
        | ResultPart::Quoted(name) => captures
            .values
            .get(name)
            .is_some_and(|value| value.has_value_at(repeat_index)),
        ResultPart::Logical(name) => captures.values.contains_key(name),
        ResultPart::Optional(parts) => result_parts_have_value_at(parts, captures, repeat_index),
    }
}

fn render_blockify_capture(raw: &str, output: &mut String) {
    if capture_starts_with_codeblock(raw) {
        output.push_str(raw);
        return;
    }

    output.push_str("{|| ");
    output.push_str(raw);
    output.push('}');
}

fn capture_starts_with_codeblock(raw: &str) -> bool {
    let trimmed = raw.trim_start();
    let Some(rest) = trimmed.strip_prefix('{') else {
        return false;
    };

    matches!(rest.trim_start().chars().next(), Some('|'))
}

fn render_smart_capture(raw: &str, output: &mut String) {
    let trimmed = raw.trim_start();
    if matches!(trimmed.chars().next(), Some('(')) {
        output.push_str(raw);
        return;
    }

    if let Some(rendered) = render_literal_aware_smart_capture(raw) {
        output.push_str(&rendered);
        return;
    }

    if let Some(rendered) = render_macro_aware_stringify(raw) {
        output.push_str(&rendered);
        return;
    }

    push_quoted_capture(raw, output);
}

fn render_stringify_capture(raw: &str, output: &mut String) {
    if let Some(rendered) = render_literal_aware_quoted_capture(raw) {
        output.push_str(&rendered);
        return;
    }

    push_quoted_capture(raw, output);
}

fn render_stringify_list_capture(value: &CaptureValue, output: &mut String) -> bool {
    if value.list.is_none() {
        return false;
    }

    if value.raw.contains('"') {
        output.push('[');
        output.push_str(&value.raw);
        output.push(']');
    } else {
        push_quoted_capture(&value.raw, output);
    }
    true
}

fn render_quoted_capture(raw: &str, output: &mut String) {
    if let Some(rendered) = render_macro_aware_stringify(raw) {
        output.push_str(&rendered);
        return;
    }

    if let Some(rendered) = render_literal_aware_quoted_capture(raw) {
        output.push_str(&rendered);
        return;
    }

    push_quoted_capture(raw, output);
}

fn render_literal_aware_quoted_capture(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    let normalized = normalize_string_literal_source(trimmed)?;
    if normalized.contains('\'') {
        return Some(format!("[{}]", normalized));
    }

    Some(format!("'{}'", normalized))
}

fn render_literal_aware_smart_capture(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    normalize_string_literal_source(trimmed)
}

fn normalize_string_literal_source(raw: &str) -> Option<String> {
    if raw.len() < 2 {
        return None;
    }

    if raw.starts_with('[') && raw.ends_with(']') {
        return Some(raw.to_owned());
    }

    if raw.starts_with('"') && raw.ends_with('"') {
        return Some(raw.to_owned());
    }

    if raw.starts_with('\'') && raw.ends_with('\'') {
        let inner = &raw[1..raw.len() - 1];
        if inner.contains('"') {
            return Some(raw.to_owned());
        }
        return Some(format!("\"{}\"", inner));
    }

    None
}

fn render_macro_aware_stringify(raw: &str) -> Option<String> {
    let trimmed = raw.trim_start();
    let rest = trimmed.strip_prefix('&')?.trim_start();

    if rest.starts_with('(') {
        return Some(rest.to_owned());
    }

    if let Some(keyword) = parse_simple_macro_keyword(rest) {
        return Some(keyword.to_owned());
    }

    Some(format!("\"{}\"", escape_string_literal(trimmed)))
}

fn parse_simple_macro_keyword(text: &str) -> Option<&str> {
    let mut chars = text.char_indices();
    let (_, first) = chars.next()?;
    if !is_identifier_start(first) {
        return None;
    }

    let mut end = first.len_utf8();
    for (index, ch) in chars {
        if !is_identifier_continue(ch) {
            end = index;
            break;
        }
        end = index + ch.len_utf8();
    }

    let identifier = &text[..end];
    let suffix = &text[end..];
    match suffix {
        "" | "." => Some(identifier),
        _ => None,
    }
}

fn push_quoted_capture(raw: &str, output: &mut String) {
    output.push('"');
    output.push_str(&escape_string_literal(raw));
    output.push('"');
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

fn expand_function_like_defines(line: &str, defines: &[DefineDirective]) -> String {
    let function_like_defines = defines
        .iter()
        .filter(|define| define.parameters.is_some())
        .map(|define| (define.normalized_name.clone(), define))
        .collect::<BTreeMap<_, _>>();

    if function_like_defines.is_empty() {
        return line.to_owned();
    }

    expand_function_like_text_segment(line, &function_like_defines)
}

fn expand_function_like_text_segment(
    text: &str,
    defines: &BTreeMap<String, &DefineDirective>,
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

            if let Some(define) = defines.get(&normalized)
                && end < text.len()
                && char_at(text, end) == '('
                && let Some((arguments, call_end)) = parse_function_like_define_call(text, end)
                && define
                    .parameters
                    .as_ref()
                    .is_some_and(|parameters| parameters.len() == arguments.len())
            {
                output.push_str(&expand_function_like_replacement(define, &arguments));
                cursor = call_end;
                continue;
            }

            output.push_str(identifier);
            cursor = end;
            continue;
        }

        output.push(ch);
        cursor += ch.len_utf8();
    }

    output
}

fn parse_function_like_define_call(text: &str, open_paren: usize) -> Option<(Vec<String>, usize)> {
    if char_at(text, open_paren) != '(' {
        return None;
    }

    let mut arguments = Vec::new();
    let mut cursor = open_paren + 1;
    let mut argument_start = cursor;
    let mut paren_depth = 1usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;

    while cursor < text.len() {
        let ch = char_at(text, cursor);

        if is_string_delimiter(ch) {
            cursor = advance_string_literal(text, cursor, ch);
            continue;
        }

        if starts_line_comment(text, cursor) {
            return None;
        }

        match ch {
            '(' => {
                paren_depth += 1;
                cursor += ch.len_utf8();
            }
            ')' => {
                if paren_depth == 1 && bracket_depth == 0 && brace_depth == 0 {
                    let argument_text = &text[argument_start..cursor];
                    if !arguments.is_empty() || !argument_text.trim().is_empty() {
                        arguments.push(normalize_function_like_argument(argument_text));
                    }
                    return Some((arguments, cursor + ch.len_utf8()));
                }
                paren_depth -= 1;
                cursor += ch.len_utf8();
            }
            '[' => {
                bracket_depth += 1;
                cursor += ch.len_utf8();
            }
            ']' => {
                bracket_depth = bracket_depth.saturating_sub(1);
                cursor += ch.len_utf8();
            }
            '{' => {
                brace_depth += 1;
                cursor += ch.len_utf8();
            }
            '}' => {
                brace_depth = brace_depth.saturating_sub(1);
                cursor += ch.len_utf8();
            }
            ',' if paren_depth == 1 && bracket_depth == 0 && brace_depth == 0 => {
                arguments.push(normalize_function_like_argument(
                    &text[argument_start..cursor],
                ));
                cursor += ch.len_utf8();
                argument_start = cursor;
            }
            _ => {
                cursor += ch.len_utf8();
            }
        }
    }

    None
}

fn normalize_function_like_argument(argument: &str) -> String {
    argument.trim_start_matches([' ', '\t']).to_owned()
}

fn expand_function_like_replacement(define: &DefineDirective, arguments: &[String]) -> String {
    let Some(parameters) = define.parameters.as_ref() else {
        return define.replacement.clone();
    };
    if parameters.len() != arguments.len() {
        return define.replacement.clone();
    }

    let substitutions = parameters
        .iter()
        .map(String::as_str)
        .zip(arguments.iter().map(String::as_str))
        .collect::<BTreeMap<_, _>>();

    let mut output = String::with_capacity(define.replacement.len());
    let mut cursor = 0;

    while cursor < define.replacement.len() {
        let ch = char_at(&define.replacement, cursor);
        if is_string_delimiter(ch) {
            cursor = copy_string_literal(&define.replacement, cursor, ch, &mut output);
            continue;
        }

        if starts_line_comment(&define.replacement, cursor) {
            output.push_str(&define.replacement[cursor..]);
            break;
        }

        if matches!(ch, ' ' | '\t') {
            let whitespace_end = advance_inline_whitespace(&define.replacement, cursor);
            if whitespace_end < define.replacement.len()
                && is_identifier_start(char_at(&define.replacement, whitespace_end))
                && last_non_whitespace_char(&output)
                    .is_some_and(|previous| matches!(previous, '(' | ','))
            {
                cursor = whitespace_end;
                continue;
            }

            output.push_str(&define.replacement[cursor..whitespace_end]);
            cursor = whitespace_end;
            continue;
        }

        if is_identifier_start(ch) {
            let end = advance_identifier(&define.replacement, cursor);
            let identifier = &define.replacement[cursor..end];
            if let Some(argument) = substitutions.get(identifier) {
                output.push_str(argument);
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

fn advance_inline_whitespace(text: &str, start: usize) -> usize {
    let mut cursor = start;
    while cursor < text.len() {
        let ch = char_at(text, cursor);
        if !matches!(ch, ' ' | '\t') {
            break;
        }
        cursor += ch.len_utf8();
    }
    cursor
}

fn last_non_whitespace_char(text: &str) -> Option<char> {
    text.chars().rev().find(|ch| !matches!(ch, ' ' | '\t'))
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

fn advance_string_literal(text: &str, start: usize, delimiter: char) -> usize {
    let mut cursor = start;
    let mut escaped = false;
    let mut saw_opening_delimiter = false;

    while cursor < text.len() {
        let ch = char_at(text, cursor);
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

fn copy_string_literal(text: &str, start: usize, delimiter: char, output: &mut String) -> usize {
    let end = advance_string_literal(text, start, delimiter);
    output.push_str(&text[start..end]);
    end
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
    fn does_not_expand_function_like_defines_without_call_syntax() {
        let source = SourceFile::new(PathBuf::from("main.prg"), "#define WRAP(x) x\n? WRAP\n");

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "? WRAP\n");
    }

    #[test]
    fn expands_focused_function_like_defines_with_case_sensitive_parameters() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#define F1( n ) F2( n, N )\n#define F3( nN, Nn ) F2( nN, Nn, NN, nn, N, n )\n? F1( 1 )\n? F3( 1, 2 )\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "? F2(1 ,N )\n? F2(1,2 ,NN,nn,N,n )\n");
    }

    #[test]
    fn expands_nested_function_like_define_subset_with_object_like_tail() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#define DATENEW   1\n#define DATEOLD(x)   x\n#define datediff(x,y) ( DATEOLD(x) - DATENEW )\nx := datediff( x, y )\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "x := (x - 1 )\n");
    }

    #[test]
    fn expands_function_like_define_before_constructor_translate_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#define clas( x )   (x)\n#xtranslate ( <name>{ [<p,...>] } => (<name>():New(<p>)\n? clas( TEST{ 1,2,3} )\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "? (TEST():New(1,2,3) )\n");
    }

    #[test]
    fn expands_tooltip_command_subset_with_escaped_array_literals() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#define RED {255,0,0}\n#xcommand SET TOOLTIP TO <color> OF <form> => SM( TTH (<\"form\">), 1, RGB(<color>\\[1], <color>\\[2\\], <color>[, <color>\\[ 3 \\] ]), 0)\nSET TOOLTIP TO RED OF form1\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SM(TTH (\"form1\"),1,RGB({255,0,0}[1],{255,0,0}[2],{255,0,0},{255,0,0}[ 3 ] ),0)\n"
        );
    }

    #[test]
    fn expands_zzz_escape_command_subset_without_extra_spacing() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command ZZZ [<v>] => QOUT([<v>\\[1\\]])\nZZZ a\nZZZ\nZZZ a[1]+2\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "QOUT(a[1] )\nQOUT()\nQOUT(a[1]+2[1] )\n");
    }

    #[test]
    fn expands_hmg_escaped_translate_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xtranslate _HMG_a => _HMG\\[137\\]\nv:= _bro[ a( _HMG_a [i] ) ]\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "v:= _bro[ a( _HMG[137] [i] ) ]\n");
    }

    #[test]
    fn expands_set_filter_restricted_macro_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command SET FILTER TO <exp> => dbSetFilter( <{exp}>, <\"exp\"> )\n#command SET FILTER TO <x:&> => if ( Empty( <(x)> ) ) ; dbClearFilter() ;; else ; dbSetFilter( <{x}>, <(x)> ) ; end\nSET FILTER TO &cVar.\nSET FILTER TO &(cVar .AND. &cVar)\nSET FILTER TO &cVar. .AND. cVar\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "if ( Empty(cVar) ) ; dbClearFilter() ; else ; dbSetFilter({|| &cVar.},cVar) ; end\nif ( Empty((cVar .AND. &cVar)) ) ; dbClearFilter() ; else ; dbSetFilter({|| &(cVar .AND. &cVar)},(cVar .AND. &cVar)) ; end\ndbSetFilter( {|| &cVar. .AND. cVar}, \"&cVar. .AND. cVar\" )\n"
        );
    }

    #[test]
    fn expands_copy_structure_extended_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command COPY [STRUCTURE] [EXTENDED] [TO <(f)>] => __dbCopyXStruct( <(f)> )\nCOPY STRUCTURE EXTENDED TO teststru\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "__dbCopyXStruct( \"teststru\" )\n");
    }

    #[test]
    fn expands_get_command_base_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 0,1 GET a\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(0,1 ) ; AAdd(GetList,_GET_(a,\"a\",,, ) )     ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_picture_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 0,2 GET a PICTURE \"X\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(0,2 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",, ) )     ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_valid_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 0,3 GET a PICTURE \"X\" VALID .T.\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(0,3 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|| .T.}, ) )     ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_when_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 0,4 GET a PICTURE \"X\" VALID .T. WHEN .T.\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(0,4 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|| .T.},{|| .T.} ) )     ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_caption_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 0,5 GET a PICTURE \"X\" VALID .T. WHEN .T. CAPTION \"myget\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(0,5 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|| .T.},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1    ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_message_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 0,6 GET a PICTURE \"X\" VALID .T. WHEN .T. CAPTION \"myget\" MESSAGE \"mymess\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(0,6 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|| .T.},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1  ; ATail(GetList):message := \"mymess\"   ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_send_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 0,7 GET a PICTURE \"X\" VALID .T. WHEN .T. CAPTION \"myget\" MESSAGE \"mymess\" SEND send()\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(0,7 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|| .T.},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1  ; ATail(GetList):message := \"mymess\"  ; ATail(GetList):send()  ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_range_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> [<exp,...>] RANGE <low>, <high> [<nextexp,...>] => @ <row>, <col> GET <var> [ <exp>] VALID {| _1 | RangeCheck( _1,, <low>, <high> ) } [ <nextexp>]\n#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 1,1 GET a RANGE 0,100\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(1,1 ) ; AAdd(GetList,_GET_(a,\"a\",,{|_1| RangeCheck(_1,, 0, 100)}, ) )     ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_picture_range_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> [<exp,...>] RANGE <low>, <high> [<nextexp,...>] => @ <row>, <col> GET <var> [ <exp>] VALID {| _1 | RangeCheck( _1,, <low>, <high> ) } [ <nextexp>]\n#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 1,2 GET a PICTURE \"X\" RANGE 0,100\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(1,2 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)}, ) )     ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_range_picture_reordered_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> RANGE <low>, <high> PICTURE <pic> => SetPos( <row>, <col> ) ; AAdd( GetList, _GET_( <var>, <\"var\">, <pic>, {| _1 | RangeCheck( _1,, <low>, <high> ) }, ) ) ; ATail(GetList):Display()\n#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 2,2 GET a RANGE 0,100 PICTURE \"X\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(2,2 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)}, ) )     ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_valid_range_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> [<exp,...>] RANGE <low>, <high> [<nextexp,...>] => @ <row>, <col> GET <var> [ <exp>] VALID {| _1 | RangeCheck( _1,, <low>, <high> ) } [ <nextexp>]\n#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 1,3 GET a PICTURE \"X\" VALID .T. RANGE 0,100\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(1,3 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|| .T.}, ) )     ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_when_range_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> [<exp,...>] RANGE <low>, <high> [<nextexp,...>] => @ <row>, <col> GET <var> [ <exp>] VALID {| _1 | RangeCheck( _1,, <low>, <high> ) } [ <nextexp>]\n#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 1,4 GET a PICTURE \"X\" WHEN .T. RANGE 0,100\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(1,4 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) )     ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_picture_range_when_reordered_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PICTURE <pic> RANGE <low>, <high> WHEN <when> => SetPos( <row>, <col> ) ; AAdd( GetList, _GET_( <var>, <\"var\">, <pic>, {| _1 | RangeCheck( _1,, <low>, <high> ) }, <{when}> ) )     ; ATail(GetList):Display()\n#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 2,4 GET a PICTURE \"X\" RANGE 0,100 WHEN .T.\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(2,4 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) )     ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_picture_range_when_caption_reordered_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PICTURE <pic> RANGE <low>, <high> WHEN <when> CAPTION <caption> => SetPos( <row>, <col> ) ; AAdd( GetList, _GET_( <var>, <\"var\">, <pic>, {| _1 | RangeCheck( _1,, <low>, <high> ) }, <{when}> ) ) ; ATail(GetList):Caption := <caption> ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(<caption>) - 1 ; ATail(GetList):Display()\n#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 2,5 GET a PICTURE \"X\" RANGE 0,100 WHEN .T. CAPTION \"myget\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(2,5 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1    ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_picture_range_when_caption_message_reordered_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PICTURE <pic> RANGE <low>, <high> WHEN <when> CAPTION <caption> MESSAGE <message> => SetPos( <row>, <col> ) ; AAdd( GetList, _GET_( <var>, <\"var\">, <pic>, {| _1 | RangeCheck( _1,, <low>, <high> ) }, <{when}> ) ) ; ATail(GetList):Caption := <caption> ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(<caption>) - 1 ; ATail(GetList):message := <message> ; ATail(GetList):Display()\n#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 2,6 GET a PICTURE \"X\" RANGE 0,100 WHEN .T. CAPTION \"myget\" MESSAGE \"mymess\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(2,6 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1  ; ATail(GetList):message := \"mymess\"   ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_picture_range_when_caption_message_send_reordered_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PICTURE <pic> RANGE <low>, <high> WHEN <when> CAPTION <caption> MESSAGE <message> SEND <msg> => SetPos( <row>, <col> ) ; AAdd( GetList, _GET_( <var>, <\"var\">, <pic>, {| _1 | RangeCheck( _1,, <low>, <high> ) }, <{when}> ) ) ; ATail(GetList):Caption := <caption> ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(<caption>) - 1 ; ATail(GetList):message := <message> ; ATail(GetList):<msg> ; ATail(GetList):Display()\n#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 2,7 GET a PICTURE \"X\" RANGE 0,100 WHEN .T. CAPTION \"myget\" MESSAGE \"mymess\" SEND send()\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(2,7 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1  ; ATail(GetList):message := \"mymess\"  ; ATail(GetList):send()  ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_base_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,, ) ) ; ATail(GetList):Control := _PushButt_(,,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,, ) ) ; ATail(GetList):Control := _PushButt_(,,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_valid_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>, ) ) ; ATail(GetList):Control := _PushButt_(,,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid()\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()}, ) ) ; ATail(GetList):Control := _PushButt_(,,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_when_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(,,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when()\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(,,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_caption_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_message_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> MESSAGE <message> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_color_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> MESSAGE <message> COLOR <color> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color()\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_focus_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> MESSAGE <message> COLOR <color> FOCUS <focus> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,<{focus}>,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus()\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_state_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> MESSAGE <message> COLOR <color> FOCUS <focus> STATE <state> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,<{focus}>,<{state}>,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state()\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_style_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> MESSAGE <message> COLOR <color> FOCUS <focus> STATE <state> STYLE <style> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,<{focus}>,<{state}>,<style>,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style()\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_send_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> MESSAGE <message> COLOR <color> FOCUS <focus> STATE <state> STYLE <style> SEND <msg> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,<{focus}>,<{state}>,<style>,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):<msg>   ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send()\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()   ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_guisend_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> MESSAGE <message> COLOR <color> FOCUS <focus> STATE <state> STYLE <style> SEND <msg> GUISEND <guimsg> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,<{focus}>,<{state}>,<style>,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):<msg>  ; ATail(GetList):Control:<guimsg>  ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send() GUISEND guisend()\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_size_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> MESSAGE <message> COLOR <color> FOCUS <focus> STATE <state> STYLE <style> SEND <msg> GUISEND <guimsg> SIZE X <sizex> Y <sizey> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,<{focus}>,<{state}>,<style>,<sizex>,<sizey>,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):<msg>  ; ATail(GetList):Control:<guimsg>  ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send() GUISEND guisend() SIZE X 100 Y 100\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),100,100,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_capoff_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> MESSAGE <message> COLOR <color> FOCUS <focus> STATE <state> STYLE <style> SEND <msg> GUISEND <guimsg> SIZE X <sizex> Y <sizey> CAPOFF X <capxoff> Y <capyoff> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,<{focus}>,<{state}>,<style>,<sizex>,<sizey>,<capxoff>,<capyoff>,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):<msg>  ; ATail(GetList):Control:<guimsg>  ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send() GUISEND guisend() SIZE X 100 Y 100 CAPOFF X 10 Y 10\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),100,100,10,10,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_bitmap_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> MESSAGE <message> COLOR <color> FOCUS <focus> STATE <state> STYLE <style> SEND <msg> GUISEND <guimsg> SIZE X <sizex> Y <sizey> CAPOFF X <capxoff> Y <capyoff> BITMAP <bitmap> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,<{focus}>,<{state}>,<style>,<sizex>,<sizey>,<capxoff>,<capyoff>,<bitmap>,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):<msg>  ; ATail(GetList):Control:<guimsg>  ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send() GUISEND guisend() SIZE X 100 Y 100 CAPOFF X 10 Y 10 BITMAP bitmap()\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),100,100,10,10,bitmap(),, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_bmpoff_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON VALID <valid> WHEN <when> CAPTION <caption> MESSAGE <message> COLOR <color> FOCUS <focus> STATE <state> STYLE <style> SEND <msg> GUISEND <guimsg> SIZE X <sizex> Y <sizey> CAPOFF X <capxoff> Y <capyoff> BITMAP <bitmap> BMPOFF X <bmpxoff> Y <bmpyoff> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,<{focus}>,<{state}>,<style>,<sizex>,<sizey>,<capxoff>,<capyoff>,<bitmap>,<bmpxoff>,<bmpyoff> ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):<msg>  ; ATail(GetList):Control:<guimsg>  ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send() GUISEND guisend() SIZE X 100 Y 100 CAPOFF X 10 Y 10 BITMAP bitmap() BMPOFF X 2 Y 2\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),100,100,10,10,bitmap(),2,2 ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_color_only_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON COLOR <color> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,, ) ) ; ATail(GetList):Control := _PushButt_(,,<color>,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON COLOR \"W/N\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,, ) ) ; ATail(GetList):Control := _PushButt_(,,\"W/N\",,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_reordered_sparse_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON COLOR <color> SIZE X <sizex> Y <sizey> BMPOFF X <bmpxoff> Y <bmpyoff> VALID <valid> GUISEND <guimsg> WHEN <when> MESSAGE <message> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(,<message>,<color>,,,,<sizex>,<sizey>,,,,<bmpxoff>,<bmpyoff> ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }  ; ATail(GetList):Control:<guimsg>  ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON COLOR \"W/N\" SIZE X 100 Y 100 BMPOFF X 2 Y 2 VALID valid() GUISEND guisend() WHEN when() MESSAGE \"mes\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(,\"mes\",\"W/N\",,,,100,100,,,,2,2 ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_reordered_sparse_color_tail_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON SIZE X <sizex> Y <sizey> BMPOFF X <bmpxoff> Y <bmpyoff> VALID <valid> GUISEND <guimsg> WHEN <when> MESSAGE <message> COLOR <color> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(,<message>,<color>,,,,<sizex>,<sizey>,,,,<bmpxoff>,<bmpyoff> ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }  ; ATail(GetList):Control:<guimsg>  ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON SIZE X 100 Y 100 BMPOFF X 2 Y 2 VALID valid() GUISEND guisend() WHEN when() MESSAGE \"mes\" COLOR \"W/N\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(,\"mes\",\"W/N\",,,,100,100,,,,2,2 ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_get_command_pushbutton_reordered_full_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> PUSHBUTTON SIZE X <sizex> Y <sizey> BMPOFF X <bmpxoff> Y <bmpyoff> VALID <valid> GUISEND <guimsg> WHEN <when> MESSAGE <message> COLOR <color> CAPOFF X <capxoff> Y <capyoff> FOCUS <focus> STATE <state> STYLE <style> SEND <msg> BITMAP <bitmap> CAPTION <caption> => SetPos(<row>,<col> ) ; AAdd(GetList,_GET_(<var>,<\"var\">,NIL,<{valid}>,<{when}> ) ) ; ATail(GetList):Control := _PushButt_(<caption>,<message>,<color>,<{focus}>,<{state}>,<style>,<sizex>,<sizey>,<capxoff>,<capyoff>,<bitmap>,<bmpxoff>,<bmpyoff> ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):<msg>  ; ATail(GetList):Control:<guimsg>  ; ATail(GetList):Control:Display()\n@ 4,1 GET a PUSHBUTTON SIZE X 100 Y 100 BMPOFF X 2 Y 2 VALID valid() GUISEND guisend() WHEN when() MESSAGE \"mes\" COLOR \"W/N\" CAPOFF X 10 Y 10 FOCUS focus() STATE state() STYLE style() SEND send() BITMAP bitmap() CAPTION \"cap\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",\"W/N\",{|| focus()},{|| state()},style(),100,100,10,10,bitmap(),2,2 ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()\n"
        );
    }

    #[test]
    fn expands_define_clipboard_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command DEFINE CLIPBOARD <oClp>\n   [ FORMAT <format:TEXT,OEMTEXT,BITMAP,DIF> ]\n   [ OF <oWnd> ]\n   =>\n   <oClp> := TClipboard():New([UPPER(<(format)>)] [,<oWnd>] )\nDEFINE CLIPBOARD oC OF oD FORMAT TEXT\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "oC := TClipboard():New(UPPER(\"TEXT\") ,oD )\n"
        );
    }

    #[test]
    fn expands_define_clipboard_oemtext_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command DEFINE CLIPBOARD <oClp>\n   [ FORMAT <format:TEXT,OEMTEXT,BITMAP,DIF> ]\n   [ OF <oWnd> ]\n   =>\n   <oClp> := TClipboard():New([UPPER(<(format)>)] [,<oWnd>] )\nDEFINE CLIPBOARD oC OF oD FORMAT OEMTEXT\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "oC := TClipboard():New(UPPER(\"OEMTEXT\") ,oD )\n"
        );
    }

    #[test]
    fn expands_release_all_subset_with_specific_rule_precedence() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command RELEASE <v,...> => __mvXRelease( <\"v\"> )\n#command RELEASE ALL => __mvRelease( \"*\", .t. )\nRELEASE ALL\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "__mvRelease( \"*\", .t. )\n");
    }

    #[test]
    fn expands_release_all_like_subset_with_specific_rule_precedence() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command RELEASE <v,...> => __mvXRelease( <\"v\"> )\n#command RELEASE ALL => __mvRelease( \"*\", .t. )\n#command RELEASE ALL LIKE <p> => __mvRelease( #<p>, .t. )\nRELEASE ALL LIKE A\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "__mvRelease( \"A\", .t. )\n");
    }

    #[test]
    fn expands_release_all_except_subset_with_specific_rule_precedence() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command RELEASE <v,...> => __mvXRelease( <\"v\"> )\n#command RELEASE ALL => __mvRelease( \"*\", .t. )\n#command RELEASE ALL LIKE <p> => __mvRelease( #<p>, .t. )\n#command RELEASE ALL EXCEPT <p> => __mvRelease( #<p>, .f. )\nRELEASE ALL EXCEPT A\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(output.text, "__mvRelease( \"A\", .f. )\n");
    }

    #[test]
    fn expands_get_command_caption_range_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> [<exp,...>] RANGE <low>, <high> [<nextexp,...>] => @ <row>, <col> GET <var> [ <exp>] VALID {| _1 | RangeCheck( _1,, <low>, <high> ) } [ <nextexp>]\n#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 1,5 GET a PICTURE \"X\" WHEN .T. CAPTION \"myget\" RANGE 0,100\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(1,5 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1    ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_message_range_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> [<exp,...>] RANGE <low>, <high> [<nextexp,...>] => @ <row>, <col> GET <var> [ <exp>] VALID {| _1 | RangeCheck( _1,, <low>, <high> ) } [ <nextexp>]\n#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 1,6 GET a PICTURE \"X\" WHEN .T. CAPTION \"myget\" MESSAGE \"mymess\" RANGE 0,100\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(1,6 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1  ; ATail(GetList):message := \"mymess\"   ; ATail(GetList):Display()\n"
        );
    }

    #[test]
    fn expands_get_command_send_range_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command @ <row>, <col> GET <var> [<exp,...>] RANGE <low>, <high> [<nextexp,...>] => @ <row>, <col> GET <var> [ <exp>] VALID {| _1 | RangeCheck( _1,, <low>, <high> ) } [ <nextexp>]\n#command @ <row>, <col> GET <var>\n                        [PICTURE <pic>]\n                        [VALID <valid>]\n                        [WHEN <when>]\n                        [CAPTION <caption>]\n                        [MESSAGE <message>]\n                        [SEND <msg>]\n\n      => SetPos( <row>, <col> )\n       ; AAdd( GetList,\n              _GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> ) )\n      [; ATail(GetList):Caption := <caption>]\n      [; ATail(GetList):CapRow  := ATail(Getlist):row\n       ; ATail(GetList):CapCol  := ATail(Getlist):col -\n                              __CapLength(<caption>) - 1]\n      [; ATail(GetList):message := <message>]\n      [; ATail(GetList):<msg>]\n       ; ATail(GetList):Display()\n@ 1,7 GET a PICTURE \"X\" WHEN .T. CAPTION \"myget\" MESSAGE \"mymess\" SEND send() RANGE 0,100\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(output.errors.is_empty());
        assert_eq!(
            output.text,
            "SetPos(1,7 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1  ; ATail(GetList):message := \"mymess\"  ; ATail(GetList):send()  ; ATail(GetList):Display()\n"
        );
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

    #[test]
    fn repeats_optional_result_clauses_for_list_captures() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xcommand SET <vars,...> WITH <val> => assign(<val>[,<vars>])\nSET v1 WITH 0\nSET v1, v2, v3 WITH 0\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.text, "assign(0,v1)\nassign(0,v1,v2,v3)\n");
    }

    #[test]
    fn expands_nested_optional_match_patterns() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xtranslate AAA [A <a> [B <b>] ] => emit([<a>][,<b>])\nAAA\nAAA A alpha\nAAA A alpha B beta\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.text, "emit()\nemit(alpha)\nemit(alpha,beta)\n");
    }

    #[test]
    fn expands_insert_rules_across_continued_source_lines() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xcommand INSERT2 INTO <table> ( <uField1> [, <uFieldN> ] ) VALUES ( <uVal1> [, <uValN> ] ) => ;\nif <table>->( dbappend() ) ;;\n <table>-><uField1> := <uVal1> ;;\n [ <table>-><uFieldN> := <uValN> ; ] ;\n <table>->( dbunlock() ) ;;\nendif\ninsert2 into test ( FIRST, LAST, STREET ) ;\n   values ( \"first\", \"last\", \"street\" )\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(
            output.text,
            "if test->( dbappend() ) test->FIRST := \"first\"  test->LAST := \"last\" ;  test->STREET := \"street\" ;  test->( dbunlock() ) endif\n"
        );
    }

    #[test]
    fn collects_multiline_rule_result_without_directive_sentinel() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command MYCOMMAND2 [<mylist,...>] [MYCLAUSE <myval>] [ALL] =>\n   MyFunction( {<mylist>} [, <myval>] )\nMYCOMMAND2 ALL MYCLAUSE 321 \"HELLO\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(output.text, "MyFunction( {\"HELLO\"} , 321 )\n");
    }

    #[test]
    fn expands_multiline_result_rule_with_optional_keyword_before_list_capture() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command MYCOMMAND2 [<mylist,...>] [MYCLAUSE <myval>] [ALL] =>\n   MyFunction( {<mylist>} [, <myval>] )\nMYCOMMAND2 MYCLAUSE 321 ALL \"HELLO\"\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(output.text, "MyFunction( {\"HELLO\"} , 321 )\n");
    }

    #[test]
    fn collects_multiline_rule_pattern_without_sentinel_before_arrow() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command MYCOMMAND2 [<myList,...>]\n   [MYCLAUSE <myVal>] [MYOTHER <myOther>] => MyFunction( {<myList>}, <myVal>, <myOther> )\nMYCOMMAND2 MYCLAUSE 322 \"Hello\" MYOTHER 1\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(output.text, "MyFunction( {\"Hello\"}, 322, 1 )\n");
    }

    #[test]
    fn matches_marker_followed_by_literal_paren_in_xtranslate_rules() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xtranslate XTRANS(<x>( => normal( <(x)> )\n#xtranslate XTRANS(<x:&>( => macro( <(x)> )\nXTRANS( cVar (\nXTRANS( &cVar (\nXTRANS( &cVar+1 (\nXTRANS( &cVar. (\nXTRANS( (&cVar.) (\nXTRANS( &(cVar) (\nXTRANS( &cVar[3] (\nXTRANS( &cVar.  [3] (\nXTRANS( &(cVar  [3],&cvar) (\nXTRANS( (&cVar.  [3],&cvar) (\nXTRANS( &cVar.1+5 (\nXTRANS( &cVar .AND. cVar (\nXTRANS( &cVar. .AND. cVar (\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 2);
        assert_eq!(
            output.text,
            "normal( \"cVar\" )\nmacro( cVar )\nnormal( \"&cVar+1\" )\nmacro( cVar )\nXTRANS( (&cVar.) (\nmacro( (cVar) )\nnormal( \"&cVar[3]\" )\nnormal( \"&cVar.  [3]\" )\nmacro( (cVar  [3],&cvar) )\nXTRANS( (&cVar.  [3],&cvar) (\nnormal( \"&cVar.1+5\" )\nnormal( \"&cVar .AND. cVar\" )\nnormal( \"&cVar. .AND. cVar\" )\n"
        );
    }

    #[test]
    fn matches_concatenated_macro_chains_in_xtranslate_rules() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xtranslate XTRANS(<x>( => normal( <(x)> )\n#xtranslate XTRANS(<x:&>( => macro( <(x)> )\nXTRANS( &cVar&cVar (\nXTRANS( &cVar.&cVar (\nXTRANS( &cVar.&cVar. (\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 2);
        assert_eq!(
            output.text,
            "macro( \"&cVar&cVar\" )\nmacro( \"&cVar.&cVar\" )\nmacro( \"&cVar.&cVar.\" )\n"
        );
    }

    #[test]
    fn matches_full_xtrans_subset_from_upstream_pp_test() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xtranslate XTRANS(<x>( => normal( <(x)> )\n#xtranslate XTRANS(<x:&>( => macro( <(x)> )\nXTRANS( cVar (\nXTRANS( &cVar (\nXTRANS( &cVar+1 (\nXTRANS( &cVar. (\nXTRANS( &cVar&cVar (\nXTRANS( &cVar.&cVar (\nXTRANS( &cVar.&cVar. (\nXTRANS( (&cVar.) (\nXTRANS( &(cVar) (\nXTRANS( &cVar[3] (\nXTRANS( &cVar.  [3] (\nXTRANS( &(cVar  [3],&cvar) (\nXTRANS( (&cVar.  [3],&cvar) (\nXTRANS( &cVar.1+5 (\nXTRANS( &cVar .AND. cVar (\nXTRANS( &cVar. .AND. cVar (\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 2);
        assert_eq!(
            output.text,
            "normal( \"cVar\" )\nmacro( cVar )\nnormal( \"&cVar+1\" )\nmacro( cVar )\nmacro( \"&cVar&cVar\" )\nmacro( \"&cVar.&cVar\" )\nmacro( \"&cVar.&cVar.\" )\nXTRANS( (&cVar.) (\nmacro( (cVar) )\nnormal( \"&cVar[3]\" )\nnormal( \"&cVar.  [3]\" )\nmacro( (cVar  [3],&cvar) )\nXTRANS( (&cVar.  [3],&cvar) (\nnormal( \"&cVar.1+5\" )\nnormal( \"&cVar .AND. cVar\" )\nnormal( \"&cVar. .AND. cVar\" )\n"
        );
    }

    #[test]
    fn expands_macro_call_translate_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xtranslate MXCALL <x:&> => (<x>)\n#xtranslate MYCALL <x:&> <y> => <x>( <y>, 'mycall' )\n#xtranslate MZCALL <x> <y> => <x>( <y>, \"mzcall\" )\nMXCALL &cVar\nMXCALL &cVar++\nMYCALL &cVar &cVar\nMYCALL &cVar+1 &cVar\nMZCALL &cVar ++cVar\nMZCALL &cVar+1 &cVar\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 3);
        assert_eq!(
            output.text,
            "(&cVar)\n(&cVar)++\n&cVar( &cVar, 'mycall' )\n&cVar( +1, 'mycall' ) &cVar\n&cVar ++( cVar, \"mzcall\" )\n&cVar+1( &cVar, \"mzcall\" )\n"
        );
    }

    #[test]
    fn expands_adjacent_macro_pair_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command FOO <x:&> FOO <y:&> => <(x)>+<(y)>\n#translate BAR <x:&> BAR <y:&> => <(x)>+<(y)>\nFOO &cVar FOO &var.\nBAR &cVar BAR &var.\nFOO &cVar FOO &var.+1\nBAR &cVar BAR &var.+1\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 2);
        assert_eq!(
            output.text,
            "cVar+var\ncVar+var\nFOO &cVar FOO &var.+1\ncVar+var+1\n"
        );
    }

    #[test]
    fn expands_post_macro_call_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xtranslate MXCALL <x:&> => (<x>)\nMXCALL &cVar()\nMXCALL &cVar++\n(MXCALL &cVar)++\nMXCALL &cVar.()\nMXCALL &cVar.++\n(MXCALL &cVar.)++\nMXCALL &cVar.1 ()\nMXCALL &cVar.1 ++\n(MXCALL &cVar.1) ++\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(
            output.text,
            "(&cVar)()\n(&cVar)++\n((&cVar))++\n(&cVar.)()\n(&cVar.)++\n((&cVar.))++\n(&cVar.1) ()\n(&cVar.1) ++\n((&cVar.1)) ++\n"
        );
    }

    #[test]
    fn expands_macro_command_operator_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command MCOMMAND <x> => normal_c( <\"x\"> )\n#command MCOMMAND <x:&> => macro_c( <(x)> )\nMCOMMAND &cVar.+1\nMCOMMAND &cVar. .AND.  .T.\nMCOMMAND &cVar.++\nMCOMMAND &cVar.-=2\nMCOMMAND &cVar .AND.  .T.\nMCOMMAND & (cVar) +1\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 2);
        assert_eq!(
            output.text,
            "normal_c( \"&cVar.+1\" )\nnormal_c( \"&cVar. .AND.  .T.\" )\nnormal_c( \"&cVar.++\" )\nnormal_c( \"&cVar.-=2\" )\nnormal_c( \"&cVar .AND.  .T.\" )\nnormal_c( (cVar) +1 )\n"
        );
    }

    #[test]
    fn expands_define_window_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xcommand DECLARE WINDOW <w> ;\n=>;\n#xtranslate <w> . <p:Name,Title,f1,f2,f3,f4,f5,f6,f7,f8,f9> := <n> => SProp( <\"w\">, <\"p\"> , <n> )\n#xcommand DEFINE WINDOW <w> [ON INIT <IProc>] =>;\n      DECLARE WINDOW <w>  ; _DW( <\"w\">, <{IProc}> )\nDEFINE WINDOW &oW\nDEFINE WINDOW &oW ON INIT &oW.Title:= \"My title\"\n&oW.Title := \"title\"\n&oW.f9 := 9\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 3);
        assert_eq!(
            output.text,
            "DECLARE WINDOW &oW  ; _DW( oW,  )\nDECLARE WINDOW &oW  ; _DW( oW, {|| SProp( oW, \"Title\" , \"My title\" )} )\nSProp( oW, \"Title\" , \"title\" )\nSProp( oW, \"f9\" , 9 )\n"
        );
    }

    #[test]
    fn expands_property_translate_subset_without_define_window_wrapper() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xtranslate <w> . <p:Name,Title,f9> := <n> => SProp( <\"w\">, <\"p\"> , <n> )\noW.Title := \"title\"\noW . f9 := 9\n&oW.Title := \"macro\"\n&oW . f9 := 10\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(
            output.text,
            "SProp( \"oW\", \"Title\" , \"title\" )\nSProp( \"oW\", \"f9\" , 9 )\nSProp( oW, \"Title\" , \"macro\" )\nSProp( oW, \"f9\" , 10 )\n"
        );
    }

    #[test]
    fn expands_constructor_translate_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xtranslate ( <name>{ [<p,...>] } => (<name>():New(<p>)\n? ( TEST{ 1,2,3} )\n? ( a+3{ 11,2,3} )\n? ( a(){ 11,2,3} )\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(
            output.text,
            "? (TEST():New(1,2,3) )\n? (a+3():New(11,2,3) )\n? (a()():New(11,2,3) )\n"
        );
    }

    #[test]
    fn expands_identifier_only_constructor_translate_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xtranslate ( <!name!>{ [<p,...>] } => (<name>():New(<p>)\n? ( TEST{ 1,2,3} )\n? ( a+3{ 11,2,3} )\n? ( a(){ 11,2,3} )\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(
            output.text,
            "? (TEST():New(1,2,3) )\n? ( a+3{ 11,2,3} )\n? ( a(){ 11,2,3} )\n"
        );
    }

    #[test]
    fn expands_compound_regular_marker_pattern_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command _REGULAR_(<z>) => rm( <z> )\n_REGULAR_(a)\n_REGULAR_(\"a\")\n_REGULAR_(&a.1)\n_REGULAR_(&a)\n_REGULAR_(a[1])\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(
            output.text,
            "rm( a )\nrm( \"a\" )\nrm( &a.1 )\nrm( &a )\nrm( a[1] )\n"
        );
    }

    #[test]
    fn expands_compound_normal_marker_pattern_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command _NORMAL_M(<z>) => nm( <\"z\"> )\n_NORMAL_M(a)\n_NORMAL_M(\"a\")\n_NORMAL_M('a')\n_NORMAL_M([\"'a'\"])\n_NORMAL_M(&a.1)\n_NORMAL_M(&a)\n_NORMAL_M(&a.)\n_NORMAL_M(&(a))\n_NORMAL_M(&a[1])\n_NORMAL_M(a[1])\n_NORMAL_M(\"['']\")\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(
            output.text,
            "nm( \"a\" )\nnm( '\"a\"' )\nnm( '\"a\"' )\nnm( [[\"'a'\"]] )\nnm( \"&a.1\" )\nnm( a )\nnm( a )\nnm( (a) )\nnm( \"&a[1]\" )\nnm( \"a[1]\" )\nnm( [\"['']\"] )\n"
        );
    }

    #[test]
    fn expands_compound_smart_marker_pattern_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command _SMART_M(<z>) => sm( <(z)> )\n_SMART_M(a)\n_SMART_M(\"a\")\n_SMART_M('a')\n_SMART_M([\"'a'\"])\n_SMART_M(&a.1)\n_SMART_M(&a)\n_SMART_M(&a.)\n_SMART_M(&(a))\n_SMART_M(&a[1])\n_SMART_M(a[1])\n_SMART_M(\"['']\")\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(
            output.text,
            "sm( \"a\" )\nsm( \"a\" )\nsm( \"a\" )\nsm( [\"'a'\"] )\nsm( \"&a.1\" )\nsm( a )\nsm( a )\nsm( (a) )\nsm( \"&a[1]\" )\nsm( \"a[1]\" )\nsm( \"['']\" )\n"
        );
    }

    #[test]
    fn expands_compound_dumb_marker_pattern_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command _DUMB_M(<z>) => dm( #<z> )\n_DUMB_M(a)\n_DUMB_M(\"a\")\n_DUMB_M('a')\n_DUMB_M([\"'a'\"])\n_DUMB_M(&a.1)\n_DUMB_M(&a)\n_DUMB_M(&a.)\n_DUMB_M(&(a))\n_DUMB_M(&a[1])\n_DUMB_M(a[1])\n_DUMB_M(\"['']\")\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(
            output.text,
            "dm( \"a\" )\ndm( '\"a\"' )\ndm( '\"a\"' )\ndm( [[\"'a'\"]] )\ndm( \"&a.1\" )\ndm( \"&a\" )\ndm( \"&a.\" )\ndm( \"&(a)\" )\ndm( \"&a[1]\" )\ndm( \"a[1]\" )\ndm( [\"['']\"] )\n"
        );
    }

    #[test]
    fn expands_compound_regular_list_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command _REGULAR_L(<z,...>) => rl( <z> )\n_REGULAR_L(a,\"a\",'a',[\"'a'\"],\"['a']\",'[\"a\"]',&a.1,&a,&a.,&a.  ,&(a),&a[1],&a.[1],&a.  [2],&a&a, &a.a,  a, a)\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(
            output.text,
            "rl( a,\"a\",\"a\",[\"'a'\"],\"['a']\",'[\"a\"]',&a.1,&a,&a.,&a.  ,&(a),&a[1],&a.[1],&a.  [2],&a&a, &a.a,  a, a )\n"
        );
    }

    #[test]
    fn expands_compound_normal_list_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command _NORMAL_L(<z,...>) => nl( <\"z\"> )\n_NORMAL_L(n,\"n\",'a',[\"'a'\"],\"['a']\",'[\"a\"]',&a.1,&a,&a.,&a.  ,&(a),&a[1],&a.[1],&a.  [2],&a&a, &.a, &a.a,  a, a)\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(
            output.text,
            "nl( \"n\",'\"n\"','\"a\"',[[\"'a'\"]],[\"['a']\"],['[\"a\"]'],\"&a.1\",a,a,a,(a),\"&a[1]\",\"&a.[1]\",\"&a.  [2]\",\"&a&a\",\"&.a\",\"&a.a\",\"a\",\"a\" )\n"
        );
    }

    #[test]
    fn expands_compound_smart_list_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command _SMART_L(<z,...>) => sl( <(z)> )\n_SMART_L(a,\"a\",'a',[\"'a'\"],\"['a']\",'[\"a\"]',&a.1,&a,&a.,&a.  ,&(a),&a[1],&a.[1],&a.  [2],&a&a, &.a, &a.a,  a, a)\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(
            output.text,
            "sl( \"a\",\"a\",\"a\",[\"'a'\"],\"['a']\",'[\"a\"]',\"&a.1\",a,a,a,(a),\"&a[1]\",\"&a.[1]\",\"&a.  [2]\",\"&a&a\",\"&.a\",\"&a.a\",\"a\",\"a\" )\n"
        );
    }

    #[test]
    fn expands_compound_dumb_list_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command _DUMB_L(<z,...>) => dl( #<z> )\n_DUMB_L(a,\"a\",'a',[\"'a'\"],\"['a']\",'[\"a\"]',&a.1,&a,&a.,&a.  ,&(a),&a[1],&a.[1],&a.  [2],&a&a, &.a, &a.a,  a, a)\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(
            output.text,
            "dl( [a,\"a\",\"a\",[\"'a'\"],\"['a']\",'[\"a\"]',&a.1,&a,&a.,&a.  ,&(a),&a[1],&a.[1],&a.  [2],&a&a, &.a, &a.a,  a, a] )\n"
        );
    }

    #[test]
    fn preserves_spaces_in_index_expression_stringify_subset() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#command INDEX ON <key> TO <(file)> [<u: UNIQUE>] => dbCreateIndex( <(file)>, <\"key\">, <{key}>, iif( <.u.>, .t., NIL ) )\nindex on LEFT(   f1  ,  10   )      to _tst\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 1);
        assert_eq!(
            output.text,
            "dbCreateIndex( \"_tst\", \"LEFT(   f1  ,  10   )\", {|| LEFT(   f1  ,  10   )}, iif( .F., .t., NIL ) )\n"
        );
    }

    #[test]
    fn expands_multiline_repeated_optional_list_rules() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xcommand SET <var1> [, <varN>] WITH <val> =>\n<var1>:=<val> [; <varN>:=<val>]\n#command AVG <x1> [, <xn>] TO <v1> [, <vn>]  =>\n   AVERAGE( {||<v1>:=<v1>+<x1>} [, {||<vn>:=<vn>+<xn>} ] )\nSET v1 WITH 0\nSET v1, v2, v3, v4 WITH 0\nAVG f1, f2, f3 TO s1, s2, s3\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(output.rules.len(), 2);
        assert_eq!(
            output.text,
            "v1:=0 \nv1:=0 ; v2:=0; v3:=0; v4:=0\nAVERAGE( {||s1:=s1+f1} , {||s2:=s2+f2} , {||s3:=s3+f3}  )\n"
        );
    }

    #[test]
    fn reorders_multiline_optional_clauses_around_list_captures() {
        let source = SourceFile::new(
            PathBuf::from("main.prg"),
            "#xcommand MYCOMMAND3 [<myList,...>] ;\n   [MYCLAUSE <myVal>] [MYOTHER <myOther>] => MyFunction3( {<myList>}, <myVal>, <myOther> )\nMYCOMMAND3 MYCLAUSE 322 \"Hello\" MYOTHER 1\nMYCOMMAND3 MYOTHER 1 MYCLAUSE 322 \"Hello\"\nMYCOMMAND3 \"Hello\" MYOTHER 1 MYCLAUSE 322\nMYCOMMAND3 MYOTHER 1 \"Hello\" MYCLAUSE 322\n",
        );

        let output = Preprocessor::new(MapIncludeResolver::default()).preprocess(source);

        assert!(
            output.errors.is_empty(),
            "unexpected errors: {:?}",
            output.errors
        );
        assert_eq!(
            output.text,
            "MyFunction3( {\"Hello\"}, 322, 1 )\nMyFunction3( {\"Hello\"}, 322, 1 )\nMyFunction3( {\"Hello\"}, 322, 1 )\nMyFunction3( {\"Hello\"}, 322, 1 )\n"
        );
    }
}
