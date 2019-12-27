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
//! The GeneralParser implementation here requires a provided ParserSpec and Lexer 
//! containing the tokens to be parsed. 

use std::collections::HashMap;
use std::marker::{Send, Sync};
use std::mem::{Discriminant, discriminant};

use prelude::*;
use types::*;

/// Parser trait. Theoretically, one could use different parser impls during parse of a 
/// language and so the syntax rules need to not be tied to any specific Parser impl. 
/// Hence why the ParserSpec uses closures with the signature ```&mut dyn Parser<T>```
pub trait Parser<T: Token + Send + Sync + 'static, Node = SimpleNode<T>> {
    fn parse(&mut self) -> Result<Node, ParseError<T>>;
    fn parse_expr(&mut self, rbp: PrecedenceLevel) -> Result<Node, ParseError<T>>;
    /// parse_sequence impl can be a bit complex - 
    /// basically it *should* call parse_expr repeatedly with prec_level, 
    /// while consuming an (optional) separator token, and then consuming 
    /// an end token, or if there is no end token, consuming until we reach Incomplete
    fn parse_sequence(&mut self, prec_level: PrecedenceLevel, sep: Option<T>, end_token: Option<T>) -> Vec<Result<Node, ParseError<T>>>;
    fn next_binds_tighter_than(&mut self, rbp: PrecedenceLevel) -> bool;
    fn consume(&mut self, end_token: T) -> Result<(), ParseError<T>>;
}

/// General implementation of Parser trait. This implementation should work for any 
/// valid set of Syntax rules. 
/// A second generic, `L`, is added in order to allow us to decouple this impl from any specific 
/// Lexer impl. 
/// Here we just copy and consume the fields of the ParserSpec in order to set this up.
/// If we instead held a ParserSpec internally, we get borrow checker error messages. 
/// Consider: 
/// ```Rust
/// let null_info = self.spec.get_null(&tk);
/// ```
/// This inherently borrows self.spec, which then borrows self as an outcome. 
/// If instead you own the HashMaps, only those specific members are considered 
/// borrowed by borrowck. 
pub struct GeneralParser<T, L, Node = SimpleNode<T>>
    where T: Token + Send + Sync + 'static, 
          L: Lexer<T>
{
    null_map: HashMap<Discriminant<T>, NullInfo<T, Node>>, 
    left_map: HashMap<Discriminant<T>, LeftInfo<T, Node>>,
    lexer: L, 
}

/// GeneralParser impl
/// Wraps trait methods to allow users to only need to import this, without 
/// the trait. Also offers a compile time check that GeneralParser still
/// impls Parser correctly. 
#[allow(dead_code)]
impl<T: Token + Send + Sync + 'static, L: Lexer<T>, Node> GeneralParser<T, L, Node> {
    pub fn new(spec: ParserSpec<T, Node>, lexer: L) -> GeneralParser<T, L, Node> {
        let (null_map, left_map) = spec.maps();
        GeneralParser {
            null_map: null_map,
            left_map: left_map,
            lexer: lexer
        }
    }

    fn parse(&mut self) -> Result<Node, ParseError<T>> {
        self.parse_expr(PrecedenceLevel::Root)
    }

    fn parse_expr(&mut self, rbp: PrecedenceLevel) -> Result<Node, ParseError<T>> {
        <Self as Parser<T, Node>>::parse_expr(self, rbp)
    }
    
    fn parse_sequence(&mut self, prec_level: PrecedenceLevel, sep: Option<T>, end_token: Option<T>) -> Vec<Result<Node, ParseError<T>>>{
        <Self as Parser<T, Node>>::parse_sequence(self, prec_level, sep, end_token)
    }

    fn next_binds_tighter_than(&mut self, rbp: PrecedenceLevel) -> bool {
        <Self as Parser<T, Node>>::next_binds_tighter_than(self, rbp)
    }

    fn consume(&mut self, end_token: T) -> Result<(), ParseError<T>> {
        <Self as Parser<T, Node>>::consume(self, end_token)
    }
}

impl<T: Token + Send + Sync + 'static, L: Lexer<T>, Node> Parser<T, Node> for GeneralParser<T, L, Node> {
    fn parse(&mut self) -> Result<Node, ParseError<T>> {
        self.parse_expr(PrecedenceLevel::Root)
    }

    fn parse_expr(&mut self, rbp: PrecedenceLevel) -> Result<Node, ParseError<T>> {
        if let Some(tk) = self.lexer.peek() {
            self.lexer.next_token();
            let (lbp, func) = {
                let val = self.null_map.get(&discriminant(&tk));
                match val {
                    Some(val) => val.clone(), 
                    None => return Err(ParseError::MissingRule {token: tk.clone(), ty: "Null".into()})
                }
            };
            let mut left = func(self, tk, lbp)?;
            while self.next_binds_tighter_than(rbp) {
                let tk = self.lexer.next_token(); //implied that token exists
                let val = {
                    let v = self.left_map.get(&discriminant(&tk));
                    match v {
                        Some(val) => val.clone(), 
                        None => return Err(ParseError::MissingRule {token: tk.clone(), ty: "Left".into()})
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

    fn parse_sequence(&mut self, prec_level: PrecedenceLevel, sep: Option<T>, end_token: Option<T>) -> Vec<Result<Node, ParseError<T>>>{
        let mut results = Vec::new();
        loop {
            let res = self.parse_expr(prec_level);
            if res.is_ok() {
                match &sep {
                    &Some(ref sep) => {
                        match self.consume(sep.clone()) {
                            Ok(()) => {},  
                            Err(ParseError::ConsumeFailed{expected: _, ref found}) => {
                                match &end_token {
                                    &Some(ref end_token) => {
                                        if end_token == found {
                                            match self.consume(found.clone()) {
                                                Ok(()) => break,
                                                Err(pe) => {
                                                    results.push(Err(pe));
                                                }
                                            }
                                        } else {
                                            results.push(Err(ParseError::ConsumeFailed{expected: sep.clone(), found: found.clone()}));
                                        }
                                    }, 
                                    &None => {
                                        results.push(Err(ParseError::ConsumeFailed{expected: sep.clone(), found: found.clone()}));
                                    }
                                };
                                break
                            }, 
                            Err(pe) => results.push(Err(pe))
                        }
                    }, 
                    None => {},
                }
            } else {
                match (&res, end_token) {
                    (&Err(ParseError::Incomplete), None) => {
                        return results;
                    }, 
                    _ => {}
                };
                results.push(res);
                break
            }
            results.push(res);
        };
        results
    }

    fn next_binds_tighter_than(&mut self, rbp: PrecedenceLevel) -> bool {
        if let Some(tk) = self.lexer.peek() {
            if let Some((_, next_rbp, _)) = self.left_map.get(&discriminant(&tk)) {
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
