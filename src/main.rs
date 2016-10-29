use std::env;
use std::io::prelude::*;
use std::fs::File;

mod builtins;
mod dictionary;
mod word;
mod branch;
mod parser;

use parser::parse;

fn main() {
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(&path).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    let branches = parse(&mut code.chars());
    // println!("{:#?}", parse(&mut code.chars()));
    let mut stack = Vec::new();
    for branch in branches {
        branch.call(&mut stack);
    }
}
