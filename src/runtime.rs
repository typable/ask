use std::collections::HashMap;

use crate::error::PerformError;
use crate::error::RuntimeError;
use crate::error::RuntimeErrorKind::*;
use crate::op::OpWrap;
use crate::Executable;
use crate::Label;
use crate::Pos;
use crate::TokenKind;
use crate::Value;

#[derive(Default)]
pub struct Runtime {
    pub memory: HashMap<Pos, Value>,
    pub stack: Vec<usize>,
    pub pins: HashMap<Label, usize>,
    pub index: usize,
    pub end: bool,
}

impl Runtime {
    pub fn execute(&mut self, exe: Executable) -> Result<(), RuntimeError> {
        self.index = 0;
        for opwrap in &exe.ops {
            if opwrap.pre_init {
                if let Err(err) = opwrap.op.perform(self) {
                    return Err(self.throw_error(&exe, opwrap, err));
                }
            }
            self.index += 1;
        }
        self.index = 0;
        while self.index < exe.ops.len() && !self.end {
            let opwrap = &exe.ops[self.index];
            if !opwrap.pre_init {
                if let Err(err) = opwrap.op.perform(self) {
                    return Err(self.throw_error(&exe, opwrap, err));
                }
            }
            self.index += 1;
        }
        Ok(())
    }
    fn throw_error(&self, exe: &Executable, opwrap: &OpWrap, err: PerformError) -> RuntimeError {
        let arg = match err.0 {
            NoPin(_) => 1,
            DuplicatePin(_) => 1,
            _ => 0,
        };
        let token = &opwrap.tokens[arg];
        let (y, _) = token.clone().pos;
        let lines = exe.raw.lines().collect::<Vec<_>>();
        let length = match &token.kind {
            TokenKind::Symbol(symbol) => symbol.len(),
            _ => 1,
        };
        RuntimeError {
            kind: err.0,
            line: lines[y].to_string(),
            loc: token.clone().pos,
            len: length,
        }
    }
}
