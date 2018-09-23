// macros.rs - MIT License
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

//! # Utility Macros
//! 
//! Three macros are provided:
//!     add_null_assoc
//!     add_left_assoc
//!     add_left_right_assoc
//!     
//! These macros allow for the assignment of multiple tokens in one go, presented as
//! an alternative to the ParserSpec.add_multi_null_assoc, etc methods
//! 

//Utility macros to assign same left_binding_power/right_binding_power values and closures for tokens

#[macro_export]
macro_rules! add_null_assoc {
    ($spec:ident, $lbp:expr, ($($token:expr),* $(,)*) => $clsr:expr) => {
        $(
            $spec.add_null_assoc($token, $lbp, $clsr)?;
        )*
    };
}

#[macro_export]
macro_rules! add_left_assoc {
    ($spec:ident, $lbp:expr, ($($token:expr),* $(,)*) => $clsr:expr) => {
        $(
            $spec.add_left_assoc($token, $lbp, $clsr)?;
        )*
    };
}

#[macro_export]
macro_rules! add_left_right_assoc {
    ($spec:ident, $lbp:expr, $rbp:expr, ($($token:expr),* $(,)*) => $clsr:expr) => {
        $(
            $spec.add_left_right_assoc($token, $lbp, $rbp, $clsr)?;
        )*
    };
}