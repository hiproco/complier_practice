use human_regex::*;
use std::{
    convert::Infallible,
    iter::{successors, Peekable},
    ops::Not,
    str::{Chars, FromStr},
};
use AST::*;
//start without comments

// pre stack or post stack

// clone base or reference based

#[test]
fn a_white() {
    assert_eq!('a'.is_whitespace(), false);
    assert_eq!('\n'.is_whitespace(), true);
}

fn main() {
    let code = "\'  char \'a # and \n \" string  \"";

    eprintln!(
        "{:?}",
        Tokenizer(code.chars().peekable())
            .map(|t| {
                if let Token::Symbol(s) = t {
                    let trimed = s.trim().to_string();
                    if trimed.is_empty() {
                        Token::None
                    } else {
                        Token::Symbol(trimed)
                    }
                } else {
                    t
                }
            })
            .filter(|t| *t != Token::None)
            .collect::<Vec<_>>()
    );
}

struct Tokenizer<'a>(Peekable<Chars<'a>>);

trait Tokenize<'a> :Iterator {
    fn tokenize(self) -> Tokenizer<'a>;
}

impl<'a> Tokenize<'a> for Chars<'a> {
    fn tokenize(self) -> Tokenizer<'a> {
        Tokenizer(self.peekable())
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let chars = &mut self.0;
        if let Some(c) = chars.next() {
            Some(match c {
                '#' => Token::Comment(String::from_iter(chars.by_ref().take_while(|c| *c != '\n'))),
                '\'' => Token::Char(String::from_iter(chars.by_ref().take_while(|c| *c != '\''))),
                '"' => Token::String(String::from_iter(chars.by_ref().take_while(|c| *c != '"'))),
                _ => {
                    let mut s = String::from(c);
                    while let Some(ch) = chars.next_if(|c| !['#', '\'', '"'].contains(c)) {
                        s.push(ch);
                    }
                    Token::Symbol(s)
                }
            })
            // Some(Token::Symbol(s))
        } else {
            None
        }
    }
}

// trait Tokenizer : Iterator {
//     fn get_token(&mut self) -> Token;
// }

// impl<'a> Tokenizer for Chars<'a> {
//     fn get_token(&mut self) -> Token {
//         if let Some(c) = self.next() {
//             match c {
//                 _ => (),
//             }
//             let mut s = String::from(c);
//             s.extend(self.by_ref().take_while(|ch| false));
//             Token::Symbol(s)
//         } else {
//             Token::EOF
//         }
//     }
// }

#[derive(Debug, PartialEq)]
enum Token {
    Comment(String),
    Char(String),
    String(String),
    Number(u32),
    Symbol(String),
    EOF,
    None,
}

mod AST {
    use std::{convert::Infallible, iter::successors, ops::Not, str::FromStr};

    pub struct RootAST {
        node: NodeAST,
    }

    enum NodeAST {
        Function {
            name: String,
            args: Vec<String>,
            expression: Box<NodeAST>,
        },
        Expression,
    }

    // pub fn parse(Tokens { stream }: Tokens) -> Result<RootAST, ParseError> {
    //     let mut tokens = stream.into_iter();
    //     if let Some(token) = tokens.next() {
    //         match token {
    //             Token::Identifier(_) => assignment(tokens),
    //             Token::Number(_) => todo!(),
    //             Token::StringLiteral(_) => return Err("expected function keyword, \"fn\""),
    //             Token::FnKey => todo!(),
    //             Token::ExternKey => todo!(),
    //         }
    //     }
    //     expression(tokens);
    //     Ok(RootAST {
    //         node: NodeAST::Expression,
    //     })
    // }

    // fn assignment<I: Iterator<Item = Token>>(mut tokens: I) -> Result<NodeAST,ParseError> {
    //     if let Some(Token::Identifier(name)) = tokens.next() {
    //         let args = Vec::new();
    //         let tokens = tokens.peekable();
    //         while let Some(Token::Identifier(arg)) = tokens.peek() {
    //             args.push(arg);
    //             tokens.next();
    //         }
    //         if let Some(Token::Divider) = tokens.next() {
    //             if let Some(Token::Identifier(types)) = tokens.next() {
    //                 if let Some(Token::FunType) = tokens.next() {
    //                     if let Some(Token::Identifier(rtype)) = tokens.next() {
    //                         if let Some(Token::Assign) = tokens.next() {
    //                             let expression = expression(tokens)?;
    //                             return Ok(NodeAST::Function{ name, args, expression })
    //                         }
    //                     }
    //                 }
    //             }
    //         } else if let Some(Token::Divider) {

    //         }
    //     }
    // }

    // fn expression<I: Iterator<Item = Token>>(mut tokens: I) -> Result<NodeAST, ParseError> {
    //     if let Some(t) = tokens.next() {
    //         if let Token::Identifier(_) = t {}
    //     }
    //     Ok(NodeAST::Expression)
    // }

    // type ParseError = &'static str;

    // fn function<I: Iterator<Item = Token>>(mut tokens: I) -> Result<NodeAST, ParseError> {
    //     if let Some(Token::FnKey) = tokens.next() {
    //         if let Some(Token::Identifier(function_name)) = tokens.next() {
    //             let expression = Box::new(expression(tokens));
    //             Ok(NodeAST::Function {
    //                 function_name,
    //                 expression,
    //             })
    //         } else {
    //             Err("expected Identifier")
    //         }
    //     } else {
    //         Err("Expected function keyword, \"fn\"")
    //     }
    // }

    #[derive(Debug)]
    pub struct Tokens {
        stream: Vec<Token>,
    }

    impl FromStr for Tokens {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Tokens {
                stream: s
                    .split('"')
                    .zip(successors(Some(false), |sl| Some(sl.not())))
                    .flat_map(|(s, string_literal)| {
                        string_literal
                            .then::<Box<dyn Iterator<Item = &str>>, _>(|| {
                                Box::new(std::iter::once(s))
                            })
                            .or_else(|| Some(Box::new(s.split_whitespace())))
                            .expect("no iterator")
                    })
                    .flat_map(|s| s.split_terminator([',', ';'].as_slice()))
                    .filter_map(|s| s.parse::<Token>().ok())
                    .collect(),
            })
        }
    }

    #[derive(Debug)]
    enum PreToken {
        Identifier(String),
        Number(String),
        StringLiteral(String),
    }

    impl FromStr for PreToken {
        type Err = Infallible; // &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut word = s.chars();
            let (p, r): (fn(char) -> bool, Result<PreToken, _>) =
                word.next().filter(|c| c.is_alphabetic()).map_or(
                    (char::is_numeric, Ok(PreToken::Number(s.to_string()))),
                    |_| {
                        (
                            char::is_alphanumeric,
                            Ok(PreToken::Identifier(s.to_string())),
                        )
                    },
                );
            if word.all(p) {
                r
            } else {
                Ok(PreToken::StringLiteral(s.to_string()))
                // Err("not token")
            }
        }
    }

    #[derive(Debug)]
    enum Token {
        Identifier(String),
        Number(i32),
        StringLiteral(String),
        FnKey,
        ExternKey,
        Divider,
        FunType,
        Assign,
    }

    impl FromStr for Token {
        type Err = Infallible;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.parse::<PreToken>().map(Token::from)
        }
    }

    impl From<PreToken> for Token {
        fn from(pt: PreToken) -> Self {
            match pt {
                PreToken::Identifier(s) => match s.as_str() {
                    "fn" => Token::FnKey,
                    "extern" => Token::ExternKey,
                    _ => Token::Identifier(s),
                },
                PreToken::Number(n) => Token::Number(n.parse().unwrap()),
                PreToken::StringLiteral(s) => Token::StringLiteral(s),
            }
        }
    }
}
