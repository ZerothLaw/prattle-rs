// spec.rs - MIT License
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

//! # ParserSpec 
//! The parser spec encapsulates the mapping of tokens with left/right binding 
//! precedences, as well as the actual syntax rule execution (such as a recursive
//! call to the parser.)
//! 
//! ## Closure types:
//! NullDenotation<T> = fn(&mut dyn Parser<T>, T, u32) -> Result<Node<T>, ParseError<T>>;
//! LeftDenotation<T> = fn(&mut dyn Parser<T>, T, u32, Node<T>) -> Result<Node<T>, ParseError<T>>;
//! 
//! where T is your token type. 
//! 
//! Tokens must implement Clone + Debug + Display + Eq + Hash + PartialOrd + PartialEq.
//! 

use std::clone::Clone;
use std::collections::{HashMap};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::{Send, Sync};

use errors::ParseError;
use node::Node;
use parser::Parser;
use precedence::PrecedenceLevel;

pub type NullDenotation<T> = fn(&mut dyn Parser<T>, T, PrecedenceLevel) -> Result<Node<T>, ParseError<T>>;
pub type LeftDenotation<T> = fn(&mut dyn Parser<T>, T, PrecedenceLevel, Node<T>) -> Result<Node<T>, ParseError<T>>;

#[derive(Clone)]
pub struct ParserSpec<T:  Clone + Debug + Display + Hash + Ord + Send + Sync + 'static> {
    pub null_map: HashMap<T, (PrecedenceLevel, NullDenotation<T>)>, 
    pub left_map: HashMap<T, (PrecedenceLevel, PrecedenceLevel, LeftDenotation<T>)>,
}

impl<T> ParserSpec<T>
where T:  Clone + Debug + Display + Hash + Ord + Send + Sync + 'static
{
    pub fn new() -> ParserSpec<T> {
        ParserSpec {
            null_map: HashMap::new(), 
            left_map: HashMap::new(),
        }
    }

    pub fn add_null_assoc(&mut self, token: T, bp: PrecedenceLevel, func: NullDenotation<T>) {
        if !self.null_map.contains_key(&token) {
            self.null_map.insert(token.clone(), (bp, func));
        }
    }

    pub fn add_left_assoc(&mut self, token: T, bp: PrecedenceLevel, func: LeftDenotation<T>) {
        if !self.left_map.contains_key(&token) {
            self.left_map.insert(token.clone(), (bp, bp, func));
        }
    }

    pub fn add_left_right_assoc(&mut self, token: T, lbp: PrecedenceLevel, rbp: PrecedenceLevel, func: LeftDenotation<T>) {
        if !self.left_map.contains_key(&token) {
            self.left_map.insert(token.clone(), (lbp, rbp, func));
        }
    }

    pub fn add_multi_null_assoc<I: IntoIterator<Item=T>>(&mut self, tokens: I, bp: PrecedenceLevel, func: NullDenotation<T>) {
        for token in tokens {
            self.add_null_assoc(token, bp, func)
        }
    }

    pub fn add_multi_left_assoc<I: IntoIterator<Item=T>>(&mut self, tokens: I, bp: PrecedenceLevel, func: LeftDenotation<T>) {
        for token in tokens {
            self.add_left_assoc(token, bp, func)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    //Catch Send/Sync changes
    #[test]
    fn test_parserspec_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ParserSpec<String>>();
    }

    #[test]
    fn test_parserspec_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ParserSpec<String>>();
    }
}