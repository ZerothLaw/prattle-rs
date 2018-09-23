// node.rs - MIT License
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

//! # Node enum
//! 
//! This is a general purpose enum construct for representing parse trees. 
//! 
//! In the most general case, you have simple nodes (representing concrete constructs 
//! like identifiers or numbers), and composite nodes, which has a single root token
//! (for example an operator), and zero-to-many child nodes. 
//! 

use std::fmt::{Debug, Display, Error, Formatter};
use std::hash::Hash;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Node<T:  Clone + Debug + Display + Hash + Ord > {
    Simple(T), 
    Composite {
        token: T,
        children: Vec<Node<T>>
    }
}

impl<T:  Clone + Debug + Display + Hash + Ord > Display for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f,
            "{}", 
            match self {
                Node::Simple(ref t) => format!("Simple({})", t), 
                Node::Composite{
                    token: ref t, 
                    children: ref childs
                } => format!("Composite(token: {}, children: {:?})", t, childs )
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    //Catch Send/Sync changes
    #[test]
    fn test_node_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Node<String>>();
    }

    #[test]
    fn test_node_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Node<String>>();
    }
}