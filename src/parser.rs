// parser.rs - MIT License
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

//! # Parser
//! 
//! This is the core trait/implementation for a pratt parser. 
//! The basic algorithm is as follows:
//! > parse_expr(rbp: 0)
//! > left = null denotation of current token //ie, what is the context of this token on its own?
//! > while next token.rbp > rbp {
//! >   left = left denotation of next token //ie, what is the context of this token 
//! >                                        //combined with the context of the last node? 
//! > }
//! > return left;
//! 
//! The GeneralParser implementation here requires a provide ParserSpec and Lexer 
//! containing the tokens to be parsed. 
//! 
//! You may need to customize the behavior for specific token types, such as String, 
//! and examples/basic_spec.rs shows how. 
//! 

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::{Send, Sync};

use errors::ParseError;
use lexer::Lexer;
use node::Node;
use spec::ParserSpec;
use precedence::PrecedenceLevel;

pub trait Parser<T:  Clone + Debug + Display + Hash + Ord + Send + Sync> {
    fn parse(&mut self) -> Result<Node<T>, ParseError<T>>;
    fn parse_expr(&mut self, rbp: PrecedenceLevel) -> Result<Node<T>, ParseError<T>>;
    fn next_binds_tighter_than(&mut self, rbp: PrecedenceLevel) -> bool;
    fn consume(&mut self, end_token: T) -> Result<(), ParseError<T>>;
}

pub struct GeneralParser<T, L>
    where T: Clone + Debug + Display + Hash + Ord + Send + Sync + 'static, 
          L: Lexer<T>
{
    null_map: HashMap<T, (PrecedenceLevel, fn(&mut dyn Parser<T>, T, PrecedenceLevel) -> Result<Node<T>, ParseError<T>>)>, 
    left_map: HashMap<T, (PrecedenceLevel, PrecedenceLevel, fn(&mut dyn Parser<T>, T, PrecedenceLevel, Node<T>) -> Result<Node<T>, ParseError<T>>)>,
    lexer: L, 
}

impl<T: Clone + Debug + Display + Hash + Ord+ Send + Sync + 'static, L: Lexer<T>> GeneralParser<T, L> {
    pub fn new(spec: ParserSpec<T>, lexer: L) -> GeneralParser<T, L> {
        GeneralParser {
            null_map: spec.null_map.clone(), 
            left_map: spec.left_map.clone(), 
            lexer: lexer
        }
    }
}

impl<T: Clone + Debug + Display + Hash + Ord + Send + Sync + 'static, L: Lexer<T>> Parser<T> for GeneralParser<T, L> {
    fn parse(&mut self) -> Result<Node<T>, ParseError<T>> {
        self.parse_expr(PrecedenceLevel::Root)
    }

    fn parse_expr(&mut self, rbp: PrecedenceLevel) -> Result<Node<T>, ParseError<T>> {
        if let Some(tk) = self.lexer.peek() {
            self.lexer.next_token();
            let (lbp, func) = {
                let val = self.null_map.get(&tk);
                match val {
                    Some(val) => val.clone(), 
                    None => return Err(ParseError::MissingRule {token: tk.clone()})
                }
            };
            let mut left = func(self, tk, lbp)?;
            while self.next_binds_tighter_than(rbp) {
                let tk = self.lexer.next_token(); //implied that token exists
                let val = {
                    let v = self.left_map.get(&tk);
                    match v {
                        Some(val) => val.clone(), 
                        None => continue
                    }
                };
                let (lbp, _, func) = val;
                left = func(self, tk, lbp, left)?;
            }
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

    fn consume(&mut self, end_token: T) -> Result<(), ParseError<T>> {
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

#[cfg(test)]
mod test {
    use super::*;
    use lexer::LexerVec;
    //Catch Send/Sync changes
    #[test]
    fn test_parser_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GeneralParser<String, LexerVec<String>>>();
    }

    #[test]
    fn test_parser_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GeneralParser<String, LexerVec<String>>>();
    }
}