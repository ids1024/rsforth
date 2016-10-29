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

#[derive(Clone)]
enum Word {
    //custom(Vec<String>),
    //custom(usize),
    custom(Rc<Vec<String>>),
    builtin(Builtin),
    colon,
    semicolon,
    _if,
    then,
    _else,
    //ifelse(Vec<String>, Vec<String>),
    //define(String, Vec<String>),
    //dot,
    //plus,
    //minus,
    //times,
    //divide,
    //dotquote(<String>),
    dotquote,
    int(i32),
    float(f32),
}

enum Branch {
    //word(Word),
    custom(Rc<Vec<String>>)
    //custom(Vec<String>),
    builtin(Builtin),
    ifelse(Vec<String>, Vec<String>),
    dotquote(<String>)
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

fn parse_next_word(code: &Chars, dict: &Dictionary) -> Some(Branch) {
    let mut word_str = String::new();
    let mut branch = None;
    while let Some(c) = chars.next() {
        if c == ' ' || c == '\n' {
            if let Some(word) = dict.get(word_str) {
                match word {
                    //stop => { return (words, true); },
                    Word::custom(_) | Word::builtin(_) | Word::int(_) | Word::float(_) => {
                        branch = Some(Branch::word(word.clone()))
                    }
                    Word::dotquote => {
                        let text = chars.take_while(|x| x != '"').collect();
                        branch = Branch::dotquote(text);
                    }
                    Word::colon => {
                        let mut name = String::new();
                        while let Some(c) = chars.next() {
                            if c == ' ' || c == '\n' {
                                break;
                            }
                            name.push(c);
                        }



                    }
                }
            break;
            }
        } else {
            word_str.push(c);
        }
    }

    branch
}

fn parse(code: &Chars) -> Vec<Branch> {
    let mut branches: Vec<Branch> = Vec::new();
    let mut word_str = String::new();
    let dict = Dictionary::default();
    while let Some(branch) = parse_next_word(code, &Dictionary) {
        branches.push(branch);
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
