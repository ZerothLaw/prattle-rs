// ebnf_spec.rs - MIT License
//  MIT License
//  Copyright (c) 2018 Tyler Laing (ZerothLaw)
// 
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
// 
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
// 
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.

use std::fmt;

extern crate prattle;

use prattle::prelude::*;

/*
    ebnf grammar rules:
        grammar : rule + ;
        rule    : nonterminal ':' productionrule ';' ;
        productionrule : production [ '|' production ] * ;
        production : term * ;
        term : element repeats ;
        element : LITERAL | IDENTIFIER | '[' productionrule ']' ;
        repeats : [ '*' | '+' ] NUMBER ? | NUMBER ? | '?' ;
*/
#[derive(Clone, Debug, PartialEq, Hash)]
pub enum EBNFToken {
    Star, 
    Plus, 
    Question, 
    Pipe, 
    LBrace, RBrace, 
    Semicolon, 
    Colon,
    Number(String), Ident(String), String(String),
    //Parse-only tokens
    Repeats, 
    Opt,
    Group,
    Sequence,
    Rule,
}

impl fmt::Display for EBNFToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:#?}", self)
    }
}

fn ebnf_spec() -> Result<ParserSpec<EBNFToken>, SpecificationError<EBNFToken>> {
    let mut spec = ParserSpec::new();

    spec.add_null_associations(vec![EBNFToken::Ident("".to_string()), EBNFToken::String("".to_string())], PrecedenceLevel::Root, |_, tk, _| {
        Ok(SimpleNode::Plain(tk))
    })?;

    spec.add_left_assoc(EBNFToken::Colon, PrecedenceLevel::First, |parser, _, _, node| {
            Ok(SimpleNode::Composite{token: EBNFToken::Rule, children: vec![node, parser.parse_expr(PrecedenceLevel::First)?]})
        }
    )?;
    spec.add_left_assoc(EBNFToken::Pipe, PrecedenceLevel::Second, |parser, tk, _, node| {
            Ok(SimpleNode::Composite{token: tk, children: vec![node, parser.parse_expr(PrecedenceLevel::Second)?]})
        }
    )?;
    spec.add_left_associations(vec![EBNFToken::Star, EBNFToken::Plus], PrecedenceLevel::Third, |_, tk, _, node| {
            Ok(SimpleNode::Composite{token: EBNFToken::Repeats, children: vec![node, SimpleNode::Plain(tk)]})
        }
    )?;
    spec.add_left_assoc(EBNFToken::Number("".to_string()), PrecedenceLevel::Third, |_, tk, _, node| {
        match node {
            SimpleNode::Composite{token: EBNFToken::Repeats, mut children } => {
                Ok(SimpleNode::Composite{ token: EBNFToken::Repeats, children: {children.push(SimpleNode::Plain(tk)); children}})
            }
            _ => {
                Ok(SimpleNode::Composite{ token: EBNFToken::Repeats, children: vec![node, SimpleNode::Plain(tk)]})
            }, 

        }
    })?;
    spec.add_left_associations(vec![EBNFToken::String("".to_string()), EBNFToken::Ident("".to_string())], PrecedenceLevel::Third, |_, tk, _, node| {
        match node {
            SimpleNode::Composite{token: c_tk, mut children} => {
                children.push(SimpleNode::Plain(tk));
                Ok(SimpleNode::Composite{token: c_tk, children: children})
            }, 
            SimpleNode::Plain(n_tk) => Ok(SimpleNode::Composite{token: EBNFToken::Sequence, children: vec![SimpleNode::Plain(n_tk), SimpleNode::Plain(tk)]})
        }
    })?;
    spec.add_null_assoc(EBNFToken::LBrace, PrecedenceLevel::Root, |parser, _, _| {
        let inner = parser.parse_expr(PrecedenceLevel::First)?;
        parser.consume(EBNFToken::RBrace)?;
        Ok(SimpleNode::Composite{token: EBNFToken::Group, children: vec![inner]})
    })?;
    spec.add_left_assoc(EBNFToken::LBrace, PrecedenceLevel::Fourth, |parser, _, _, node| {
        let inner = parser.parse_expr(PrecedenceLevel::First)?;
        parser.consume(EBNFToken::RBrace)?;
        Ok(SimpleNode::Composite{token: EBNFToken::Sequence, children: vec![node, SimpleNode::Composite{token: EBNFToken::Group, children: vec![inner]}]}) //change this logic for token: Rule
    })?;
    spec.add_left_assoc(EBNFToken::Question, PrecedenceLevel::Third, |_, _, _, node| {
        Ok(SimpleNode::Composite{token: EBNFToken::Opt, children: vec![node]})
    })?;

    Ok(spec)
}

fn main() {
    let spec = ebnf_spec().unwrap();
    let lexer = LexerVec::new(
        vec![
            //grammar : rule + ;
            EBNFToken::Ident("grammar".to_string()), 
            EBNFToken::Colon, 
            EBNFToken::Ident("rule".to_string()),
            EBNFToken::Plus,
            EBNFToken::Semicolon,
            //rule    : nonterminal ':' productionrule ';' ;
            EBNFToken::Ident("rule".to_string()), 
            EBNFToken::Colon, 
            EBNFToken::Ident("nonterminal".to_string()), 
            EBNFToken::String(":".to_string()), 
            EBNFToken::Ident("productionrule".to_string()),
            EBNFToken::String(";".to_string()), 
            EBNFToken::Semicolon, 
            //productionrule : production [ '|' production ] * ;
            EBNFToken::Ident("productionrule".to_string()),
            EBNFToken::Colon, 
            EBNFToken::Ident("production".to_string()),
            EBNFToken::LBrace,
            EBNFToken::String("|".to_string()),
            EBNFToken::Ident("production".to_string()),
            EBNFToken::RBrace, 
            EBNFToken::Star, 
            EBNFToken::Semicolon,
            //production : term * ;
            EBNFToken::Ident("production".to_string()),
            EBNFToken::Colon, 
            EBNFToken::Ident("term".to_string()),
            EBNFToken::Star, 
            EBNFToken::Semicolon,
            //term : element repeats ;
            EBNFToken::Ident("term".to_string()),
            EBNFToken::Colon, 
            EBNFToken::Ident("element".to_string()),
            EBNFToken::Ident("repeats".to_string()),
            EBNFToken::Semicolon,
            //element : LITERAL | IDENTIFIER | '[' productionrule ']' ;
            EBNFToken::Ident("repeats".to_string()),
            EBNFToken::Colon, 
            EBNFToken::Ident("LITERAL".to_string()),
            EBNFToken::Pipe, 
            EBNFToken::Ident("IDENTIFIER".to_string()),
            EBNFToken::Pipe, 
            EBNFToken::String("[".to_string()), 
            EBNFToken::Ident("productionrule".to_string()), 
            EBNFToken::String("]".to_string()),
            EBNFToken::Semicolon,
            //repeats : [ '*' | '+' ] NUMBER ? | NUMBER ? | '?' ;
            EBNFToken::Ident("repeats".to_string()),
            EBNFToken::Colon, 
            EBNFToken::LBrace, 
            EBNFToken::String("*".to_string()), 
            EBNFToken::Pipe, 
            EBNFToken::String("+".to_string()), 
            EBNFToken::RBrace, 
            EBNFToken::Ident("NUMBER".to_string()), 
            EBNFToken::Question, 
            EBNFToken::Pipe, 
            EBNFToken::Ident("NUMBER".to_string()), 
            EBNFToken::Question, 
            EBNFToken::Pipe, 
            EBNFToken::String("?".to_string()),
            EBNFToken::Semicolon,
        ]
    );
    let mut parser = GeneralParser::new(spec, lexer);
    println!("{:?}", parser.parse_sequence(PrecedenceLevel::Root, Some(EBNFToken::Semicolon), None));
}
