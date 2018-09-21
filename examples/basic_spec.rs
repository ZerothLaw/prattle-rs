use std::clone::Clone;
use std::collections::{HashMap};

extern crate prattle;
use prattle::errors::ParseError;
use prattle::lexer::{Lexer, LexerVec};
use prattle::node::Node;
use prattle::parser::Parser;
use prattle::precedence::PrecedenceLevel;
use prattle::spec::ParserSpec;


macro_rules! multi {
    ($spec:ident, null, $lbp:expr, ($($token:expr),*) => $cl:expr) => {
        $(
            $spec.add_null_assoc($token.to_string(), $lbp, $cl);
        )*
        
    };
    ($spec:ident, left, $lbp:expr, ($($token:expr),*) => $cl:expr) => {
        $(
            $spec.add_left_assoc($token.to_string(), $lbp, $cl);
        )*
    };
}

fn basic_spec() -> ParserSpec<String> {
    let null_node = |_parser: &mut dyn Parser<String>, token: String, _rbp: PrecedenceLevel| -> Result<Node<String>, ParseError<String>> {
        Ok(Node::Simple(token.to_string()))
    };
    let recurse_call = |parser: &mut dyn Parser<String>, token: String, rbp: PrecedenceLevel, node: Node<String>| -> Result<Node<String>, ParseError<String>> {
        Ok(Node::Composite{token: token.to_string(), children: vec![node, parser.parse_expr(rbp)?]})
    };
    
    let parens_call = |parser: &mut dyn Parser<String>, _token: String, rbp: PrecedenceLevel| -> Result<Node<String>, ParseError<String>> {
        let res = parser.parse_expr(rbp)?;
        parser.consume(")".to_string())?;
        Ok(res)
    };
    
    let mut spec = ParserSpec::new();
    
    multi!(spec, null, PrecedenceLevel::Root, ("ident", "number") => null_node);
    multi!(spec, left, PrecedenceLevel::First, ("+", "-") => recurse_call);
    multi!(spec, left, PrecedenceLevel::Second, ("*", "/", "%") => recurse_call);
    multi!(spec, null, PrecedenceLevel::First, ("(") => parens_call);
    spec
}


pub struct BasicParser {
    null_map: HashMap<String, (PrecedenceLevel, fn(&mut dyn Parser<String>, String, PrecedenceLevel) -> Result<Node<String>, ParseError<String>>)>, 
    left_map: HashMap<String, (PrecedenceLevel, PrecedenceLevel, fn(&mut dyn Parser<String>, String, PrecedenceLevel, Node<String>) -> Result<Node<String>, ParseError<String>>)>,
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
        "a".to_string(), 
        "+".to_string(), 
        "(".to_string(), 
        "10".to_string(), 
        "*".to_string(),
        "(".to_string(),
        "b".to_string(),
        "/".to_string(),
        "2".to_string(),
        ")".to_string(),
        "%".to_string(),
        "4".to_string(),
        ")".to_string(),
        "-".to_string(), 
        "c".to_string(),
    ];
    let lexer = LexerVec::new(tokens);
    let spec = basic_spec();
    let mut parser = BasicParser::new(spec, Box::new(lexer));
    let res = parser.parse();
    println!("{:?}", res);
    println!("{:?}", parser.lexer.peek());
}
