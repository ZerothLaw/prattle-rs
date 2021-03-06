// errors.rs - MIT License
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

//! # Errors Module 
//! 
//! Contains the utilitarian ParseError enum that wraps useful information 
//! of what exactly went wrong during parsing. 
//! 
//! Generally, your rules shouldn't manually return these errors - the parser will 
//! return these errors where they make the best sense.

use std::marker::{Send, Sync};

use node::Node;
use token::Token;

/// # ParseError
/// ## Explanation
/// This enum implements failure::Fail, which in turn requires 
/// Debug + Display + Send + Sync + 'static to ensure the errors
/// can be sent to other threads and referenced from other threads. 
/// 
/// This in turn forces types that deal with ParseError to be 
/// Send + Sync + 'static as well as Token implementations. 
/// 
/// Deriving Fail implies implementation of std::error::Error trait.
/// 
#[derive(Clone, Debug, Eq, Fail, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParseError<T: Token + Send + Sync + 'static> {
    /// # ParseError::MalformedSyntax
    /// Represents parser context when a syntax rule fails.
    /// Not returned by the general parser implementation. 
    /// Usage: during a syntax rule, if this error is to be 
    /// returned, use *node* for the current node passed to
    /// the syntax rule, and *token* for the token that lead to
    /// the error to be returned.
    #[fail(display = "incorrect syntax, failed on node: {} with token: {}", node, token)]
    MalformedSyntax{ node: Node<T>, token: T }, 
    /// Returned by the parser when a rule is not found for a specific token.
    /// Generally only should be seen during development of a language spec.
    #[fail(display = "missing a {} syntax rule for: {}", ty, token)]
    MissingRule {token: T, ty: String}, 
    /// Expected more input than was available. Returned by the parser.
    #[fail(display = "token iteration ended before parsing context finished")]
    Incomplete, 
    /// <P as Parser<T>>::consume(end_token: T) was called, and the required
    /// token was not found as the next token(returned by peek/next_token).
    #[fail(display = "parser.consume(end_token: {}) didn't find expected token, instead found: {}.", expected, found)]
    ConsumeFailed {expected: T, found: T}
}

#[cfg(test)]
mod test {
    use super::*;
    //Catch Send/Sync changes
    #[test]
    fn test_parseerror_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ParseError<String>>();
    }

    #[test]
    fn test_parseerror_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ParseError<String>>();
    }
}