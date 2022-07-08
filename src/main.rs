use std::env;
use std::fs;

use ask::Compiler;
use ask::Runtime;

fn main() {
    let mut args = env::args();
    args.next();
    let path = args.next().expect("No file path provided!");
    let raw = fs::read_to_string(path).expect("Unable to read file!");
    let compiler = Compiler::default();
    let exe = compiler.compile(&raw);
    let mut runtime = Runtime::default();
    if let Err(err) = runtime.execute(exe) {
        println!("{}", err);
    }
}
