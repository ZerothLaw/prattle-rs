// trait.rs - MIT License
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

//! # Token trait and blanket impl
//! ## Rationale
//! This makes it easier to define required trait bounds, 
//! rather than using the long form. 
//! 
//! The reason for each trait is as follows:
//!  * Clone - This is a useful utility trait to implement. It makes it easier to 
//!            build an Abstract Syntax Tree without dealing with references and 
//!            lifetimes. 
//!  * Debug - Necessary impl for failure::Fail trait
//!  * Display - Necessary impl for failure::Fail trait
//!  * Hash - Necessary impl for use with HashMap in the ParserSpec 
//!  * Ord  - Ord is defined as : Eq + PartialOrd, and Eq is : Partial Eq. We need
//!           all three traits (PartialOrd, PartialEq, and Hash) for working with 
//!           ParserSpec HashMap members

use std::fmt::{Debug, Display};
use std::hash::Hash;

pub trait Token:  Clone + Debug + Display + Hash + Ord  {}

impl<T> Token for T where T:  Clone + Debug + Display + Hash + Ord {}
