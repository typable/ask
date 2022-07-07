use std::collections::HashMap;
use std::env;
use std::fs;

#[allow(dead_code)]
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    #[allow(dead_code)]
    fn fg(&self) -> usize {
        match self {
            Self::Black => 30,
            Self::Red => 31,
            Self::Green => 32,
            Self::Yellow => 33,
            Self::Blue => 34,
            Self::Magenta => 35,
            Self::Cyan => 36,
            Self::White => 37,
            Self::BrightBlack => 90,
            Self::BrightRed => 91,
            Self::BrightGreen => 92,
            Self::BrightYellow => 93,
            Self::BrightBlue => 94,
            Self::BrightMagenta => 95,
            Self::BrightCyan => 96,
            Self::BrightWhite => 97,
        }
    }
    #[allow(dead_code)]
    fn bg(&self) -> usize {
        match self {
            Self::Black => 40,
            Self::Red => 41,
            Self::Green => 42,
            Self::Yellow => 43,
            Self::Blue => 44,
            Self::Magenta => 45,
            Self::Cyan => 46,
            Self::White => 47,
            Self::BrightBlack => 100,
            Self::BrightRed => 101,
            Self::BrightGreen => 102,
            Self::BrightYellow => 103,
            Self::BrightBlue => 104,
            Self::BrightMagenta => 105,
            Self::BrightCyan => 106,
            Self::BrightWhite => 107,
        }
    }
}

macro_rules! color {
    ($str:expr, $fg:expr) => {
        format!("\x1b[{}m{}\x1b[0m", $fg.fg(), $str)
    };
    ($str:expr, $fg:expr, $bg:expr) => {
        format!("\x1b[{};{}m{}\x1b[0m", $fg.fg(), $bg.bg(), $str)
    };
}

type Label = String;
type Pos = String;
type Value = usize;

#[derive(Debug)]
enum Op {
    Pin(Label),
    Mov(Pos, Value),
    Add(Pos, Pos),
    Sub(Pos, Pos),
    Cmp(Pos, Value),
    Jmp(Label, Value),
    Out(Pos),
    Utf(Pos),
}

enum Token {
    Symbol(String),
    Value(String),
    Break,
    Pin,
    Comment,
}

struct Parser;

impl Parser {
    fn tokenize(&self, raw: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut symbol = Vec::new();
        let mut value = Vec::new();
        for c in raw.chars() {
            match c {
                c if c.is_alphabetic() => {
                    symbol.push(c);
                }
                c if c.is_alphanumeric() => {
                    value.push(c);
                }
                ' ' | '\t' | '\r' | '\n' => {
                    if !symbol.is_empty() {
                        tokens.push(Token::Symbol(String::from_iter(symbol.clone())));
                        symbol.clear();
                    }
                    if !value.is_empty() {
                        tokens.push(Token::Value(String::from_iter(value.clone())));
                        value.clear();
                    }
                    if c == '\n' {
                        tokens.push(Token::Break);
                    }
                }
                ':' => {
                    tokens.push(Token::Pin);
                }
                '"' => {
                    tokens.push(Token::Comment);
                }
                _ => unreachable!(),
            }
        }
        if !symbol.is_empty() {
            tokens.push(Token::Symbol(String::from_iter(symbol.clone())));
            symbol.clear();
        }
        if !value.is_empty() {
            tokens.push(Token::Value(String::from_iter(value.clone())));
            value.clear();
        }
        tokens.push(Token::Break);
        tokens
    }
    fn parse(&self, tokens: Vec<Token>) -> Vec<Op> {
        let mut ops = Vec::new();
        let mut pin = false;
        let mut pin_label = None;
        let mut op_name = None;
        let mut op_args = Vec::new();
        let mut comment = false;
        for token in tokens {
            match token {
                Token::Pin => {
                    if comment {
                        continue;
                    }
                    if op_name.is_some() {
                        panic!();
                    }
                    pin = true;
                }
                Token::Symbol(symbol) => {
                    if comment {
                        continue;
                    }
                    if pin {
                        if pin_label.is_some() {
                            panic!();
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
                Token::Value(value) => {
                    if comment {
                        continue;
                    }
                    if op_name.is_some() {
                        op_args.push(value);
                    }
                }
                Token::Comment => {
                    comment = true;
                }
                Token::Break => {
                    if comment {
                        comment = false;
                        continue;
                    }
                    if pin_label.is_some() {
                        let label = pin_label.take().unwrap();
                        ops.push(Op::Pin(label));
                        pin = false;
                        continue;
                    }
                    if op_name.is_some() {
                        let name = op_name.take().unwrap();
                        let mut args = op_args.drain(..);
                        let op = match name.as_str() {
                            "mov" => {
                                let pos = args.next().unwrap().clone();
                                let value = args.next().unwrap().parse::<usize>().unwrap();
                                Op::Mov(pos, value)
                            }
                            "add" => {
                                let pos_a = args.next().unwrap().clone();
                                let pos_b = args.next().unwrap().clone();
                                Op::Add(pos_a, pos_b)
                            }
                            "sub" => {
                                let pos_a = args.next().unwrap().clone();
                                let pos_b = args.next().unwrap().clone();
                                Op::Sub(pos_a, pos_b)
                            }
                            "cmp" => {
                                let pos = args.next().unwrap().clone();
                                let value = args.next().unwrap().parse::<usize>().unwrap();
                                Op::Cmp(pos, value)
                            }
                            "jmp" => {
                                let label = args.next().unwrap();
                                let value = args.next().unwrap().parse::<usize>().unwrap();
                                Op::Jmp(label, value)
                            }
                            "out" => {
                                let pos = args.next().unwrap().clone();
                                Op::Out(pos)
                            }
                            "utf" => {
                                let pos = args.next().unwrap().clone();
                                Op::Utf(pos)
                            }
                            _ => unreachable!(),
                        };
                        ops.push(op);
                        continue;
                    }
                }
            }
        }
        ops
    }
}

#[derive(Default)]
struct Executor {
    memory: HashMap<String, usize>,
    pins: HashMap<String, usize>,
}

impl Executor {
    fn execute(&mut self, ops: Vec<Op>) {
        let mut i = 0;
        while i < ops.len() {
            match &ops[i] {
                Op::Pin(label) => {
                    self.pins.insert(label.clone(), i);
                }
                Op::Mov(pos, value) => {
                    self.memory.insert(pos.clone(), *value);
                }
                Op::Add(pos_a, pos_b) => {
                    let val_a = self.memory.get(pos_a).unwrap();
                    let val_b = self.memory.get(pos_b).unwrap();
                    self.memory.insert(pos_a.clone(), val_a + val_b);
                }
                Op::Sub(pos_a, pos_b) => {
                    let val_a = self.memory.get(pos_a).unwrap();
                    let val_b = self.memory.get(pos_b).unwrap();
                    self.memory.insert(pos_a.clone(), val_a - val_b);
                }
                Op::Cmp(pos, value) => {
                    let val = self.memory.get(pos).unwrap();
                    self.memory
                        .insert("#".to_string(), if *val == *value { 1 } else { 0 });
                }
                Op::Jmp(label, value) => {
                    let code = self.memory.remove("#").unwrap();
                    if code == *value {
                        i = *self.pins.get(label).unwrap();
                    }
                }
                Op::Out(pos) => {
                    let val = self.memory.get(pos).unwrap();
                    print!("{}", val);
                }
                Op::Utf(pos) => {
                    let val = self.memory.get(pos).unwrap();
                    print!("{}", String::from_utf8_lossy(&[*val as u8]));
                }
            }
            i += 1;
        }
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let path = args.next().expect("No file path provided!");
    let flag = args.next();
    let raw = fs::read_to_string(path).expect("Unable to read file!");
    let parser = Parser;
    let tokens = parser.tokenize(&raw);
    if let Some(flag) = flag {
        match flag.as_str() {
            "--fmt" => {
                let mut i = 0;
                let mut pin = false;
                let mut label = false;
                let mut comment = false;
                for token in tokens {
                    match token {
                        Token::Symbol(symbol) => {
                            if comment {
                                if i > 1 && !pin {
                                    print!(" ");
                                }
                            } else {
                                if i > 0 && !pin {
                                    print!(" ");
                                }
                            }
                            let color = if i == 0 {
                                Color::BrightRed
                            } else if pin || label {
                                Color::BrightBlue
                            } else {
                                Color::White
                            };
                            if "jmp".eq(&symbol) {
                                label = true;
                            }
                            if comment {
                                print!("{}", color!(symbol, Color::BrightGreen));
                            } else {
                                print!("{}", color!(symbol, color));
                            }
                        }
                        Token::Value(value) => {
                            if comment {
                                print!(" {}", color!(value, Color::BrightGreen));
                                continue;
                            }
                            print!(" {}", color!(value, Color::White));
                        }
                        Token::Pin => {
                            pin = true;
                            if comment {
                                print!("{}", color!(":", Color::BrightGreen));
                                continue;
                            }
                            print!("{}", color!(":", Color::BrightBlue));
                        }
                        Token::Comment => {
                            print!("{}", color!("\" ", Color::BrightGreen));
                            comment = true;
                        }
                        Token::Break => {
                            i = 0;
                            pin = false;
                            label = false;
                            comment = false;
                            println!("");
                            continue;
                        }
                    }
                    i += 1;
                }
            }
            _ => unreachable!(),
        }
        return;
    }
    let ops = parser.parse(tokens);
    let mut executor = Executor::default();
    executor.execute(ops);
}
