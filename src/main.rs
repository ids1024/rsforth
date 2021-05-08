#[macro_use]
extern crate derivation;
extern crate enum_variants;

mod branch;
mod builtins;
mod dictionary;
mod parser;
mod rliterchar;
mod state;
mod word;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use dictionary::Dictionary;
use parser::{next_word, parse, parse_word};
use rliterchar::RLIterChar;
use state::InterpState;

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
