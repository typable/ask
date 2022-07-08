use std::fmt;

use crate::color;
use crate::Color;
use crate::Key;
use crate::Label;
use crate::Pos;

pub struct CompileError;

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
pub enum RuntimeErrorKind {
    Undefined(Key),
    NoCompare,
    NoPin(Label),
    DuplicatePin(Label),
}
