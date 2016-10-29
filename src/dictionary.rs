use std::collections::HashMap;
use builtins::Builtin;
use word::Word;

pub struct Dictionary {
    items: HashMap<String, Word>,
}

impl Dictionary {
    pub fn get(&self, name: &str) -> Option<Word> {
        if let Some(word) = self.items.get(name) {
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
        self.items.insert(name.to_owned(), value);
    }
}

impl Default for Dictionary {
    fn default() -> Dictionary {
        let mut dict = HashMap::new();
        dict.insert(".\"".to_owned(), Word::Dotquote);
        dict.insert(".".to_owned(), Word::Builtin(Builtin::Dot));
        dict.insert("+".to_owned(), Word::Builtin(Builtin::Plus));
        dict.insert("-".to_owned(), Word::Builtin(Builtin::Minus));
        dict.insert("*".to_owned(), Word::Builtin(Builtin::Times));
        dict.insert("/".to_owned(), Word::Builtin(Builtin::Divide));
        dict.insert(":".to_owned(), Word::Colon);
        dict.insert(";".to_owned(), Word::Semicolon);
        dict.insert("if".to_owned(), Word::If);
        dict.insert("then".to_owned(), Word::Then);
        dict.insert("else".to_owned(), Word::Else);
        dict.insert("(".to_owned(), Word::Parenthesis);
        Dictionary { items: dict }
    }
}
