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

use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;

use token::Token;

///Basic lexer trait that Parser implementations should use. 
/// How one implements it is entirely up to implementors. 
/// A basic implementation around a Vec is provided for convenience.
pub trait Lexer<T: Token> {
    ///Parser impls should use this before *every* next_token call. 
    fn peek(&self) -> Option<T>;
    ///Moves Lexer forward to the next token, returning it. 
    fn next_token(&mut self) -> T;
    //Moves Lexer backward to previous token, returning it.
    fn prev_token(&mut self) -> T;
}

/// Basic implementation of the Lexer trait
/// Just as simple wrapper around a Vec, with an index that can
/// be incremented or decremented.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LexerVec<T: Token> {
    inner: Vec<T>,
    index: usize,
}

///User facing view of LexerVec. 
/// Auto impl Debug will expose internal tokens. 
impl<T: Token> Display for LexerVec<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "(LexerVec)")
    }
}

///Basic implementation of Lexer that wraps a vector 
/// Wraps the trait calls so users can include just this struct 
/// without the trait. 
#[allow(dead_code)]
impl<T: Token> LexerVec<T>
{
    pub fn new<Iter: IntoIterator<Item=I>, I: Into<T>>(tokens: Iter) -> LexerVec<T> {
        let tokens = tokens.into_iter().map(|i|i.into()).collect();
        LexerVec {
            inner: tokens,
            index: 0
        }
    }

    fn peek(&self) -> Option<T> {
        <Self as Lexer<T>>::peek(self)
    }

    fn next_token(&mut self) -> T {
        <Self as Lexer<T>>::next_token(self)
    }

    fn prev_token(&mut self) -> T {
        <Self as Lexer<T>>::prev_token(self)
    }
}

impl<T: Token> Lexer<T> for LexerVec<T>
{
    ///Basic index bounds checking. 
    /// index is usize, so can never be less 
    /// than 0. 
    fn peek(&self) -> Option<T> {
        if self.index < self.inner.len() {
            Some(self.inner[self.index].clone())
        } else {
            None
        }
    }

    ///Returns token pointed to by current index, then increments it
    /// (with bounds checking)
    fn next_token(&mut self) -> T {
        let t = self.inner[self.index].clone();
        if self.index + 1 < self.inner.len() {
            self.index += 1;
        }
        t
    }

    ///Returns token pointed to by the current index, then decrements it
    /// There is implied bounds checking in that usize can never be 
    /// less than 0. 
    fn prev_token(&mut self) -> T {
        let t = self.inner[self.index].clone();
        self.index -= 1;
        t
    }
}

impl<T: Token, I: Into<T>> FromIterator<I> for LexerVec<T> {
    fn from_iter<Iter: IntoIterator<Item=I>>(iter: Iter) -> Self {
        let mut v = Vec::new();
        for i in iter {
            v.push(i.into());
        }
        LexerVec::new(v)
    }
}

impl<T: Token> Extend<T> for LexerVec<T> {
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