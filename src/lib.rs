mod color;
mod compiler;
mod runtime;

pub mod error;

use error::RuntimeError;
use error::RuntimeErrorKind;

pub use color::Color;
pub use compiler::Compiler;
pub use runtime::Runtime;

// #[macro_export]
// macro_rules! def {
//     ($($e:tt)*) => {
//         {
//             let expr = stringify!($($e)*);
//             let mut list = expr.split(',').map(|x| x.trim()).collect::<Vec<_>>();
//             list.pop();
//             let raw = list.join("\n");
//             let compiler = ask::Compiler;
//             let tokens = compiler.tokenize(&raw);
//             compiler.parse(tokens)
//         }
//     }
// }

pub type Label = String;
pub type Key = String;
pub type Value = usize;
pub type Pos = (usize, usize);

#[derive(Debug, Clone)]
pub enum OpKind {
    Pin(Label),
    Mov(Key, Value),
    Add(Key, Key),
    Sub(Key, Key),
    Cmp(Key, Value),
    Jif(Label, Value),
    Out(Key),
    Utf(Key),
    Jmp(Label),
    Ret,
    End,
}

#[derive(Debug, Clone)]
pub struct Op {
    kind: OpKind,
    tokens: Vec<Token>,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Symbol(String),
    Value(String),
    Break,
    Pin,
    Comment,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Pos,
}

#[derive(Debug, Clone)]
pub struct Executable {
    pub ops: Vec<Op>,
    pub raw: String,
}

impl Executable {
    pub fn throw_at(&self, kind: RuntimeErrorKind, op: &Op, index: usize) -> RuntimeError {
        let token = &op.tokens[index];
        let (y, _) = token.clone().pos;
        let lines = self.raw.lines().collect::<Vec<_>>();
        let length = match &token.kind {
            TokenKind::Symbol(symbol) => symbol.len(),
            _ => 1,
        };
        RuntimeError {
            kind,
            line: lines[y].to_string(),
            pos: token.clone().pos,
            len: length,
        }
    }
}
