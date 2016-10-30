use std::str::Chars;
use std::rc::Rc;
use dictionary::Dictionary;
use word::Word;
use branch::Branch;

/// Takes a chars iterator and returns all characters up to the next whitespace,
/// excluding the whitespace character. Returns `None` the `chars` iterator is
/// exhausted.
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
            Word::Paren => {
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

pub fn parse(code: &mut Chars, dict: &mut Dictionary) -> Vec<Branch> {
    let mut branches: Vec<Branch> = Vec::new();
    while let Some(word_str) = next_word(code) {
        if let Some(branch) = parse_word(&word_str, code, dict) {
            branches.push(branch);
        }
    }

    branches
}
