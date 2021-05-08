use builtins::Builtin;
use enum_variants::Variants;
use parser::parse;
use state::InterpState;
use std::collections::HashMap;
use word::Word;

/// Represents the Forth dictionary, which maps words (Strings) to their
/// implementations (the Word type)
pub struct Dictionary {
    items: HashMap<String, Word>,
}

impl Dictionary {
    pub fn get(&self, name: &str) -> Option<Word> {
        // Make lookup case insensitive
        let uppername = name.to_uppercase();
        if let Some(word) = self.items.get(&uppername) {
            Some(word.clone())
        } else if let Ok(num) = name.parse::<i32>() {
            Some(Word::Int(num))
        } else if let Ok(num) = name.parse::<f32>() {
            Some(Word::Float(num))
        } else {
            None
        }
    }
    pub fn set(&mut self, name: &str, value: Word) {
        if self.items.contains_key(name) {
            // XXX display debug message another way
            println!("Redefined {}", name);
        }
        // Make lookup case insensitive
        self.items.insert(name.to_uppercase(), value);
    }

    pub fn new(state: &mut InterpState) -> Dictionary {
        let mut dict = Dictionary {
            items: HashMap::new(),
        };
        dict.set(".\"", Word::Dotquote);
        for i in Builtin::variants() {
            dict.set(i.word(), Word::Builtin(i));
        }
        dict.set(":", Word::Colon);
        dict.set(";", Word::Semicolon);
        dict.set("if", Word::If);
        dict.set("then", Word::Then);
        dict.set("else", Word::Else);
        dict.set("(", Word::Paren);
        dict.set("variable", Word::Variable);
        dict.set("create", Word::Create);
        // TODO Deal with standard library a better way
        let stdlib = include_str!("std.fs");
        let branches = parse(&mut stdlib.chars(), &mut dict, state);
        for branch in branches {
            branch.call(state);
        }

        dict
    }
}
