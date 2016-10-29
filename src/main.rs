use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::str::Chars;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Builtin {
    dot,
    plus,
    minus,
    times,
    divide,
}

impl Builtin {
    fn call(&self, stack: &mut Vec<i32>) {
        match *self {
            Builtin::dot => print!("{}", stack.pop().unwrap()),
            Builtin::plus => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 + n2);
            },
            Builtin::minus => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 - n2);
            },
            Builtin::times => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 * n2);
            },
            Builtin::divide => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 / n2);
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Word {
    custom(Rc<Vec<Branch>>),
    builtin(Builtin),
    int(i32),
    float(f32),
    parenthesis,
    colon,
    semicolon,
    _if,
    then,
    _else,
    dotquote,
}

#[derive(Debug, PartialEq)]
enum Branch {
    custom(Rc<Vec<Branch>>),
    builtin(Builtin),
    int(i32),
    float(f32),
    ifelse(Vec<Branch>, Vec<Branch>),
    dotquote(String)
}

impl Branch {
    fn call(&self, stack: &mut Vec<i32>) {
        match self {
            &Branch::custom(ref branches) => {
                for branch in branches.iter() {
                    branch.call(stack);
                }
            },
            &Branch::builtin(ref builtin) => builtin.call(stack),
            &Branch::int(int) => stack.push(int),
            &Branch::float(float) => stack.push(float as i32), //XXX
            &Branch::ifelse(ref ifbranches, ref elsebranches) => {
                if stack.pop().unwrap() == 0 {
                    for branch in ifbranches.iter() {
                        branch.call(stack);
                    }
                } else {
                    for branch in elsebranches.iter() {
                        branch.call(stack);
                    }
                }
            },
            &Branch::dotquote(ref text) => print!("{}", text),
        }
    }
}

struct Dictionary {
    items: HashMap<String, Word>
}

impl Dictionary {
    fn get(&self, name: &str) -> Option<Word> {
        if let Some(word) = self.items.get(name) {
            Some(word.clone())
        } else if let Ok(num) = name.parse::<i32>() {
            Some(Word::int(num))
        } else if let Ok(num) = name.parse::<f32>() {
            Some(Word::float(num))
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
        dict.insert(".\"".to_owned(), Word::dotquote);
        dict.insert(".".to_owned(), Word::builtin(Builtin::dot));
        dict.insert("+".to_owned(), Word::builtin(Builtin::plus));
        dict.insert("-".to_owned(), Word::builtin(Builtin::minus));
        dict.insert("*".to_owned(), Word::builtin(Builtin::times));
        dict.insert("/".to_owned(), Word::builtin(Builtin::divide));
        dict.insert(":".to_owned(), Word::colon);
        dict.insert(";".to_owned(), Word::semicolon);
        dict.insert("if".to_owned(), Word::_if);
        dict.insert("then".to_owned(), Word::then);
        dict.insert("else".to_owned(), Word::_else);
        dict.insert("(".to_owned(), Word::parenthesis);
        Dictionary{items: dict}
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
            Word::custom(x) => Some(Branch::custom(x)),
            Word::builtin(x) => Some(Branch::builtin(x)),
            Word::int(x) => Some(Branch::int(x)),
            Word::float(x) => Some(Branch::float(x)),
            Word::dotquote => {
                let text = chars.take_while(|x| *x != '"').collect();
                Some(Branch::dotquote(text))
            },
            Word::parenthesis => {
                // Consumes all characters up to next )
                chars.take_while(|x| *x != ')').count();
                None
            },
            Word::colon => {
                let name = next_word(chars).unwrap();
                let mut inner_branches: Vec<Branch> = Vec::new();
                while let Some(inner_word_str) = next_word(chars) {
                    if dict.get(&inner_word_str) == Some(Word::semicolon) {
                        break;
                    }
                    if let Some(branch) = parse_word(&inner_word_str, chars, dict) {
                        inner_branches.push(branch);
                    }
                }
                dict.set(&name, Word::custom(Rc::new(inner_branches)));
                None
            },
            Word::_if => {
                let mut ifbranches = Vec::new();
                let mut elsebranches = Vec::new();
                while let Some(inner_word_str) = next_word(chars) {
                    if dict.get(&inner_word_str) == Some(Word::_else) {
                        while let Some(inner_word_str) = next_word(chars) {
                            if dict.get(&inner_word_str) == Some(Word::then) {
                                break;
                            }
                    }
                        if let Some(branch) = parse_word(&inner_word_str, chars, dict) {
                            elsebranches.push(branch);
                        }
                        break;
                    } else if dict.get(&inner_word_str) == Some(Word::then) {
                        break;
                    }
                    if let Some(branch) = parse_word(&inner_word_str, chars, dict) {
                        ifbranches.push(branch);
                    }
                }
                Some(Branch::ifelse(ifbranches, elsebranches))
            },
            Word::semicolon | Word::_else | Word::then => {
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
    //println!("{:#?}", parse(&mut code.chars()));
    let mut stack = Vec::new();
    for branch in branches {
        branch.call(&mut stack);
    }
}
