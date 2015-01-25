use std::fmt;
use std::result::Result;

use lexer::position::Position;

#[derive(PartialEq,Show,Copy)]
pub enum TokenType {
    String, Number, Identifier, End,

    // Keywords
    Fn, Var, While, If, Else,
    Loop, Break, Continue, Return, Int,

    // Operators
    Add, Sub, Mul, Div, Mod, Not,
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    Comma, Semicolon, Dot, Assign,
    Eq, NEq, LThan, LEq, GThan, GEq
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub position: Position
}

impl Token {
    pub fn new( tok: TokenType, pos: Position ) -> Token {
        Token { token_type: tok, value: "".to_string(), position: pos }
    }

    pub fn is_eof(&self) -> bool {
        self.token_type == TokenType::End
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "type {:?} (with value {:?}) at {:?}", self.token_type, self.value, self.position)
    }
}

impl fmt::Show for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "type {:?} (with value {:?}) at {:?}", self.token_type, self.value, self.position)
    }
}

#[test]
fn test_new() {
    let tok = Token::new(TokenType::End, Position::new(1, 1));
    assert_eq!(format!("{:?}", tok).as_slice(), "type End (with value \"\") at 1:1");
}
