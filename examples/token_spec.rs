use std::fmt::{Display, Formatter, Error}; 

#[macro_use] extern crate prattle;

use prattle::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum CToken {
    Number(String), 
    Ident(String), 
    Add, Sub, 
    Mul, Div, Mod, 
    LParens, RParens
}

impl Display for CToken {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", match *self {
            CToken::Number(ref s) => format!("(Number: {})", s), 
            CToken::Ident(ref s) => format!("(Ident: {})", s), 
            CToken::Add => "Add".to_string(), 
            CToken::Sub => "Sub".to_string(),
            CToken::Mul => "Mul".to_string(),
            CToken::Div => "Div".to_string(),
            CToken::Mod => "Mod".to_string(),
            CToken::LParens => "(".to_string(),
            CToken::RParens => ")".to_string(),
        })
    }
}

fn token_spec() -> Result<ParserSpec<CToken>, SpecificationError<CToken>> {
    let mut spec = ParserSpec::new();
    add_null_assoc!(spec, PrecedenceLevel::Root, (CToken::Number("".to_string()), CToken::Ident("".to_string())) => |_, token: CToken, _| {
        Ok(Node::Simple(token.clone()))
    });
    add_left_assoc!(spec, PrecedenceLevel::First, (CToken::Add, CToken::Sub) => |parser, token, lbp, node| {
        Ok(Node::Composite { token: token.clone(), children: vec![node, parser.parse_expr(lbp)?] } )
    } );
    add_left_assoc!(spec, PrecedenceLevel::Second, (CToken::Mul, CToken::Div, CToken::Mod) => |parser, token, lbp, node| {
        Ok(Node::Composite { token: token.clone(), children: vec![node, parser.parse_expr(lbp)?] } )
    } );
    add_null_assoc!(spec, PrecedenceLevel::First, (CToken::LParens) => |parser, _, lbp| {
        let res = parser.parse_expr(lbp)?;
        parser.consume(CToken::RParens)?;
        Ok(res)
    });
    Ok(spec)
}

fn main() {
    let tokens = vec![
        CToken::Ident("a".to_string()), 
        CToken::Add,
        CToken::LParens,
        CToken::Number("10".to_string()), 
        CToken::Mul,
        CToken::LParens,
        CToken::Ident("b".to_string()),
        CToken::Div,
        CToken::Number("2".to_string()),
        CToken::RParens,
        CToken::Mod, 
        CToken::Number("4".to_string()),
        CToken::RParens,
        CToken::Sub,
        CToken::Ident("c".to_string()),
    ];
    let lexer = LexerVec::new(tokens);
    let spec = token_spec().expect("Should work.");
    let mut parser = GeneralParser::new(spec, lexer);
    let res = parser.parse();
    println!("{:?}", res);
}