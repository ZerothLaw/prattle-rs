// lib.rs - MIT License
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

//! # Introduction
//! 
//! This crate implements a configurable and general-purpose Pratt parser.
//! 
//! A Pratt parser is also known as a Top-Down Operator Precedence parser, 
//! aka TDOP parser. The general algorithm was discovered and outlined by 
//! Vaughn Pratt in 1973 in his paper[1]. 
//! 
//! It differs from recursive-descent classes of parsers by associating
//! parsing rules to tokens rather than grammar rules. 
//! 
//! This is especially valuable when parsing expression grammars with 
//! operator precedence without significant descent and type costs. 
//! 
//! Consider the following expression: 
//!
//! > a + b * c
//! 
//! So which variables are to be associated with which operators? 
//! A recursive-descent parser would need to implement two layers of 
//! grammar rules which are recursive with each other. This leads to 
//! potential infinite loops if your parser follows the wrong rule and
//! doesn't backtrack effectively. 
//! 
//! Whereas with a Pratt Parser, you need just three rules and three 
//! binding powers:
//! null denotation of Variable => Node::Simple(token);
//! left denotation of Add => Node::Composite(token: token, children: vec![node, 
//!     parser.parse_expr(5)]);
//! left denotation of Mul => Node::Composite(token: token, children: vec![node, 
//!     parser.parse_expr(10)]);
//! lbp of Variable = 0;
//! lbp of Add = 5;
//! lbp of Mul = 10;
//! 
//! And now it will correctly associate 'b' and 'c' tokens with the mul operator, and 
//! then the result of that with the 'a' token and the add operator. 
//! 
//! It also executes with far fewer parse calls, creating a minimal stack depth 
//! during execution. 
//! 
//! ## Example
//! 
//! ```rust
//! use std::fmt::{Display, Formatter, Error};
//! 
//! extern crate prattle;
//! use prattle::prelude::*;
//! 
//! #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
//! enum CToken {
//!     Var(String), 
//!     Add, 
//!     Mul
//! }
//! 
//! impl Display for CToken {
//!     fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
//!         write!(f, "{:?}", self)
//!     }
//! }
//! 
//! fn main() {
//!     let mut spec = ParserSpec::new();
//! 
//!     spec.add_null_assoc(
//!         CToken::Var("".to_string()),
//!         PrecedenceLevel::Root, 
//!         |parser: &mut dyn Parser<CToken>, token, _| {
//!             Ok(Node::Simple(token.clone()))
//!         }
//!     );
//!     spec.add_left_assoc(
//!         CToken::Add, 
//!         PrecedenceLevel::First, 
//!         |parser, token, lbp, node| {
//!             Ok(Node::Composite{token: token.clone(), children: vec![
//!                 node, 
//!                 parser.parse_expr(lbp)?]})
//!     });
//!     spec.add_left_assoc(
//!         CToken::Mul, 
//!         PrecedenceLevel::Second, 
//!         |parser, token, lbp, node| {
//!             Ok(Node::Composite{token: token.clone(), children: vec![
//!                 node, 
//!                 parser.parse_expr(lbp)?]})
//!     });
//! 
//!     let lexer = LexerVec::new(vec![
//!         CToken::Var("a".to_string()),
//!         CToken::Add, 
//!         CToken::Var("b".to_string()), 
//!         CToken::Mul, 
//!         CToken::Var("c".to_string())
//!     ]);
//! 
//!     let mut parser = GeneralParser::new(spec, lexer);
//! 
//!     let res = parser.parse();
//!     println!("{:?}", res);
//! }
//! ```
//! 
//! ## Capabilities
//! 
//! This crate enables a very fast and simple parser, with simple rules, that is 
//! easy to understand and maintain. Most of the work is in implementing the
//! required traits on your Token type. 
//! 
//! ## More complex examples
//! 
//! Run: 
//! > cargo run --example token_spec
//! 
//! examples/token_spec.rs shows an example of how to implement the traits for 
//! the token type so it can be used to lookup the parse rules (uses HashMap).
//! 
//! ## Citations
//! > [1] Vaughan R. Pratt. 1973. Top down operator precedence. In Proceedings
//! > of the 1st annual ACM SIGACT-SIGPLAN symposium on Principles of 
//! > programming languages (POPL '73). ACM, New York, NY, USA, 41-51. 
//! > DOI=http://dx.doi.org/10.1145/512927.512931
//! 

#[macro_use] extern crate failure;

#[macro_use] pub mod macros;

pub mod errors;
pub mod lexer;
pub mod node;
pub mod parser;
pub mod precedence;
pub mod spec;
pub mod token;

/// Handy prelude mod containing everything you need to get started. 
pub mod prelude {
    pub use errors::ParseError;
    pub use lexer::{Lexer, LexerVec};
    pub use node::Node;
    pub use parser::{Parser, GeneralParser};
    pub use precedence::PrecedenceLevel;
    pub use spec::{ParserSpec, SpecificationError};
    pub use token::Token;
}

//Little container mod for type aliases that are convenient and short
pub mod types {
    use super::prelude::*;
    pub type NullDenotation<T> = fn(&mut dyn Parser<T>, T, PrecedenceLevel) -> Result<Node<T>, ParseError<T>>;
    pub type LeftDenotation<T> = fn(&mut dyn Parser<T>, T, PrecedenceLevel, Node<T>) -> Result<Node<T>, ParseError<T>>;

    pub type NullInfo<T> = (PrecedenceLevel, NullDenotation<T>);
    pub type LeftInfo<T> = (PrecedenceLevel, PrecedenceLevel, LeftDenotation<T>);
}