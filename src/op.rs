use crate::error::PerformError;
use crate::error::RuntimeErrorKind::*;
use crate::unwrap_or_throw;
use crate::Label;
use crate::Op;
use crate::PerformResult;
use crate::Pos;
use crate::Ref;
use crate::Runtime;
use crate::Token;

pub struct OpWrap {
    pub op: Box<dyn Op>,
    pub tokens: Vec<Token>,
    pub pre_init: bool,
}

pub struct Pin(pub Label);
pub struct Mov(pub Pos, pub Ref);
pub struct Add(pub Pos, pub Ref);
pub struct Sub(pub Pos, pub Ref);
pub struct Cmp(pub Pos, pub Ref);
pub struct Jif(pub Label);
pub struct Jel(pub Label);
pub struct Jmp(pub Label);
pub struct Out(pub Ref);
pub struct Utf(pub Ref);
pub struct Ret;
pub struct End;

impl Op for Pin {
    fn perform(&self, runtime: &mut Runtime) -> PerformResult {
        let label = &self.0;
        if runtime.pins.get(label).is_some() {
            return Err(PerformError(DuplicatePin(label.clone())));
        }
        runtime.pins.insert(label.clone(), runtime.index);
        Ok(())
    }
}

impl Op for Mov {
    fn perform(&self, runtime: &mut Runtime) -> PerformResult {
        let pos = self.0.clone();
        let refer = &self.1;
        match refer {
            Ref::Pos(pos_old) => {
                let value =
                    unwrap_or_throw!(runtime.memory.get(pos_old), PerformError(Undefined(pos)));
                runtime.memory.insert(pos, *value);
            }
            Ref::Value(value) => {
                runtime.memory.insert(pos, *value);
            }
        }
        Ok(())
    }
}

impl Op for Add {
    fn perform(&self, runtime: &mut Runtime) -> PerformResult {
        let pos = &self.0;
        let value = unwrap_or_throw!(
            runtime.memory.get(pos),
            PerformError(Undefined(pos.clone()))
        );
        let refer_a = &self.1;
        match refer_a {
            Ref::Pos(pos_old) => {
                let value_old = unwrap_or_throw!(
                    runtime.memory.get(pos_old),
                    PerformError(Undefined(pos_old.clone()))
                );
                runtime.memory.insert(pos.clone(), *value + value_old);
            }
            Ref::Value(value_old) => {
                runtime.memory.insert(pos.clone(), value + *value_old);
            }
        }
        Ok(())
    }
}

impl Op for Sub {
    fn perform(&self, runtime: &mut Runtime) -> PerformResult {
        let pos = &self.0;
        let value = unwrap_or_throw!(
            runtime.memory.get(pos),
            PerformError(Undefined(pos.clone()))
        );
        let refer_a = &self.1;
        match refer_a {
            Ref::Pos(pos_old) => {
                let value_old = unwrap_or_throw!(
                    runtime.memory.get(pos_old),
                    PerformError(Undefined(pos_old.clone()))
                );
                runtime.memory.insert(pos.clone(), *value - value_old);
            }
            Ref::Value(value_old) => {
                runtime.memory.insert(pos.clone(), value - *value_old);
            }
        }
        Ok(())
    }
}

impl Op for Cmp {
    fn perform(&self, runtime: &mut Runtime) -> PerformResult {
        let pos = &self.0;
        let value = unwrap_or_throw!(
            runtime.memory.get(pos),
            PerformError(Undefined(pos.clone()))
        );
        let refer_a = &self.1;
        match refer_a {
            Ref::Pos(pos_old) => {
                let value_old = unwrap_or_throw!(
                    runtime.memory.get(pos_old),
                    PerformError(Undefined(pos_old.clone()))
                );
                runtime
                    .memory
                    .insert("#".to_string(), if *value == *value_old { 1 } else { 0 });
            }
            Ref::Value(value_old) => {
                runtime
                    .memory
                    .insert("#".to_string(), if *value == *value_old { 1 } else { 0 });
            }
        }
        Ok(())
    }
}

impl Op for Jif {
    fn perform(&self, runtime: &mut Runtime) -> PerformResult {
        let code = unwrap_or_throw!(runtime.memory.get("#"), PerformError(NoCompare));
        if *code == 1 {
            let label = &self.0;
            let pos = unwrap_or_throw!(runtime.pins.get(label), PerformError(NoPin(label.clone())));
            runtime.stack.push(runtime.index);
            runtime.index = *pos;
        }
        Ok(())
    }
}

impl Op for Jel {
    fn perform(&self, runtime: &mut Runtime) -> PerformResult {
        let code = unwrap_or_throw!(runtime.memory.get("#"), PerformError(NoCompare));
        if *code == 0 {
            let label = &self.0;
            let pos = unwrap_or_throw!(runtime.pins.get(label), PerformError(NoPin(label.clone())));
            runtime.stack.push(runtime.index);
            runtime.index = *pos;
        }
        Ok(())
    }
}

impl Op for Jmp {
    fn perform(&self, runtime: &mut Runtime) -> PerformResult {
        let label = &self.0;
        let pos = unwrap_or_throw!(runtime.pins.get(label), PerformError(NoPin(label.clone())));
        runtime.stack.push(runtime.index);
        runtime.index = *pos;
        Ok(())
    }
}

impl Op for Out {
    fn perform(&self, runtime: &mut Runtime) -> PerformResult {
        let refer = &self.0;
        match refer {
            Ref::Pos(pos) => {
                let value = unwrap_or_throw!(
                    runtime.memory.get(pos),
                    PerformError(Undefined(pos.clone()))
                );
                print!("{}", value);
            }
            Ref::Value(value) => {
                print!("{}", value);
            }
        }
        Ok(())
    }
}

impl Op for Utf {
    fn perform(&self, runtime: &mut Runtime) -> PerformResult {
        let refer = &self.0;
        match refer {
            Ref::Pos(pos) => {
                let value = unwrap_or_throw!(
                    runtime.memory.get(pos),
                    PerformError(Undefined(pos.clone()))
                );
                print!("{}", String::from_utf8_lossy(&[*value as u8]));
            }
            Ref::Value(value) => {
                print!("{}", String::from_utf8_lossy(&[*value as u8]));
            }
        }
        Ok(())
    }
}

impl Op for Ret {
    fn perform(&self, runtime: &mut Runtime) -> PerformResult {
        let pos = unwrap_or_throw!(runtime.stack.pop(), PerformError(NoReturn));
        runtime.index = pos;
        Ok(())
    }
}

impl Op for End {
    fn perform(&self, runtime: &mut Runtime) -> PerformResult {
        runtime.end = true;
        Ok(())
    }
}
