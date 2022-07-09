use crate::error::CompileError;
use crate::error::CompileErrorKind;
use crate::Executable;
use crate::Op;
use crate::OpKind;
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
    fn parse(&self, tokens: &Vec<Token>) -> Result<Vec<Op>, CompileError> {
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
                        ops.push(Op {
                            kind: OpKind::Pin(label.clone()),
                            tokens: tokens_in_line.clone(),
                        });
                        tokens_in_line.clear();
                        pin = false;
                        continue;
                    }
                    if op_name.is_some() {
                        let name = op_name.take().unwrap();
                        let mut args = op_args.drain(..);
                        let op_kind = match name.as_str() {
                            "mov" => {
                                let key = args.next().unwrap().clone();
                                let value = args.next().unwrap().parse::<usize>().unwrap();
                                OpKind::Mov(key, value)
                            }
                            "add" => {
                                let key_a = args.next().unwrap().clone();
                                let key_b = args.next().unwrap().clone();
                                OpKind::Add(key_a, key_b)
                            }
                            "sub" => {
                                let key_a = args.next().unwrap().clone();
                                let key_b = args.next().unwrap().clone();
                                OpKind::Sub(key_a, key_b)
                            }
                            "cmp" => {
                                let key = args.next().unwrap().clone();
                                let value = args.next().unwrap().parse::<usize>().unwrap();
                                OpKind::Cmp(key, value)
                            }
                            "jif" => {
                                let label = args.next().unwrap();
                                OpKind::Jif(label.clone())
                            }
                            "jel" => {
                                let label = args.next().unwrap();
                                OpKind::Jel(label.clone())
                            }
                            "out" => {
                                let key = args.next().unwrap().clone();
                                OpKind::Out(key)
                            }
                            "utf" => {
                                let key = args.next().unwrap().clone();
                                OpKind::Utf(key)
                            }
                            "jmp" => {
                                let label = args.next().unwrap();
                                OpKind::Jmp(label.clone())
                            }
                            "ret" => OpKind::Ret,
                            "end" => OpKind::End,
                            _ => {
                                return Err(self.throw_at(
                                    CompileErrorKind::UnknownOp,
                                    &tokens_in_line,
                                    0,
                                ));
                            }
                        };
                        ops.push(Op {
                            kind: op_kind,
                            tokens: tokens_in_line.clone(),
                        });
                        tokens_in_line.clear();
                        continue;
                    }
                }
            }
        }
        Ok(ops)
    }
    pub fn throw_at(
        &self,
        kind: CompileErrorKind,
        tokens: &Vec<Token>,
        index: usize,
    ) -> CompileError {
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
