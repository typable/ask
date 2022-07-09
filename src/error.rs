use std::fmt;

use crate::color;
use crate::Color;
use crate::Key;
use crate::Label;
use crate::Pos;

#[derive(Debug)]
pub struct CompileError {
    pub kind: CompileErrorKind,
    pub line: String,
    pub pos: Pos,
    pub len: usize,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (y, x) = self.pos;
        let message = match self.kind {
            CompileErrorKind::UnexpectedChar(c) => format!("Unexpected character '{}'!", c),
            CompileErrorKind::InvalidLocation => "Invalid location!".to_string(),
            CompileErrorKind::UnknownOp => "Unknown operation!".to_string(),
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
    pub pos: Pos,
    pub len: usize,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (y, x) = self.pos;
        let message = match &self.kind {
            RuntimeErrorKind::Undefined(key) => format!("'{}' is not defined!", key),
            RuntimeErrorKind::NoCompare => "'cmp' operation before expected!".to_string(),
            RuntimeErrorKind::NoPin(label) => format!("No pin with name '{}' found!", label),
            RuntimeErrorKind::DuplicatePin(label) => {
                format!("Pin with name '{}' already in use!", label)
            }
            RuntimeErrorKind::NoReturn => "No pin to jump back to!".to_string(),
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
pub enum CompileErrorKind {
    UnexpectedChar(char),
    UnknownOp,
    InvalidLocation,
}

#[derive(Debug)]
pub enum RuntimeErrorKind {
    Undefined(Key),
    NoCompare,
    NoPin(Label),
    DuplicatePin(Label),
    NoReturn,
}
