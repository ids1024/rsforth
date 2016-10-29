use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::str::Chars;
use std::rc::Rc;
use std::collections::HashMap;

mod builtins;
use builtins::Builtin;

#[derive(Clone, Debug, PartialEq)]
enum Word {
    Custom(Rc<Vec<Branch>>),
    Builtin(Builtin),
    Int(i32),
    Float(f32),
    Parenthesis,
    Colon,
    Semicolon,
    If,
    Then,
    Else,
    Dotquote,
}

#[derive(Debug, PartialEq)]
enum Branch {
    Custom(Rc<Vec<Branch>>),
    Builtin(Builtin),
    Int(i32),
    Float(f32),
    IfElse(Vec<Branch>, Vec<Branch>),
    Dotquote(String),
}

impl Branch {
    fn call(&self, stack: &mut Vec<i32>) {
        match self {
            &Branch::Custom(ref branches) => {
                for branch in branches.iter() {
                    branch.call(stack);
                }
            }
            &Branch::Builtin(ref builtin) => builtin.call(stack),
            &Branch::Int(int) => stack.push(int),
            &Branch::Float(float) => stack.push(float as i32), //XXX
            &Branch::IfElse(ref ifbranches, ref elsebranches) => {
                if stack.pop().unwrap() == 0 {
                    for branch in ifbranches.iter() {
                        branch.call(stack);
                    }
                } else {
                    for branch in elsebranches.iter() {
                        branch.call(stack);
                    }
                }
            }
            &Branch::Dotquote(ref text) => print!("{}", text),
        }
    }
}

struct Dictionary {
    items: HashMap<String, Word>,
}

impl Dictionary {
    fn get(&self, name: &str) -> Option<Word> {
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
    fn set(&mut self, name: &str, value: Word) {
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

fn next_word(chars: &mut Chars) -> Option<String> {
    let mut word = String::new();

    while let Some(c) = chars.next() {
        if c == ' ' || c == '\n' {
            if word.is_empty() {
                continue;
            }
            break;
        }
        word.push(c);
    }

    if word.is_empty() {
        // No input left
        None
    } else {
        Some(word)
    }
}

fn parse_word(word_str: &str, chars: &mut Chars, dict: &mut Dictionary) -> Option<Branch> {
    if let Some(word) = dict.get(word_str) {
        match word {
            Word::Custom(x) => Some(Branch::Custom(x)),
            Word::Builtin(x) => Some(Branch::Builtin(x)),
            Word::Int(x) => Some(Branch::Int(x)),
            Word::Float(x) => Some(Branch::Float(x)),
            Word::Dotquote => {
                let text = chars.take_while(|x| *x != '"').collect();
                Some(Branch::Dotquote(text))
            }
            Word::Parenthesis => {
                // Consumes all characters up to next )
                chars.take_while(|x| *x != ')').count();
                None
            }
            Word::Colon => {
                let name = next_word(chars).unwrap();
                let mut inner_branches: Vec<Branch> = Vec::new();
                while let Some(inner_word_str) = next_word(chars) {
                    if dict.get(&inner_word_str) == Some(Word::Semicolon) {
                        break;
                    }
                    if let Some(branch) = parse_word(&inner_word_str, chars, dict) {
                        inner_branches.push(branch);
                    }
                }
                dict.set(&name, Word::Custom(Rc::new(inner_branches)));
                None
            }
            Word::If => {
                let mut ifbranches = Vec::new();
                let mut elsebranches = Vec::new();
                while let Some(inner_word_str) = next_word(chars) {
                    if dict.get(&inner_word_str) == Some(Word::Else) {
                        while let Some(inner_word_str) = next_word(chars) {
                            if dict.get(&inner_word_str) == Some(Word::Then) {
                                break;
                            }
                        }
                        if let Some(branch) = parse_word(&inner_word_str, chars, dict) {
                            elsebranches.push(branch);
                        }
                        break;
                    } else if dict.get(&inner_word_str) == Some(Word::Then) {
                        break;
                    }
                    if let Some(branch) = parse_word(&inner_word_str, chars, dict) {
                        ifbranches.push(branch);
                    }
                }
                Some(Branch::IfElse(ifbranches, elsebranches))
            }
            Word::Semicolon | Word::Else | Word::Then => {
                panic!("Invalid here");
            }
        }
    } else {
        panic!("Word '{}' not in dictionary.", word_str);
    }
}

fn parse(code: &mut Chars) -> Vec<Branch> {
    let mut branches: Vec<Branch> = Vec::new();
    let mut dict = Dictionary::default();
    while let Some(word_str) = next_word(code) {
        if let Some(branch) = parse_word(&word_str, code, &mut dict) {
            branches.push(branch);
        }
    }

    branches
}

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
