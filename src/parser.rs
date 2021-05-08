use branch::Branch;
use dictionary::Dictionary;
use state::InterpState;
use std::rc::Rc;
use word::Word;

/// Takes a chars iterator and returns all characters up to the next whitespace,
/// excluding the whitespace character. Returns `None` the `chars` iterator is
/// exhausted.
pub fn next_word<T: Iterator<Item = char>>(chars: &mut T) -> Option<String> {
    let word = chars
        .skip_while(|x| x.is_whitespace())
        .take_while(|x| !x.is_whitespace())
        .collect::<String>();

    if word.is_empty() {
        // No input left
        None
    } else {
        Some(word)
    }
}

pub fn parse_word<T: Iterator<Item = char>>(
    word_str: &str,
    chars: &mut T,
    dict: &mut Dictionary,
    state: &mut InterpState,
) -> Option<Branch> {
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
                    if let Some(branch) = parse_word(&inner_word_str, chars, dict, state) {
                        inner_branches.push(branch);
                    }
                }
                dict.set(&name, Word::Custom(Rc::new(inner_branches)));
                None
            }
            Word::If => {
                let mut ifbranches = Vec::new();
                let mut elsebranches = Vec::new();
                let mut inelse = false;
                while let Some(inner_word_str) = next_word(chars) {
                    match dict.get(&inner_word_str) {
                        Some(Word::Else) => inelse = true,
                        Some(Word::Then) => break,
                        _ => {}
                    }
                    if let Some(branch) = parse_word(&inner_word_str, chars, dict, state) {
                        if inelse {
                            elsebranches.push(branch);
                        } else {
                            ifbranches.push(branch);
                        }
                    }
                }
                Some(Branch::IfElse(ifbranches, elsebranches))
            }
            Word::Variable => {
                let name = next_word(chars).unwrap();
                let addr = state.memory.new(0);
                dict.set(&name, Word::Int(addr));
                None
            }
            Word::Create => {
                let name = next_word(chars).unwrap();
                let addr = state.memory.here() + 1;
                dict.set(&name, Word::Int(addr));
                None
            }
            Word::Semicolon | Word::Else | Word::Then => {
                panic!("Invalid here");
            }
        }
    } else {
        panic!("Word '{}' not in dictionary.", word_str);
    }
}

pub fn parse<T: Iterator<Item = char>>(
    code: &mut T,
    dict: &mut Dictionary,
    state: &mut InterpState,
) -> Vec<Branch> {
    let mut branches: Vec<Branch> = Vec::new();
    while let Some(word_str) = next_word(code) {
        if let Some(branch) = parse_word(&word_str, code, dict, state) {
            branches.push(branch);
        }
    }

    branches
}
