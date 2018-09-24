// precedence.rs - MIT License
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

//! # Precendence Levels
//! This is the most powerful, subtle, and easy to do incorrectly part of the 
//! Pratt parser. Each token has a specific null/left/right binding power, 
//! and to be processed, they need to be higher than the current context
//! 
//! This allows the algorithm/parser to naturally implement precedence climbing
//! which is what basically says X operator is processed before other operators. 
//! 
//! ## Example
//! > a + b * c
//! 
//! When mul is defined with a higher precedence than add, that results in the 
//! following grouping:
//! > a + (b * c)
//! where b*c is done before the addition. Many languages have either a defined
//! or implicit precedence ordering of operators. 
//! 
//! For example, see [C++ Operator Precedence table](https://en.cppreference.com/w/cpp/language/operator_precedence)
//! 

use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PrecedenceLevel {
    Root    = 0, 
    First   = 5, 
    Second  = 10, 
    Third   = 15, 
    Fourth  = 20, 
    Fifth   = 25, 
    Sixth   = 30, 
    Seventh = 35, 
    Eighth  = 40,
}

impl Display for PrecedenceLevel {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "(Precedence: {})", *self as u32)
    }
}