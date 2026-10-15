use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

pub fn strip_line_comments(source: &str) -> String {

    // Removes NXD // line comments while preserving:
    // - newline characters
    // - // inside string literals
    // - escaped quotation marks inside strings


}

pub fn lex(src: &str) -> Vec<Token> {
    let src = strip_line_comments(src);

    println!("=== STRIPPED SOURCE ===");
    println!("{}", src);
    println!("=======================");

    let mut tokens = Vec::new();

    // TODO

    tokens.push(Token {
        kind: TokenKind::Eof,
        lexeme: String::new(),
        line: 1,
        column: 1,
    });

    tokens
}