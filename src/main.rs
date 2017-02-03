#[macro_use]
extern crate derivation;
extern crate enum_variants;

mod builtins;
mod dictionary;
mod word;
mod branch;
mod parser;
mod state;
mod rliterchar;

use std::env;
use std::io::prelude::*;
use std::fs::File;

use parser::{parse, next_word, parse_word};
use dictionary::Dictionary;
use state::InterpState;
use rliterchar::RLIterChar;


fn main() {
    let mut state = InterpState::default();
    let mut dict = Dictionary::new(&mut state);

    if let Some(path) = env::args().nth(1) {
        let mut file = File::open(&path).unwrap();
        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();

        let branches = parse(&mut code.chars(), &mut dict, &mut state);
        for branch in branches {
            branch.call(&mut state);
        }
    } else {
        // REPL
        let mut chars = RLIterChar::new();
        while let Some(word_str) = next_word(&mut chars) {
            if let Some(branch) = parse_word(&word_str, &mut chars, &mut dict, &mut state) {
                branch.call(&mut state);
            }
        }
    }
}
