use std::str::FromStr;

use crate::error::CompileError;
use crate::error::CompileErrorKind;
use crate::op::Add;
use crate::op::Cmp;
use crate::op::End;
use crate::op::Jel;
use crate::op::Jif;
use crate::op::Jmp;
use crate::op::Mov;
use crate::op::OpWrap;
use crate::op::Out;
use crate::op::Pin;
use crate::op::Ret;
use crate::op::Sub;
use crate::op::Utf;
use crate::unwrap_or_throw;
use crate::Executable;
use crate::Op;
use crate::Ref;
use crate::Token;
use crate::TokenKind;

#[derive(Default)]
pub struct Compiler {
    raw: String,
}

impl Compiler {
    pub fn compile(&mut self, raw: &str) -> Result<Executable, CompileError> {
        self.raw = raw.to_string();
        let tokens = self.tokenize()?;
        Ok(Executable {
            ops: self.parse(&tokens)?,
            raw: raw.to_string(),
        })
    }
    fn tokenize(&self) -> Result<Vec<Token>, CompileError> {
        let mut tokens = Vec::new();
        let mut symbol = Vec::new();
        let mut value = Vec::new();
        let mut y = 0;
        let mut x = 0;
        for c in self.raw.chars() {
            match c {
                c if c.is_alphabetic() || c == '_' => {
                    symbol.push(c);
                }
                c if c.is_alphanumeric() => {
                    value.push(c);
                }
                ' ' | '\t' | '\r' | '\n' => {
                    if !symbol.is_empty() {
                        tokens.push(Token {
                            kind: TokenKind::Symbol(String::from_iter(symbol.clone())),
                            pos: (y, x),
                        });
                        symbol.clear();
                    }
                    if !value.is_empty() {
                        tokens.push(Token {
                            kind: TokenKind::Value(String::from_iter(value.clone())),
                            pos: (y, x),
                        });
                        value.clear();
                    }
                    if c == '\n' {
                        tokens.push(Token {
                            kind: TokenKind::Break,
                            pos: (y, x),
                        });
                        y += 1;
                        x = 0;
                    }
                }
                ':' => {
                    tokens.push(Token {
                        kind: TokenKind::Pin,
                        pos: (y, x),
                    });
                }
                '"' => {
                    tokens.push(Token {
                        kind: TokenKind::Comment,
                        pos: (y, x),
                    });
                }
                _ => {
                    tokens.push(Token {
                        kind: TokenKind::Symbol(c.to_string()),
                        pos: (y, x),
                    });
                    return Err(self.throw_at(
                        CompileErrorKind::UnexpectedChar(c),
                        &tokens,
                        tokens.len() - 1,
                    ));
                }
            }
            x += 1;
        }
        if !symbol.is_empty() {
            tokens.push(Token {
                kind: TokenKind::Symbol(String::from_iter(symbol.clone())),
                pos: (y, x),
            });
            symbol.clear();
        }
        if !value.is_empty() {
            tokens.push(Token {
                kind: TokenKind::Value(String::from_iter(value.clone())),
                pos: (y, x),
            });
            value.clear();
        }
        tokens.push(Token {
            kind: TokenKind::Break,
            pos: (y, x),
        });
        Ok(tokens)
    }
    fn parse(&self, tokens: &Vec<Token>) -> Result<Vec<OpWrap>, CompileError> {
        let mut ops = Vec::new();
        let mut pin = false;
        let mut pin_label = None;
        let mut op_name = None;
        let mut op_args = Vec::new();
        let mut comment = false;
        let mut tokens_in_line = Vec::new();
        for token in tokens {
            tokens_in_line.push(token.clone());
            match &token.kind {
                TokenKind::Pin => {
                    if comment {
                        continue;
                    }
                    if op_name.is_some() {
                        return Err(self.throw_at(
                            CompileErrorKind::InvalidLocation,
                            &tokens_in_line,
                            0,
                        ));
                    }
                    pin = true;
                }
                TokenKind::Symbol(symbol) => {
                    if comment {
                        continue;
                    }
                    if pin {
                        if pin_label.is_some() {
                            return Err(self.throw_at(
                                CompileErrorKind::InvalidLocation,
                                &tokens_in_line,
                                2,
                            ));
                        }
                        pin_label = Some(symbol);
                        continue;
                    }
                    if op_name.is_some() {
                        op_args.push(symbol);
                        continue;
                    }
                    op_name = Some(symbol);
                }
                TokenKind::Value(value) => {
                    if comment {
                        continue;
                    }
                    if op_name.is_some() {
                        op_args.push(value);
                    }
                }
                TokenKind::Comment => {
                    comment = true;
                }
                TokenKind::Break => {
                    if comment {
                        comment = false;
                        tokens_in_line.clear();
                        continue;
                    }
                    if pin_label.is_some() {
                        let label = pin_label.take().unwrap();
                        let op = Box::new(Pin(label.clone()));
                        ops.push(OpWrap {
                            op,
                            tokens: tokens_in_line.clone(),
                            pre_init: true,
                        });
                        tokens_in_line.clear();
                        pin = false;
                        continue;
                    }
                    if op_name.is_some() {
                        let name = op_name.take().unwrap();
                        let mut args = op_args.drain(..);
                        let op: Box<dyn Op> = match name.as_str() {
                            "mov" => {
                                let pos = unwrap_or_throw!(
                                    args.next(),
                                    self.throw_at(
                                        CompileErrorKind::ExpectedArgument,
                                        &tokens_in_line,
                                        0
                                    )
                                );
                                let refer = unwrap_or_throw!(
                                    args.next(),
                                    self.throw_at(
                                        CompileErrorKind::ExpectedArgument,
                                        &tokens_in_line,
                                        0
                                    )
                                );
                                Box::new(Mov(pos.to_string(), Ref::from_str(refer)?))
                            }
                            "add" => {
                                let pos = unwrap_or_throw!(
                                    args.next(),
                                    self.throw_at(
                                        CompileErrorKind::ExpectedArgument,
                                        &tokens_in_line,
                                        0
                                    )
                                );
                                let refer = unwrap_or_throw!(
                                    args.next(),
                                    self.throw_at(
                                        CompileErrorKind::ExpectedArgument,
                                        &tokens_in_line,
                                        0
                                    )
                                );
                                Box::new(Add(pos.to_string(), Ref::from_str(refer)?))
                            }
                            "sub" => {
                                let pos = unwrap_or_throw!(
                                    args.next(),
                                    self.throw_at(
                                        CompileErrorKind::ExpectedArgument,
                                        &tokens_in_line,
                                        0
                                    )
                                );
                                let refer = unwrap_or_throw!(
                                    args.next(),
                                    self.throw_at(
                                        CompileErrorKind::ExpectedArgument,
                                        &tokens_in_line,
                                        0
                                    )
                                );
                                Box::new(Sub(pos.to_string(), Ref::from_str(refer)?))
                            }
                            "cmp" => {
                                let pos = unwrap_or_throw!(
                                    args.next(),
                                    self.throw_at(
                                        CompileErrorKind::ExpectedArgument,
                                        &tokens_in_line,
                                        0
                                    )
                                );
                                let refer = unwrap_or_throw!(
                                    args.next(),
                                    self.throw_at(
                                        CompileErrorKind::ExpectedArgument,
                                        &tokens_in_line,
                                        0
                                    )
                                );
                                Box::new(Cmp(pos.to_string(), Ref::from_str(refer)?))
                            }
                            "jif" => {
                                let label = unwrap_or_throw!(
                                    args.next(),
                                    self.throw_at(
                                        CompileErrorKind::ExpectedArgument,
                                        &tokens_in_line,
                                        0
                                    )
                                );
                                Box::new(Jif(label.to_string()))
                            }
                            "jel" => {
                                let label = unwrap_or_throw!(
                                    args.next(),
                                    self.throw_at(
                                        CompileErrorKind::ExpectedArgument,
                                        &tokens_in_line,
                                        0
                                    )
                                );
                                Box::new(Jel(label.to_string()))
                            }
                            "jmp" => {
                                let label = unwrap_or_throw!(
                                    args.next(),
                                    self.throw_at(
                                        CompileErrorKind::ExpectedArgument,
                                        &tokens_in_line,
                                        0
                                    )
                                );
                                Box::new(Jmp(label.to_string()))
                            }
                            "out" => {
                                let refer = unwrap_or_throw!(
                                    args.next(),
                                    self.throw_at(
                                        CompileErrorKind::ExpectedArgument,
                                        &tokens_in_line,
                                        0,
                                    )
                                );
                                Box::new(Out(Ref::from_str(refer)?))
                            }
                            "utf" => {
                                let refer = unwrap_or_throw!(
                                    args.next(),
                                    self.throw_at(
                                        CompileErrorKind::ExpectedArgument,
                                        &tokens_in_line,
                                        0,
                                    )
                                );
                                Box::new(Utf(Ref::from_str(refer)?))
                            }
                            "ret" => Box::new(Ret),
                            "end" => Box::new(End),
                            _ => {
                                return Err(self.throw_at(
                                    CompileErrorKind::UnknownOp,
                                    &tokens_in_line,
                                    0,
                                ))
                            }
                        };
                        ops.push(OpWrap {
                            op,
                            tokens: tokens_in_line.clone(),
                            pre_init: false,
                        });
                        tokens_in_line.clear();
                        continue;
                    }
                }
            }
        }
        Ok(ops)
    }
    pub fn throw_at(&self, kind: CompileErrorKind, tokens: &[Token], index: usize) -> CompileError {
        let token = &tokens[index];
        let (y, _) = token.clone().pos;
        let lines = self.raw.lines().collect::<Vec<_>>();
        let length = match &token.kind {
            TokenKind::Symbol(symbol) => symbol.len(),
            _ => 1,
        };
        CompileError {
            kind,
            line: lines[y].to_string(),
            pos: token.clone().pos,
            len: length,
        }
    }
}
