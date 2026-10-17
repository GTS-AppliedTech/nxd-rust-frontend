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

    pub fn next_is(
        &self,
        kind: TokenKind,
    ) -> bool {
        match self.peek(1) {
            Some(token) => token.kind == kind,
            None => false,
        }
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
        &mut self,
    ) -> Result<AstImport, ParserError> {

        let import_token = self.eat(
            Some(TokenKind::Keyword),
            Some("IMPORT"),
        )?;

        let path_token = self.eat(
            Some(TokenKind::Ident),
            None,
        )?;

        let mut alias = None;
        let mut end_span = path_token.span();

        if self.at(
            Some(TokenKind::Op),
            Some("AS"),
        ) {
            self.eat(
                Some(TokenKind::Op),
                Some("AS"),
            )?;

            let alias_token = self.eat(
                Some(TokenKind::Ident),
                None,
            )?;

            end_span = alias_token.span();
            alias = Some(alias_token.lexeme);
        }

        Ok(AstImport {
            path: path_token.lexeme,
            alias,
            span: SourceSpan {
                line: import_token.line,
                column: import_token.column,
                end_line: end_span.end_line,
                end_column: end_span.end_column,
            },
        })
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

        self.eat(
            Some(TokenKind::Keyword),
            Some("TYPE"),
        )?;

        let name_token = self.eat(
            Some(TokenKind::Ident),
            None,
        )?;

        let name = name_token.lexeme;

        if self.at(
            Some(TokenKind::Keyword),
            Some("ENUM"),
        ) {
            self.eat(
                Some(TokenKind::Keyword),
                Some("ENUM"),
            )?;

            return self.parse_enum_type(name);
        }

        if self.at(
            Some(TokenKind::Keyword),
            Some("UNION"),
        ) {
            self.eat(
                Some(TokenKind::Keyword),
                Some("UNION"),
            )?;

            return self.parse_union_type(name);
        }

        if self.at(
            Some(TokenKind::Keyword),
            Some("TRAIT"),
        ) {
            self.eat(
                Some(TokenKind::Keyword),
                Some("TRAIT"),
            )?;

            return self.parse_trait_type(name);
        }

        if self.at(
            Some(TokenKind::Keyword),
            Some("STRUCT"),
        ) {
            self.eat(
                Some(TokenKind::Keyword),
                Some("STRUCT"),
            )?;

            return self.parse_struct_type(name);
        }

        todo!("NXD-P1003");
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

    pub fn parse_enum_type(
        &mut self,
        name: String,
    ) -> Result<AstNode, ParserError> {

        self.eat(
            Some(TokenKind::Colon),
            None,
        )?;

        while self.at(
            Some(TokenKind::Newline),
            None,
        ) {
            self.eat(
                Some(TokenKind::Newline),
                None,
            )?;
        }

        let mut variants: Vec<String> = Vec::new();

        while self.at(
            Some(TokenKind::Ident),
            None,
        ) {

            let variant = self.eat(
                Some(TokenKind::Ident),
                None,
            )?;

            variants.push(
                variant.lexeme,
            );

            if self.at(
                Some(TokenKind::Comma),
                None,
            ) {
                self.eat(
                    Some(TokenKind::Comma),
                    None,
                )?;
            }

            while self.at(
                Some(TokenKind::Newline),
                None,
            ) {
                self.eat(
                    Some(TokenKind::Newline),
                    None,
                )?;
            }
        }

        Ok(
            AstNode::Enum(
                AstEnum {
                    name,
                    variants,
                }
            )
        )
    }

    pub fn parse_union_type(
        &mut self,
        name: String,
    ) -> Result<AstNode, ParserError> {
        self.eat(
            Some(TokenKind::Colon),
            None,
        )?;

        while self.at(
            Some(TokenKind::Newline),
            None,
        ) {
            self.eat(
                Some(TokenKind::Newline),
                None,
            )?;
        }

        let mut variants: Vec<AstUnionVariant> = Vec::new();

        while self.at(
            Some(TokenKind::Ident),
            None,
        ) {
            let kind_token = self.eat(
                Some(TokenKind::Ident),
                None,
            )?;

            let kind = kind_token.lexeme;

            self.eat(
                Some(TokenKind::LBrace),
                None,
            )?;

            let mut fields: Vec<AstField> = Vec::new();

            while self.at(
                Some(TokenKind::Newline),
                None,
            ) {
                self.eat(
                    Some(TokenKind::Newline),
                    None,
                )?;
            }

            while !self.at(
                Some(TokenKind::RBrace),
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

                let field_name_token = self.eat(
                    Some(TokenKind::Ident),
                    None,
                )?;

                self.eat(
                    Some(TokenKind::Colon),
                    None,
                )?;

                let field_type = self.parse_type_ref()?;

                fields.push(AstField {
                    name: field_name_token.lexeme,
                    ty: field_type,
                });

                if self.at(
                    Some(TokenKind::Comma),
                    None,
                ) {
                    self.eat(
                        Some(TokenKind::Comma),
                        None,
                    )?;
                }
            }

            self.eat(
                Some(TokenKind::RBrace),
                None,
            )?;

            variants.push(AstUnionVariant {
                kind,
                fields,
            });

            while self.at(
                Some(TokenKind::Newline),
                None,
            ) {
                self.eat(
                    Some(TokenKind::Newline),
                    None,
                )?;
            }
        }

        Ok(AstNode::Union(AstUnion {
            name,
            variants,
        }))
    }

    pub fn parse_trait_type(
        &mut self,
        name: String,
    ) -> Result<AstNode, ParserError> {
        self.eat(
            Some(TokenKind::Colon),
            None,
        )?;

        while self.at(
            Some(TokenKind::Newline),
            None,
        ) {
            self.eat(
                Some(TokenKind::Newline),
                None,
            )?;
        }

        let mut methods: Vec<AstFunctionSignature> = Vec::new();

        while self.at(
            Some(TokenKind::Keyword),
            Some("FUNC"),
        ) {
            let func_token = self.eat(
                Some(TokenKind::Keyword),
                Some("FUNC"),
            )?;

            let method_name_token = self.eat(
                Some(TokenKind::Ident),
                None,
            )?;

            let method_name = method_name_token.lexeme;

            self.eat(
                Some(TokenKind::LParen),
                None,
            )?;

            let mut params: Vec<String> = Vec::new();

            if !self.at(
                Some(TokenKind::RParen),
                None,
            ) {
                let parameter_token = self.eat(
                    Some(TokenKind::Ident),
                    None,
                )?;

                params.push(parameter_token.lexeme);

                while self.at(
                    Some(TokenKind::Comma),
                    None,
                ) {
                    self.eat(
                        Some(TokenKind::Comma),
                        None,
                    )?;

                    let parameter_token = self.eat(
                        Some(TokenKind::Ident),
                        None,
                    )?;

                    params.push(parameter_token.lexeme);
                }
            }

            let rparen_token = self.eat(
                Some(TokenKind::RParen),
                None,
            )?;

            let mut return_type: Option<String> = None;
            let mut end_span = rparen_token.span();

            if self.at(
                Some(TokenKind::Colon),
                None,
            ) {
                self.eat(
                    Some(TokenKind::Colon),
                    None,
                )?;

                let return_type_token = if self.at(
                    Some(TokenKind::LowType),
                    None,
                ) {
                    self.eat(
                        Some(TokenKind::LowType),
                        None,
                    )?
                } else {
                    self.eat(
                        Some(TokenKind::Ident),
                        None,
                    )?
                };

                end_span = return_type_token.span();
                return_type = Some(return_type_token.lexeme);
            }

            methods.push(AstFunctionSignature {
                name: method_name,
                span: SourceSpan {
                    line: func_token.line,
                    column: func_token.column,
                    end_line: end_span.end_line,
                    end_column: end_span.end_column,
                },
                params,
                return_type,
            });

            while self.at(
                Some(TokenKind::Newline),
                None,
            ) {
                self.eat(
                    Some(TokenKind::Newline),
                    None,
                )?;
            }
        }

        Ok(AstNode::Trait(AstTrait {
            name,
            methods,
        }))
    }

    pub fn parse_struct_type(
        &mut self,
        name: String,
    ) -> Result<AstNode, ParserError> {
        self.eat(
            Some(TokenKind::Colon),
            None,
        )?;

        while self.at(
            Some(TokenKind::Newline),
            None,
        ) {
            self.eat(
                Some(TokenKind::Newline),
                None,
            )?;
        }

        let mut fields: Vec<AstField> = Vec::new();

        while self.at(
            Some(TokenKind::Ident),
            None,
        ) && self.next_is(TokenKind::Colon)
        {
            let field_name_token = self.eat(
                Some(TokenKind::Ident),
                None,
            )?;

            self.eat(
                Some(TokenKind::Colon),
                None,
            )?;

            let field_type = self.parse_type_ref()?;

            if self.at(
                Some(TokenKind::Comma),
                None,
            ) {
                self.eat(
                    Some(TokenKind::Comma),
                    None,
                )?;
            }

            fields.push(AstField {
                name: field_name_token.lexeme,
                ty: field_type,
            });

            while self.at(
                Some(TokenKind::Newline),
                None,
            ) {
                self.eat(
                    Some(TokenKind::Newline),
                    None,
                )?;
            }
        }

        Ok(AstNode::Struct(
            AstStruct {
                name,
                fields,
            },
        ))
    }


    pub fn parse_type_ref(
        &mut self,
    ) -> Result<String, ParserError> {

        if self.at(
            Some(TokenKind::LowType),
            None,
        ) {
            let token = self.eat(
                Some(TokenKind::LowType),
                None,
            )?;

            return Ok(token.lexeme);
        }

        let token = self.eat(
            Some(TokenKind::Ident),
            None,
        )?;

        Ok(token.lexeme)
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