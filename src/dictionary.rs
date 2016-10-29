use std::collections::HashMap;
use builtins::Builtin;
use word::Word;
use parser::parse;

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
}

impl Default for Dictionary {
    fn default() -> Dictionary {
        let mut dict = Dictionary { items: HashMap::new() };
        dict.set(".\"", Word::Dotquote);
        dict.set(".", Word::Builtin(Builtin::Dot));
        dict.set("+", Word::Builtin(Builtin::Plus));
        dict.set("-", Word::Builtin(Builtin::Minus));
        dict.set("*", Word::Builtin(Builtin::Times));
        dict.set("/", Word::Builtin(Builtin::Divide));
        dict.set("=", Word::Builtin(Builtin::Equal));
        dict.set("<", Word::Builtin(Builtin::LessThan));
        dict.set(">", Word::Builtin(Builtin::GreaterThan));
        dict.set("<=", Word::Builtin(Builtin::LessEqual));
        dict.set(">=", Word::Builtin(Builtin::GreaterEqual));
        dict.set("emit", Word::Builtin(Builtin::Emit));
        dict.set(":", Word::Colon);
        dict.set(";", Word::Semicolon);
        dict.set("if", Word::If);
        dict.set("then", Word::Then);
        dict.set("else", Word::Else);
        dict.set("(", Word::Parenthesis);
        // TODO Deal with standard library a better way
        let stdlib = include_str!("std.fs");
        // Stdlib should contain only definitions
        assert!(parse(&mut stdlib.chars(), &mut dict) == vec![]);
        dict
    }
}
