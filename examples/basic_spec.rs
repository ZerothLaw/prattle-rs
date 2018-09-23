use std::clone::Clone;
use std::collections::{HashMap};

#[macro_use] extern crate prattle;

use prattle::prelude::*;
use prattle::types::*;

fn basic_spec() -> ParserSpec<String> {
    let recurse_call = |parser: &mut dyn Parser<String>, token, rbp, node| {
        Ok(Node::Composite{token: token, children: vec![node, parser.parse_expr(rbp)?]})
    };
    
    let parens_call = |parser: &mut dyn Parser<String>,_, rbp| {
        let res = parser.parse_expr(rbp)?;
        parser.consume(")".into())?;
        Ok(res)
    };
    
    let mut spec = ParserSpec::new();
    
    spec.add_null_associations(vec!["ident", "number"], PrecedenceLevel::Root, |_, token, _| {Ok(Node::Simple(token))});
    spec.add_left_associations(vec!["+", "-"],  PrecedenceLevel::First, recurse_call);
    spec.add_left_associations(vec!["*", "/", "%"], PrecedenceLevel::Second, recurse_call);
    spec.add_null_assoc("(", PrecedenceLevel::First, parens_call);
    spec
}


pub struct BasicParser {
    null_map: HashMap<String, NullInfo<String>>, 
    left_map: HashMap<String, LeftInfo<String>>,
    lexer: Box<Lexer<String>>, 
}

impl BasicParser
{
    pub fn new(spec: ParserSpec<String>, lexert: Box<Lexer<String>>) -> BasicParser {
        BasicParser {
            null_map: spec.null_map.clone(), 
            left_map: spec.left_map.clone(), 
            lexer: lexert
        }
    }
    //needed so we can effectively retrieve syntax rules for arbitrary ident/number values
    fn map_string(&self, token: String) -> String {
        let mut is_ident = false;
        for chr in token.chars() {
            //println!("chr: {}", chr);
            match chr {
                '0'...'9' => {
                    continue
                }, 
                chr if (chr >= 'a' && chr <= 'z') || (chr >= 'A' && chr <= 'Z') => {
                    is_ident = true;
                    continue
                },
                _ => return token.clone()
            }
        }
        if is_ident {
            "ident".to_string()
        } else {
            "number".to_string()
        }
    }
}

impl Parser<String> for BasicParser
{
    fn parse(&mut self) -> Result<Node<String>, ParseError<String>> {
        self.parse_expr(PrecedenceLevel::Root)
    }

    fn parse_expr(&mut self, rbp: PrecedenceLevel) -> Result<Node<String>, ParseError<String>> {
        println!("parse_expr(rbp: {})", rbp);
        if let Some(tk) = self.lexer.peek() {
            self.lexer.next_token();
            let mtk = self.map_string(tk.clone());
            println!("{} => {}", tk, mtk);
            let (lbp, func) = {
                let val = self.null_map.get(&mtk);
                match val {
                    Some(val) => val.clone(), 
                    None => return Err(ParseError::MissingRule{token: tk.clone()})
                }
            };
            let mut left = func(self, tk, lbp)?;
            println!("left: {:?}", left);
            while self.next_binds_tighter_than(rbp) {
                let tk = self.lexer.next_token(); //implied that token exists
                let mtk = self.map_string(tk.clone());
                let val = {
                    let v = self.left_map.get(&mtk);
                    match v {
                        Some(val) => val.clone(), 
                        None => continue
                    }
                };
                let (lbp, _, func) = val;
                left = func(self, tk, lbp, left)?;
            }
            println!("returning {:?}", left);
            Ok(left)
        } else {
            Err(ParseError::Incomplete)
        }
    }

    fn next_binds_tighter_than(&mut self, rbp: PrecedenceLevel) -> bool {
        if let Some(tk) = self.lexer.peek() {
            if let Some((_, next_rbp, _)) = self.left_map.get(&tk) {
                *next_rbp > rbp
            } else {
                false
            }
        } else {
            false
        }
    }

    fn consume(&mut self, end_token: String) -> Result<(), ParseError<String>> {
        if let Some(tk) = self.lexer.peek() {
            if tk == end_token {
                self.lexer.next_token();
                Ok(())
            } else {
                Err(ParseError::ConsumeFailed{expected: end_token, found: tk.clone()})
            }
        } else {
            Err(ParseError::Incomplete)
        }
    }
} 

fn main() {
    let tokens = vec![
        "a", 
        "+", 
        "(", 
        "10", 
        "*",
        "(",
        "b",
        "/",
        "2",
        ")",
        "%",
        "4",
        ")",
        "-", 
        "c",
    ];
    let lexer = LexerVec::new(tokens);
    let spec = basic_spec();
    let mut parser = BasicParser::new(spec, Box::new(lexer));
    let res = parser.parse();
    println!("{:?}", res);
    println!("{:?}", parser.lexer.peek());
}
