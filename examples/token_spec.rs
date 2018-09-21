use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Error}; 
use std::hash::{Hash, Hasher};

#[macro_use] extern crate prattle;

use prattle::node::Node;
use prattle::spec::ParserSpec;
use prattle::lexer::LexerVec;
use prattle::parser::{GeneralParser, Parser};
use prattle::precedence::PrecedenceLevel;

#[derive(Debug, Clone, Eq, Ord)]
pub enum Token {
    Number(String), 
    Ident(String), 
    Add, Sub, 
    Mul, Div, Mod, 
    LParens, RParens
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        let lhs = match self {
            Token::Number(_) => 0, 
            Token::Ident(_) => 1, 
            Token::Add => 2, 
            Token::Sub => 3,
            Token::Mul => 4,
            Token::Div => 5,
            Token::Mod => 6,
            Token::LParens => 7,
            Token::RParens => 8,
        };
        let rhs = match other {
            Token::Number(_) => 0, 
            Token::Ident(_) => 1, 
            Token::Add => 2, 
            Token::Sub => 3,
            Token::Mul => 4,
            Token::Div => 5,
            Token::Mod => 6,
            Token::LParens => 7,
            Token::RParens => 8,
        };
        lhs.eq(&rhs)
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Token) -> Option<Ordering> {
        let lhs = match self {
            Token::Number(_) => 0, 
            Token::Ident(_) => 1, 
            Token::Add => 2, 
            Token::Sub => 3,
            Token::Mul => 4,
            Token::Div => 5,
            Token::Mod => 6,
            Token::LParens => 7,
            Token::RParens => 8,
        };
        let rhs = match other {
            Token::Number(_) => 0, 
            Token::Ident(_) => 1, 
            Token::Add => 2, 
            Token::Sub => 3,
            Token::Mul => 4,
            Token::Div => 5,
            Token::Mod => 6,
            Token::LParens => 7,
            Token::RParens => 8,
        };
        lhs.partial_cmp(&rhs)
    }
}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let lhs = match self {
            Token::Number(_) => 0, 
            Token::Ident(_) => 1, 
            Token::Add => 2, 
            Token::Sub => 3,
            Token::Mul => 4,
            Token::Div => 5,
            Token::Mod => 6,
            Token::LParens => 7,
            Token::RParens => 8,
        };
        lhs.hash(state);
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", match *self {
            Token::Number(ref s) => format!("(Number: {})", s), 
            Token::Ident(ref s) => format!("(Ident: {})", s), 
            Token::Add => "Add".to_string(), 
            Token::Sub => "Sub".to_string(),
            Token::Mul => "Mul".to_string(),
            Token::Div => "Div".to_string(),
            Token::Mod => "Mod".to_string(),
            Token::LParens => "(".to_string(),
            Token::RParens => ")".to_string(),
        })
    }
}

fn token_spec() -> ParserSpec<Token> {
    let mut spec = ParserSpec::new();
    add_null_assoc!(spec, PrecedenceLevel::Root, (Token::Number("".to_string()), Token::Ident("".to_string())) => |_, token: Token, _| {
        Ok(Node::Simple(token.clone()))
    });
    add_left_assoc!(spec, PrecedenceLevel::First, (Token::Add, Token::Sub) => |parser, token: Token, lbp: PrecedenceLevel, node: Node<Token>| {
        Ok(Node::Composite { token: token.clone(), children: vec![node, parser.parse_expr(lbp)?] } )
    } );
    add_left_assoc!(spec, PrecedenceLevel::Second, (Token::Mul, Token::Div, Token::Mod) => |parser, token: Token, lbp: PrecedenceLevel, node: Node<Token>| {
        Ok(Node::Composite { token: token.clone(), children: vec![node, parser.parse_expr(lbp)?] } )
    } );
    add_null_assoc!(spec, PrecedenceLevel::First, (Token::LParens) => |parser, _, lbp| {
        let res = parser.parse_expr(lbp)?;
        parser.consume(Token::RParens)?;
        Ok(res)
    });

    spec
}

fn main() {
    let tokens = vec![
        Token::Ident("a".to_string()), 
        Token::Add,
        Token::LParens,
        Token::Number("10".to_string()), 
        Token::Mul,
        Token::LParens,
        Token::Ident("b".to_string()),
        Token::Div,
        Token::Number("2".to_string()),
        Token::RParens,
        Token::Mod, 
        Token::Number("4".to_string()),
        Token::RParens,
        Token::Sub,
        Token::Ident("c".to_string()),
    ];
    let lexer = LexerVec::new(tokens);
    let spec = token_spec();
    let mut parser = GeneralParser::new(spec, lexer);
    let res = parser.parse();
    println!("{:?}", res);
    //println!("{:?}", parser.lexer.peek());
}