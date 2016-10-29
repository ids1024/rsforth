use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::str::Chars;

enum Builtin {
    dot,
    plus,
    minus,
    times,
    divide,
}

impl Builtin {
    fn call(&self, stack: &Vec<i32>) {
        match *self {
            Builtin::dot => print!("{}", stack.pop()),
            Bulltin::plus => {
                n2 = stack.pop();
                n1 = stack.pop();
                stack.push(n1 + n2);
            },
            Bulltin::minus => {
                n2 = stack.pop();
                n1 = stack.pop();
                stack.push(n1 - n2);
            },
            Bulltin::times => {
                n2 = stack.pop();
                n1 = stack.pop();
                stack.push(n1 * n2);
            },
            Bulltin::divide => {
                n2 = stack.pop();
                n1 = stack.pop();
                stack.push(n1 / n2);
            },
        }
    }
}

#[derive(Clone)]
enum Word {
    custom(Rc<Vec<Word>>),
    builtin(Builtin),
    colon,
    semicolon,
    _if,
    then,
    _else,
    dotquote,
    int(i32),
    float(f32),
}

enum Branch {
    custom(Rc<Vec<Word>>)
    builtin(Builtin),
    ifelse(Vec<Word>, Vec<Word>),
    dotquote(<String>)
    int(i32),
    float(f32),
}

struct Dictionary {
    items: HashMap<String, Word>
}

impl Dictionary {
    fn get(&self, name: &str) -> Option<Word> {
        if let Some(word) = self.get(name) {
            Some(word)
        } else if let Ok(num) = name.parse::<i32>() {
            Some(Word::int(num))
        } else if let Ok(num) = name.parse::<f32>() {
            Some(Word::float(num))
        } else {
            None
        }
    }
    fn set(&self, name: &str, value: Word) {
        if self.items.contains_key(name) {
            // XXX display debug message another way
            println!("Redefined {}", name);
        }
    }
}

impl Default for Dictionary {
    fn default() -> Dictionary {
        dict = HashMap::new();
        dict.insert(".\"", Word::dotquote);
        dict.insert(".", Word::builtin(Builtin::dot));
        dict.insert("+", Word::builtin(Builtin::plus));
        dict.insert("-", Word::builtin(Buitlin::minus));
        dict.insert("*", Word::builtin(Buitlin::times));
        dict.insert("/", Word::builtin(Buitlin::divide));
        dict.insert(":", Word::colon);
        dict.insert(";", Word::semicolon);
        dict.insert("if", Word::_if);
        dict.insert("then", Word::_then);
        dict.insert("else", Word::_else);
        Dictionary{items: dict}
    }
}

fn next_word(chars: &Chars) -> Some(String) {
    let mut word = String::new();

    while let Some(c) = chars.next() {
        if (c == ' ' || c == '\n') && !string.is_empty() {
            break;
        }
        string.push(c);
    }

    if string.is_empty() {
        // No input left
        None
    else {
        Some(string)
    }
}

fn parse_next_word(word_str: &str, chars: &Chars, dict: &Dictionary) -> Branch {
    if let Some(word) = dict.get(word_str) {
        match word {
            //stop => { return (words, true); },
            Word::custom(_) | Word::builtin(_) | Word::int(_) | Word::float(_) => {
                Branch::word(word.clone())
            }
            Word::dotquote => {
                let text = chars.take_while(|x| x != '"').collect();
                Branch::dotquote(text)
            }
            Word::colon => {
                let name = next_word(chars).unwrap();
                


            }
        }
    } else {
        panic!("Word not in dictionary.");
    }
}

fn parse(code: &Chars) -> Vec<Branch> {
    let mut branches: Vec<Branch> = Vec::new();
    let mut word_str = String::new();
    let dict = Dictionary::default();
    while let Some(word_str) = next_word(code) {
        branches.push(parse_word(&word_str, code, dict));
    }

    branches
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let mut file = File::open(&path).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    
    println!("{:#?}", parse(&code.iter(), None).0);
}
