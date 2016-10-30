use std::env;
use std::io::prelude::*;
use std::fs::File;

mod builtins;
mod dictionary;
mod word;
mod branch;
mod parser;
mod stack;

use parser::parse;
use stack::Stack;
use dictionary::Dictionary;

fn main() {
    let mut dict = Dictionary::default();
    let mut stack = Stack::default();

    if let Some(path) = env::args().nth(1) {
        let mut file = File::open(&path).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();

        let branches = parse(&mut code.chars(), &mut dict);
        for branch in branches {
            branch.call(&mut stack);
        }
    } else {
        // Primitive REPL
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let branches = parse(&mut line.unwrap().chars(), &mut dict);
            for branch in branches {
                branch.call(&mut stack);
            }
            std::io::stdout().flush().unwrap();
        }
    }
}
