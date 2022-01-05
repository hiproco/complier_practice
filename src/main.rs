use std::{convert::Infallible, iter::successors, ops::Not, str::FromStr};
use AST::*;

fn main() {
    let code = "";

    let parsed = code.parse::<Tokens>().unwrap();

    let ast = AST::parse(parsed);

    // eprintln!("{:?}", parsed);
}

mod AST {
    use std::{str::FromStr, iter::successors, convert::Infallible, ops::Not};


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

    pub fn parse(Tokens { stream }: Tokens) -> Result<RootAST, ParseError> {
        let mut tokens = stream.into_iter();
        if let Some(token) = tokens.next() {
            match token {
                Token::Identifier(_) => assignment(tokens),
                Token::Number(_) => todo!(),
                Token::StringLiteral(_) => return Err("expected function keyword, \"fn\""),
                Token::FnKey => todo!(),
                Token::ExternKey => todo!(),
            }
        }
        expression(tokens);
        Ok(RootAST {
            node: NodeAST::Expression,
        })
    }

    fn assignment<I: Iterator<Item = Token>>(mut tokens: I) -> Result<NodeAST,ParseError> {
        if let Some(Token::Identifier(name)) = tokens.next() {
            let args = Vec::new();
            let tokens = tokens.peekable();
            while let Some(Token::Identifier(arg)) = tokens.peek() { 
                args.push(arg);
                tokens.next(); 
            }
            if let Some(Token::Divider) = tokens.next() {
                if let Some(Token::Identifier(types)) = tokens.next() {
                    if let Some(Token::FunType) = tokens.next() {
                        if let Some(Token::Identifier(rtype)) = tokens.next() {
                            if let Some(Token::Assign) = tokens.next() {
                                let expression = expression(tokens)?;
                                return Ok(NodeAST::Function{ name, args, expression })
                            }
                        }
                    }
                }
            } else if let Some(Token::Divider) {

            }
        }
    }

    fn expression<I: Iterator<Item = Token>>(mut tokens: I) -> Result<NodeAST, ParseError> {
        if let Some(t) = tokens.next() {
            if let Token::Identifier(_) = t {}
        }
        Ok(NodeAST::Expression)
    }

    type ParseError = &'static str;

    fn function<I: Iterator<Item = Token>>(mut tokens: I) -> Result<NodeAST, ParseError> {
        if let Some(Token::FnKey) = tokens.next() {
            if let Some(Token::Identifier(function_name)) = tokens.next() {
                let expression = Box::new(expression(tokens));
                Ok(NodeAST::Function {
                    function_name,
                    expression,
                })
            } else {
                Err("expected Identifier")
            }
        } else {
            Err("Expected function keyword, \"fn\"")
        }
    }

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
        Assign
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
