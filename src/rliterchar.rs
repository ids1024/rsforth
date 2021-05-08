use rustyline::error::ReadlineError;
use std::collections::VecDeque;

/// Provides a wrapper around the rustyline library that behaves as a char
/// iterator, so it can be passed to the parser function that accept
/// char iterators.
pub struct RLIterChar {
    rl: rustyline::Editor<()>,
    chars: VecDeque<char>,
    first: bool,
}

impl RLIterChar {
    pub fn new() -> RLIterChar {
        RLIterChar {
            rl: rustyline::Editor::<()>::new(),
            chars: VecDeque::new(),
            first: true,
        }
    }
}

impl Iterator for RLIterChar {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.chars.pop_front() {
            Some(c)
        } else {
            if self.first {
                // XXX hack
                self.first = false;
            } else {
                println!("  OK");
            }
            match self.rl.readline("> ") {
                Ok(line) => {
                    self.rl.add_history_entry(&line);
                    self.chars.extend(line.chars());
                    self.chars.push_back('\n');
                    self.chars.pop_front()
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => None,
                _ => panic!("Readline error"), // TODO: handle better
            }
        }
    }
}
