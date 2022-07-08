use std::collections::HashMap;

use crate::error::RuntimeError;
use crate::error::RuntimeErrorKind;
use crate::Executable;
use crate::OpKind;

#[derive(Default)]
pub struct Runtime {
    memory: HashMap<String, usize>,
    stack: Vec<usize>,
    pins: HashMap<String, usize>,
}

impl Runtime {
    pub fn execute(&mut self, exe: Executable) -> Result<(), RuntimeError> {
        for (i, op) in exe.ops.iter().enumerate() {
            if let OpKind::Pin(label) = &op.kind {
                if self.pins.get(label).is_some() {
                    return Err(exe.throw_at(RuntimeErrorKind::DuplicatePin(label.clone()), op, 2));
                }
                self.pins.insert(label.clone(), i);
            }
        }
        let mut i = 0;
        while i < exe.ops.len() {
            let op = &exe.ops[i];
            match &op.kind {
                OpKind::Pin(_) => (),
                OpKind::Mov(key, value) => {
                    self.memory.insert(key.clone(), *value);
                }
                OpKind::Add(key_a, key_b) => {
                    let val_a = self.memory.get(key_a);
                    if val_a.is_none() {
                        return Err(exe.throw_at(
                            RuntimeErrorKind::Undefined(key_a.clone()),
                            op,
                            1,
                        ));
                    }
                    let val_b = self.memory.get(key_b);
                    if val_b.is_none() {
                        return Err(exe.throw_at(
                            RuntimeErrorKind::Undefined(key_b.clone()),
                            op,
                            2,
                        ));
                    }
                    self.memory
                        .insert(key_a.clone(), val_a.unwrap() + val_b.unwrap());
                }
                OpKind::Sub(key_a, key_b) => {
                    let val_a = self.memory.get(key_a);
                    if val_a.is_none() {
                        return Err(exe.throw_at(
                            RuntimeErrorKind::Undefined(key_a.clone()),
                            op,
                            1,
                        ));
                    }
                    let val_b = self.memory.get(key_b);
                    if val_b.is_none() {
                        return Err(exe.throw_at(
                            RuntimeErrorKind::Undefined(key_b.clone()),
                            op,
                            2,
                        ));
                    }
                    self.memory
                        .insert(key_a.clone(), val_a.unwrap() - val_b.unwrap());
                }
                OpKind::Cmp(key, value) => {
                    let val = self.memory.get(key);
                    if val.is_none() {
                        return Err(exe.throw_at(RuntimeErrorKind::Undefined(key.clone()), op, 1));
                    }
                    self.memory
                        .insert("#".to_string(), if *val.unwrap() == *value { 1 } else { 0 });
                }
                OpKind::Jif(label, value) => {
                    let code = self.memory.remove("#");
                    if code.is_none() {
                        return Err(exe.throw_at(RuntimeErrorKind::NoCompare, op, 0));
                    }
                    if code.unwrap() == *value {
                        let pos = self.pins.get(label);
                        if pos.is_none() {
                            return Err(exe.throw_at(
                                RuntimeErrorKind::NoPin(label.clone()),
                                op,
                                1,
                            ));
                        }
                        self.stack.push(i);
                        i = *pos.unwrap();
                    }
                }
                OpKind::Out(key) => {
                    let val = self.memory.get(key);
                    if val.is_none() {
                        return Err(exe.throw_at(RuntimeErrorKind::Undefined(key.clone()), op, 1));
                    }
                    print!("{}", val.unwrap());
                }
                OpKind::Utf(key) => {
                    let val = self.memory.get(key);
                    if val.is_none() {
                        return Err(exe.throw_at(RuntimeErrorKind::Undefined(key.clone()), op, 1));
                    }
                    print!("{}", String::from_utf8_lossy(&[*val.unwrap() as u8]));
                }
                OpKind::Jmp(label) => {
                    let pos = self.pins.get(label);
                    if pos.is_none() {
                        return Err(exe.throw_at(RuntimeErrorKind::NoPin(label.clone()), op, 1));
                    }
                    self.stack.push(i);
                    i = *pos.unwrap();
                }
                OpKind::Ret => {
                    let pos = self.stack.pop();
                    if pos.is_none() {
                        return Err(exe.throw_at(RuntimeErrorKind::NoReturn, op, 0));
                    }
                    i = pos.unwrap();
                }
                OpKind::End => {
                    break;
                }
            }
            i += 1;
        }
        Ok(())
    }
}
