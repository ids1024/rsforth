use std::collections::HashMap;
use builtins::Builtin;
use word::Word;
use parser::parse;
use state::InterpState;

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
        dict.set("*", Word::Builtin(Builtin::Star));
        dict.set("/", Word::Builtin(Builtin::Slash));
        dict.set("abs", Word::Builtin(Builtin::Abs));
        dict.set("and", Word::Builtin(Builtin::And));
        dict.set("or", Word::Builtin(Builtin::Or));
        dict.set("xor", Word::Builtin(Builtin::Xor));
        dict.set("=", Word::Builtin(Builtin::Equals));
        dict.set("<", Word::Builtin(Builtin::LessThan));
        dict.set(">", Word::Builtin(Builtin::GreaterThan));
        dict.set("<=", Word::Builtin(Builtin::LessEqual));
        dict.set(">=", Word::Builtin(Builtin::GreaterEqual));
        dict.set("emit", Word::Builtin(Builtin::Emit));
        dict.set("dup", Word::Builtin(Builtin::Dup));
        dict.set("swap", Word::Builtin(Builtin::Swap));
        dict.set("over", Word::Builtin(Builtin::Over));
        dict.set("rot", Word::Builtin(Builtin::Rot));
        dict.set("tuck", Word::Builtin(Builtin::Tuck));
        dict.set("drop", Word::Builtin(Builtin::Drop));
        dict.set("@", Word::Builtin(Builtin::Fetch));
        dict.set("!", Word::Builtin(Builtin::Store));
        dict.set(":", Word::Colon);
        dict.set(";", Word::Semicolon);
        dict.set("if", Word::If);
        dict.set("then", Word::Then);
        dict.set("else", Word::Else);
        dict.set("(", Word::Paren);
        dict.set("variable", Word::Variable);
        // TODO Deal with standard library a better way
        let stdlib = include_str!("std.fs");
        let mut state = InterpState::default();
        // Stdlib should contain only definitions
        assert!(parse(&mut stdlib.chars(), &mut dict, &mut state) == vec![]);
        dict
    }
}
