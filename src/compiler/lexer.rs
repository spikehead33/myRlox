use crate::location::Location;
use crate::utils::Resettable;
use crate::CompilerError;

pub struct Token<'a> {
    pub location: Location,
    pub kind: TokenKind,
    pub lexeme: &'a str,
}

pub enum TokenKind {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // relational operator
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literal
    Identifier,
    String,
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,

    Eof,
}

pub struct Lexer {
    pub filename: String,
    pub source: String,
    pub current_position: Location,
}

impl Lexer {
    pub fn new(filename: String, source: String) -> Self {
        Self {
            filename,
            source,
            current_position: Location(filename, 0, 0),
        }
    }

    pub fn get_token(&self) -> Option<Token> {
        todo!()
    }
}
