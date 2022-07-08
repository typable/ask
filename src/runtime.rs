use std::collections::HashMap;

use crate::error::RuntimeError;
use crate::error::RuntimeErrorKind;
use crate::Executable;
use crate::Op;
use crate::OpKind;
use crate::TokenKind;

#[derive(Default)]
pub struct Runtime {
    memory: HashMap<String, usize>,
    pins: HashMap<String, usize>,
}

impl Runtime {
    pub fn execute(&mut self, executable: Executable) -> Result<(), RuntimeError> {
        let mut i = 0;
        while i < executable.ops.len() {
            let op = &executable.ops[i];
            match &op.kind {
                OpKind::Pin(label) => {
                    self.pins.insert(label.clone(), i);
                }
                OpKind::Mov(key, value) => {
                    self.memory.insert(key.clone(), *value);
                }
                OpKind::Add(key_a, key_b) => {
                    let val_a = self.memory.get(key_a).unwrap();
                    let val_b = self.memory.get(key_b).unwrap();
                    self.memory.insert(key_a.clone(), val_a + val_b);
                }
                OpKind::Sub(key_a, key_b) => {
                    let val_a = self.memory.get(key_a).unwrap();
                    let val_b = self.memory.get(key_b).unwrap();
                    self.memory.insert(key_a.clone(), val_a - val_b);
                }
                OpKind::Cmp(key, value) => {
                    let val = self.memory.get(key);
                    if val.is_none() {
                        return Err(throw_error(
                            RuntimeErrorKind::Undefined(key.clone()),
                            op,
                            &executable.raw,
                            1,
                        ));
                    }
                    self.memory
                        .insert("#".to_string(), if *val.unwrap() == *value { 1 } else { 0 });
                }
                OpKind::Jmp(label, value) => {
                    let code = self.memory.remove("#");
                    if code.is_none() {
                        return Err(throw_error(
                            RuntimeErrorKind::NoCompare,
                            op,
                            &executable.raw,
                            0,
                        ));
                    }
                    if code.unwrap() == *value {
                        i = *self.pins.get(label).unwrap();
                    }
                }
                OpKind::Out(key) => {
                    let val = self.memory.get(key);
                    if val.is_none() {
                        return Err(throw_error(
                            RuntimeErrorKind::Undefined(key.clone()),
                            op,
                            &executable.raw,
                            1,
                        ));
                    }
                    print!("{}", val.unwrap());
                }
                OpKind::Utf(key) => {
                    let val = self.memory.get(key);
                    if val.is_none() {
                        return Err(throw_error(
                            RuntimeErrorKind::Undefined(key.clone()),
                            op,
                            &executable.raw,
                            1,
                        ));
                    }
                    print!("{}", String::from_utf8_lossy(&[*val.unwrap() as u8]));
                }
            }
            i += 1;
        }
        Ok(())
    }
}

fn throw_error(kind: RuntimeErrorKind, op: &Op, raw: &str, index: usize) -> RuntimeError {
    let token = &op.tokens[index];
    let (y, _) = token.clone().pos;
    let lines = raw.lines().collect::<Vec<_>>();
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
