// no comment or string/char

use std::{str::Chars, iter::Peekable};

fn main() {
    let code = " hi there";
    
    let result = code.chars().tokenize().collect::<Vec<_>>();

    println!("result:{:?}",result);
}

#[allow(dead_code)]
#[derive(Debug)]
enum Token {
    Symbol(String),
    Non
}

struct Tokenizer<I:Iterator>(Peekable<I>);

impl<I:Iterator<Item = char>> Iterator for Tokenizer<I> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if None == self.0.by_ref().peek() {
            return None;
        }
        loop {
            let s = self.0.by_ref().take_while(|c| !c.is_whitespace()).collect::<String>();
            if !s.is_empty() {
                return Some(Token::Symbol(s));
            }
        }
    }
}

trait Tokenize : Iterator + Sized {
    fn tokenize(self) -> Tokenizer<Self>;
}

impl<'a> Tokenize for Chars<'a> {
    fn tokenize(self) -> Tokenizer<Self> {
        Tokenizer(self.peekable())
    }
}