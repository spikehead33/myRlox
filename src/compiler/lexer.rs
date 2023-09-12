use crate::location::Location;

pub struct Token<'a, 'b> {
    pub location: Location<'a>,
    pub kind: TokenKind,
    lexeme: &'b str
}

pub enum TokenKind {
}

pub struct Lexer<'a, 'b> {
    pub current_position: Location<'a>,
    pub tokens: Vec<Token<'a, 'b>>,
}

impl<'a, 'b> Lexer<'a, 'b> {
    pub fn lex(&self) {
        todo!()
    }

    pub fn tokens(&self) -> &Vec<Token<'a, 'b>> {
        &self.tokens
    }
}