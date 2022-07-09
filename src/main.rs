use std::env;
use std::fs;
use std::process;

use ask::color;
use ask::Color;
use ask::Compiler;
use ask::Runtime;

fn main() {
    let mut args = env::args();
    args.next();
    let path = args.next();
    if path.is_none() {
        println!(
            "{}",
            color!("No file path specified! Use: ask <file>", Color::BrightRed),
        );
        process::exit(1);
    }
    let raw = fs::read_to_string(path.unwrap());
    if let Err(err) = raw {
        println!(
            "{}",
            color!(
                format!("Unable to read file! Reason: {}", err),
                Color::BrightRed
            )
        );
        process::exit(1);
    }
    let mut compiler = Compiler::default();
    let exe = compiler.compile(&raw.unwrap());
    if let Err(err) = &exe {
        println!("{}", err);
        process::exit(1);
    }
    let mut runtime = Runtime::default();
    if let Err(err) = runtime.execute(exe.unwrap()) {
        println!("{}", err);
    }
}
