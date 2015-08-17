use std::fmt;
use parser::lexer::position::Position;

#[derive(Copy,Clone)]
pub struct CharPos {
    pub value: char,
    pub position: Position
}

impl fmt::Display for CharPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "char {} at line {:?}", self.value, self.position)
    }
}
