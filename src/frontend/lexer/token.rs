use crate::frontend::lexer::token_kind::TokenKind;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: u32,
    pub column: u32,
}