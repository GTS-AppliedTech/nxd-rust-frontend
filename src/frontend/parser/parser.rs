use crate::frontend::lexer::scanner::lex;
use crate::frontend::lexer::token::Token;
use crate::frontend::ast::nodes::SourceSpan;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub code: String,
    pub message: String,
    pub span: SourceSpan,
    pub severity: DiagnosticSeverity,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DIagnosticSeverity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct WarningDiagnostic {
    pub code: String,
    pub message: String,
    pub span: SourceSpan,
    pub severity: DiagnosticSeverity,
}

impl ParserError {
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
        span: SourceSpan,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            span,
            severity: DiagnosticSeverity::Error,
        }
    }
}

impl WarningDiagnostic {
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
        span: SourceSpan,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            span,
            severity: DiagnosticSeverity::Warning,
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
    pub indent_stack: Vec<usize>,
    pub errors: Vec<ParserError>,
    pub warnings: Vec<WarningDiagnostic>
    pub recover_errors: bool,
}

impl Parser {
    pub fn new(
        src: &str,
        recover_errors: bool,
    ) -> Result<Self, String> {
        let tokens = lex(src)?;

        Ok(Self {
            tokens,
            pos: 0,
            indent_stack: vec![0],
            errors: Vec::new(),
            recover_errors,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_parser_from_valid_source() {
        let parser = Parser::new(
            "LET VALUE 42\nRETURN VALUE",
            false,
        )
        .expect("Parser should accept valid lexer input");

        assert_eq!(parser.pos, 0);
        assert_eq!(parser.indent_stack, vec![0]);
        assert!(parser.errors.is_empty());
        assert!(!parser.recover_errors);
        assert!(!parser.tokens.is_empty());
    }

    #[test]
    fn propagates_lexer_errors() {
        let result = Parser::new("LET VALUE @", false);

        assert!(result.is_err());
    }
}