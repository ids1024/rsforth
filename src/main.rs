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
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(&path).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    let mut dict = Dictionary::default();
    let branches = parse(&mut code.chars(), &mut dict);
    // println!("{:#?}", parse(&mut code.chars()));
    let mut stack = Stack::default();
    for branch in branches {
        branch.call(&mut stack);
    }
}
