use std::{borrow::Borrow, iter::Peekable, str::Chars};

fn main() {
    let code = "some # random with ' char literal ' \n and a  \" string literal \" s and ending with # a comment \n";

    Tokenizer::from(code).for_each(|token| eprint!("{:?},", token));

    // eprintln!("{:?}", split.collect::<Vec<_>>())
}

struct Tokenizer<'a>(Peekable<Chars<'a>>);

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(s: &'a str) -> Self {
        Tokenizer(s.chars().peekable())
    }
}

#[derive(Debug,PartialEq)]
enum Token {
    Symbol(String),
    Comment,
    Char(String),
    String(String),
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let chars = &mut self.0;
        if let Some(&c) = chars.peek() {
            match c {
                '\'' | '"' | '#' => {
                    let (end, varient):(_,fn(String)->Token) = match c {
                        '#' => ('\n', |_| Token::Comment),
                        '"' => (c,|s| Token::String(s)),
                        '\'' => (c, |s| Token::Char(s)),
                        _ => (c,|_| Token::Comment),
                    };
                    chars.next();
                    let token = varient(chars.by_ref().take_while(|&nc| nc != end).collect());
                    if token == Token::Comment {
                        self.next()
                    } else {
                        Some(token)
                    }
                }
                _ => {
                    let mut s = String::new();
                    while let Some(c) = chars.next_if(|nc| !['#', '\'', '"'].contains(nc)) {
                        if c == ' ' {
                            break;
                        }
                        s.push(c);
                    }
                    if s.is_empty() {
                        self.next()
                    } else {
                        Some(Token::Symbol(s))
                    }

                }
            }
        } else {
            None
        }
    }
}
