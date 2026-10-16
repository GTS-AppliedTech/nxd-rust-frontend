use crate::frontend::lexer::scanner::lex;
use crate::frontend::lexer::token::Token;
use crate::frontend::ast::nodes::*;
use crate::frontend::lexer::token_kind::TokenKind;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub code: String,
    pub message: String,
    pub span: SourceSpan,
    pub severity: DiagnosticSeverity,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticSeverity {
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
    pub warnings: Vec<WarningDiagnostic>,
    pub recover_errors: bool,
}

impl Parser {

    // Constructor

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
            warnings: Vec::new(),
            recover_errors,
        })
    }

    // Navigation helpers

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.pos + offset)
    }

    pub fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    pub fn at_end(&self) -> bool {
        match self.current() {
            Some(token) => token.kind == TokenKind::Eof,
            None => true,
        }
    }

    // Token checking helpers

    pub fn at(
        &self,
        kind: Option<TokenKind>,
        lexeme: Option<&str>,
    ) -> bool {
        let Some(token) = self.current() else {
            return false;
        };

        if let Some(expected_kind) = kind {
            if token.kind != expected_kind {
                return false;
            }
        }

        if let Some(expected_lexeme) = lexeme {
            if token.lexeme != expected_lexeme {
                return false;
            }
        }

        true
    }

    pub fn eat(
        &mut self,
        kind: Option<TokenKind>,
        lexeme: Option<&str>,
    ) -> Result<Token, ParserError> {
        let token = self.current()
            .cloned()
            .ok_or_else(|| {
                ParserError::new(
                    "NXD-P1000",
                    "Unexpected end of input",
                    SourceSpan {
                        line: 0,
                        column: 0,
                        end_line: 0,
                        end_column: 0,
                    },
                )
            })?;
        if !self.at(kind, lexeme) {
            return Err(
                ParserError::new(
                    "NXD-P1001",
                    format!(
                        "unexpected token '{}'",
                        token.lexeme
                    ),
                    token.span(),
                )
            );
        }

        self.advance();

        Ok(token)
    }

    // Parser entry points

    pub fn parse_program(
        &mut self,
    ) -> Result<AstModule, ParserError> {
        while self.at(
            Some(TokenKind::Newline),
            None,
        ) {
            self.eat(
                Some(TokenKind::Newline),
                None,
            )?;
        }
        if self.at(
            Some(TokenKind::Keyword),
            Some("MODULE"),
        ) {
            return self.parse_module();
        }
        
        let mut imports: Vec<AstImport> = Vec::new();
        let mut body: Vec<AstNode> = Vec::new();

        while !self.at(
            Some(TokenKind::Eof),
            None,
        ) {
            if self.at(
                Some(TokenKind::Newline),
                None,
            ) {
                self.eat(
                    Some(TokenKind::Newline),
                    None,
                )?;

                continue;
            }

            if self.at(
                Some(TokenKind::Keyword),
                Some("IMPORT"),
            ) {
                imports.push(self.parse_import()?);
                continue;   
            }

            let node = self.parse_top_level()?;
                body.push(node);
        }

        Ok(AstModule {
            name: "ANONYMOUS".to_string(),
            imports,
            body,
        })
    }

    // Top level grammer

    pub fn parse_module(
        &mut self,
    ) -> Result<AstModule, ParserError> {
        todo!()
    }

    pub fn parse_import(
        &mut self
    ) -> Result<AstImport, ParserError> {
        todo!()
    }

    pub fn parse_top_level(
    &mut self,
    ) -> Result<AstNode, ParserError> {
        if self.at(
            Some(TokenKind::Keyword),
            Some("TYPE"),
        ) {
            return self.parse_type_decl();
        }

        if self.at(
            Some(TokenKind::Keyword),
            Some("TRAIT"),
        ) {
            return self.parse_trait_decl();
        }

        if self.at(
            Some(TokenKind::Keyword),
            Some("IMPL"),
        ) {
            return self.parse_impl_decl();
        }

        if self.at(
            Some(TokenKind::Keyword),
            Some("ASYNC"),
        ) {
            self.eat(
                Some(TokenKind::Keyword),
                Some("ASYNC"),
            )?;

            let is_async = true;

            return self.parse_func_decl(is_async);
        }

        if self.at(
            Some(TokenKind::Keyword),
            Some("FUNC"),
        ) {
            let is_async = false;

            return self.parse_func_decl(is_async);
        }

        self.parse_statement()
    }

    // Declarations

    pub fn parse_type_decl(
    &mut self,
    ) -> Result<AstNode, ParserError> {
        todo!()
    }

    pub fn parse_trait_decl(
        &mut self,
    ) -> Result<AstNode, ParserError> {
        todo!()
    }

    pub fn parse_impl_decl(
        &mut self,
    ) -> Result<AstNode, ParserError> {
        todo!()
    }

    pub fn parse_func_decl(
        &mut self,
        is_async: bool,
    ) -> Result<AstNode, ParserError> {
        todo!()
    }


    pub fn parse_struct() {
        todo!()
    }

    pub fn parse_enum() {
        todo!()
    }

    // Statements

    pub fn parse_if() {
        todo!()
    }

    pub fn parse_loop() {
        todo!()
    }

    pub fn parse_match() {
        todo!()
    }

    pub fn parse_statement(
        &mut self,
    ) -> Result<AstNode, ParserError> {
        todo!()
    }

    // Expressions

    pub fn parse_expr() {
        todo!()
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