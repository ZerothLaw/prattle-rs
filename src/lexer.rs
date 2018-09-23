// lexer.rs - MIT License
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

//! # Lexer trait and simple implementation
//! 
//! ```rust
//! use prattle::lexer::{Lexer, LexerVec};
//! ```
//! 
//! The parser is looking for a type that implements a Lexer because it wants to
//! be able to peek at the next token, and receive the next one. 
//! 
//! ## Usage
//! 
//! The trait could be implemented by a stream adapter, and the parser need not know
//! more than that it implements the Lexer trait. 
//! 
//! Here is a simple wrapper around a vector as a reference/default 
//! implementation.
//!  

use std::clone::Clone;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::iter::FromIterator;

pub trait Lexer<T:  Clone + Debug + Display + Hash + Ord  > {
    fn peek(&self) -> Option<T>;
    fn next_token(&mut self) -> T;
    fn prev_token(&mut self) -> T;
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LexerVec<T:  Clone + Debug + Display + Hash + Ord > {
    inner: Vec<T>, 
    index: usize,
}

impl<T:  Clone + Debug + Display + Hash + Ord > Display for LexerVec<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "(LexerVec)")
    }
}

impl<T> LexerVec<T>
where T:  Clone + Debug + Display + Hash + Ord 
{
    pub fn new(tokens: Vec<T>) -> LexerVec<T> {
        LexerVec {
            inner: tokens, 
            index: 0
        }
    }
}

impl<T:  Clone + Debug + Display + Hash + Ord > Lexer<T> for LexerVec<T>
{
    fn peek(&self) -> Option<T> {
        if self.index < self.inner.len() {
            Some(self.inner[self.index].clone())
        } else {
            None
        }
    }

    fn next_token(&mut self) -> T {
        let t = self.inner[self.index].clone();
        self.index += 1;
        t
    }

    fn prev_token(&mut self) -> T {
        let t = self.inner[self.index].clone();
        self.index -= 1;
        t
    }
}

impl<T:  Clone + Debug + Display + Hash + Ord > FromIterator<T> for LexerVec<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut v = Vec::new();
        for i in iter {
            v.push(i);
        }
        LexerVec::new(v)
    }
}

impl<T:  Clone + Debug + Display + Hash + Ord > Extend<T> for LexerVec<T> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    //Catch Send/Sync changes
    #[test]
    fn test_lexervec_send() {
        fn assert_send<T: Send>() {}
        assert_send::<LexerVec<String>>();
    }

    #[test]
    fn test_lexervec_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<LexerVec<String>>();
    }
}