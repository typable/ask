mod color;
mod compiler;
mod runtime;

pub mod error;
pub mod op;

use std::str::FromStr;

use error::CompileError;
use error::PerformError;
use op::OpWrap;

pub use color::Color;
pub use compiler::Compiler;
pub use runtime::Runtime;

#[macro_export]
macro_rules! unwrap_or_throw {
    ($option:expr, $err:expr) => {
        match $option {
            Some(x) => x,
            None => return Err($err),
        }
    };
}

pub type Label = String;
pub type Pos = String;
pub type Value = usize;

pub type PerformResult = Result<(), PerformError>;

pub trait Op {
    fn perform(&self, runtime: &mut Runtime) -> PerformResult;
}

#[derive(Debug, Clone)]
pub enum Ref {
    Pos(Pos),
    Value(usize),
}

impl FromStr for Ref {
    type Err = CompileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<Value>();
        if let Ok(value) = value {
            return Ok(Ref::Value(value));
        }
        Ok(Ref::Pos(s.to_string()))
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Symbol(String),
    Value(String),
    Break,
    Pin,
    Comment,
    Cast(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: (usize, usize),
}

pub struct Executable {
    pub ops: Vec<OpWrap>,
    pub raw: String,
}
