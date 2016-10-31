use std::env;
use std::io::prelude::*;
use std::fs::File;

mod builtins;
mod dictionary;
mod word;
mod branch;
mod parser;
mod state;

use parser::parse;
use dictionary::Dictionary;
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
        // Primitive REPL
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let branches = parse(&mut line.unwrap().chars(), &mut dict, &mut state);
            for branch in branches {
                branch.call(&mut state);
            }
            std::io::stdout().flush().unwrap();
        }
    }
}
