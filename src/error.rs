use std::fmt;

use crate::color;
use crate::Color;
use crate::Label;
use crate::Pos;

#[derive(Debug)]
pub struct CompileError {
    pub kind: CompileErrorKind,
    pub line: String,
    pub pos: (usize, usize),
    pub len: usize,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CompileErrorKind::*;
        let (y, x) = self.pos;
        let message = match self.kind {
            UnexpectedChar(c) => format!("Unexpected character '{}'!", c),
            InvalidLocation => "Invalid location!".to_string(),
            UnknownOp => "Unknown operation!".to_string(),
            ExpectedArgument => "Expected argument!".to_string(),
            InvalidBlock => "Invalid block!".to_string(),
            InvalidCast => "Invalid cast!".to_string(),
        };
        write!(
            f,
            "\n{}: {}\n {: <digit$} {}\n{} {}\n {: <digit$} {} {} {}\n",
            color!("CompileError", Color::BrightRed),
            color!(message, Color::BrightWhite),
            "",
            color!("|", Color::BrightBlue),
            color!(format!(" {} |", y + 1), Color::BrightBlue),
            self.line,
            "",
            color!("|", Color::BrightBlue),
            color!(
                format!("{: >width$}", "^".repeat(self.len), width = x - 1),
                Color::BrightRed
            ),
            color!(message, Color::BrightRed),
            digit = (y + 1).to_string().len(),
        )
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    pub line: String,
    pub loc: (usize, usize),
    pub len: usize,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RuntimeErrorKind::*;
        let (y, x) = self.loc;
        let message = match &self.kind {
            Undefined(key) => format!("'{}' is not defined!", key),
            NoCompare => "'cmp' operation before expected!".to_string(),
            NoPin(label) => format!("No pin with name '{}' found!", label),
            DuplicatePin(label) => {
                format!("Pin with name '{}' already in use!", label)
            }
            NoReturn => "No pin to jump back to!".to_string(),
        };
        write!(
            f,
            "\n{}: {}\n {: <digit$} {}\n{} {}\n {: <digit$} {} {} {}\n",
            color!("RuntimeError", Color::BrightRed),
            color!(message, Color::BrightWhite),
            "",
            color!("|", Color::BrightBlue),
            color!(format!(" {} |", y + 1), Color::BrightBlue),
            self.line,
            "",
            color!("|", Color::BrightBlue),
            color!(
                format!("{: >width$}", "^".repeat(self.len), width = x - 1),
                Color::BrightRed
            ),
            color!(message, Color::BrightRed),
            digit = (y + 1).to_string().len(),
        )
    }
}

#[derive(Debug)]
pub struct PerformError(pub RuntimeErrorKind);

#[derive(Debug)]
pub enum CompileErrorKind {
    UnexpectedChar(char),
    UnknownOp,
    InvalidLocation,
    ExpectedArgument,
    InvalidBlock,
    InvalidCast,
}

#[derive(Debug)]
pub enum RuntimeErrorKind {
    Undefined(Pos),
    NoCompare,
    NoPin(Label),
    DuplicatePin(Label),
    NoReturn,
}
