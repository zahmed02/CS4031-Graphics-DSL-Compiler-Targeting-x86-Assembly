use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Token {
    // Keywords
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("loop")]
    Loop,
    #[token("times")]
    Times,
    #[token("then")]
    Then,
    #[token("end")]
    End,
    #[token("draw_pixel")]
    DrawPixel,
    #[token("draw_rect")]
    DrawRect,
    #[token("clear_screen")]
    ClearScreen,
    #[token("delay")]
    Delay,
    #[token("wait_key")]
    WaitKey,

    // Identifiers & literals
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[regex(r"[0-9]+")]
    Integer,

    // Operators & punctuation
    #[token("=")]
    Assign,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token(",")]
    Comma,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    // Comparison operators
    #[token("<")]
    Less,
    #[token(">")]
    Greater,

    // Trivia – skipped
    #[regex(r"[ \t\r\n]+", logos::skip)]
    Whitespace,
    #[regex(r"//[^\n]*", logos::skip)]
    Comment,
}

pub fn tokenize(source: &str) -> Vec<(Token, String)> {
    let mut lex = Token::lexer(source);
    let mut tokens = Vec::new();
    while let Some(result) = lex.next() {
        if let Ok(token) = result {
            let slice = lex.slice();
            tokens.push((token, slice.to_string()));
        }
    }
    tokens
}