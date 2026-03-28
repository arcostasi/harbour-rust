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
}

struct PreprocessState<'a, R> {
    include_resolver: &'a R,
    output: String,
    defines: Vec<DefineDirective>,
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
            line_origins: Vec::new(),
            errors: Vec::new(),
            include_stack: Vec::new(),
        }
    }

    fn finish(self) -> PreprocessOutput {
        PreprocessOutput {
            text: self.output,
            defines: self.defines,
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

        for (index, raw_line) in split_lines(&source.text).enumerate() {
            let line_number = index + 1;
            match parse_directive(&source.path, raw_line, line_number) {
                None => {
                    let (expanded_line, mut errors) = expand_object_like_defines(
                        raw_line,
                        &self.defines,
                        &source.path,
                        line_number,
                    );
                    self.errors.append(&mut errors);
                    self.push_output_line(&source.path, line_number, &expanded_line);
                }
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
                Some(Err(error)) => self.errors.push(error),
            }
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
        _ => Err(directive_error(
            path,
            line_number,
            directive_column,
            format!("unsupported preprocessor directive '#{}'", keyword),
        )),
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
}
