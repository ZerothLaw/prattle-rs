// c_spec.rs - MIT License
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

use std::fmt::{Display, Error, Formatter};

extern crate prattle;

use prattle::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum CToken {
    //Value-carrying terminals
    Ident(String),
    IntConst(String), 
    ChrConst(String), 
    FltConst(String),
    String(String),
    //Structures
    Enum, Struct, Union,
    //type-qual
    Const, Volatile, 
    //type-specs
    Void,     Char,   Short, Int, 
    Long,     Double, Float, Signed,
    Unsigned,
    //storage-class-specs
    Auto,   Register, Static,
    Extern, Typedef,
    //Pre/postfix
    Inc, Dec,
    //Prefix
    Sizeof,
    //mul-expr
    Mul, Div, Mod,
    //add-expr
    Add, Sub,
    //sh-expr
    Shl, Shr,
    //rel-expr
    LT, GT, LTE, GTE, 
    //equality-expr
    Eqs, NEqs,
    //operands
    Xor,    // ^
    InclOr, // |
    LogAnd, // &&
    LogOr,  // ||
    //Ternary
    Question, 
    //Access ops
    Dot, Deref,
    //Unary Ops
    And, 
    Star, 
    BitNeg, //~
    Not,
    //Assign ops
    Equal, 
    MulEq, DivEq, ModEq, 
    AddEq, SubEq, ShlEq, 
    ShrEq, AndEq, XorEq, 
    OrEq,
    //Groupings
    LParens, RParens, 
    LBrace, RBrace,
    LCurly, RCurly,
    //Punctuation
    Semicolon,
    Comma,
    Colon,
    //Label
    Case, 
    Default, 
    //Select
    If, Else, Switch,
    //Iter
    While, Do, For,
    //Jump
    Goto, Continue, Break, Return,

    //syntactical tokens used during parsing
    DeclSpecs, //<declaration-specifier>*
    Ternary,
    Postfix,
}

impl Display for CToken {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

fn c_spec() -> Result<ParserSpec<CToken>, SpecificationError<CToken>> {
    let mut spec = ParserSpec::new();

    spec.add_null_associations(vec![
            CToken::Ident("".to_string()), 
            CToken::IntConst("".to_string()), 
            CToken::ChrConst("".to_string()), 
            CToken::FltConst("".to_string()),
            CToken::String("".to_string()),
        ], PrecedenceLevel::Root, |parser: &mut dyn Parser<CToken>, tk: CToken, lbp: PrecedenceLevel| {
        Ok(Node::Simple(tk.clone()))
    })?;
    spec.add_null_associations(vec![CToken::Enum], PrecedenceLevel::Root, |parser, _, _| {
        let id = match parser.parse_expr(PrecedenceLevel::Highest) {
            Ok(id) => Some(id), 
            Err(_) => None
        };
        //assuming an identifier
        //next parse {
        parser.consume(CToken::LCurly)?;
        //terminals of identifier, =, const-expr, ","
        //end on a comma, loop until we hit "}"
        let mut v = Vec::new();
        while let Ok(en_id) = parser.parse_expr(PrecedenceLevel::Highest) {
            match parser.consume(CToken::Equal) {
                Ok(_) => v.push(Node::Composite{token: CToken::Equal, children: vec![en_id, parser.parse_expr(PrecedenceLevel::Second)?]}), 
                Err(ParseError::ConsumeFailed{expected: _, found: CToken::Comma}) => v.push(en_id),
                Err(pe) => return Err(pe)
            };
            match parser.consume(CToken::Comma) {
                Ok(_) => continue, 
                Err(ParseError::ConsumeFailed{expected: _, found: CToken::RCurly}) => {
                    let _r = parser.consume(CToken::RCurly);
                    break
                }, 
                Err(pe) => return Err(pe)
            }
        }
        match id {
            Some(id) => Ok(Node::Composite{token: CToken::Enum, children: vec![id, Node::Composite{token: CToken::Comma, children: v}]}), 
            None => Ok(Node::Composite{token: CToken::Enum, children: vec![Node::Composite{token: CToken::Comma, children: v}]}), 
        }
    })?;

    spec.add_null_associations(vec![
        //storage class keywords
        CToken::Auto,   CToken::Register, CToken::Static, 
        CToken::Extern, CToken::Typedef,  
        //type keywords
        CToken::Void,   CToken::Char,     CToken::Short,
        CToken::Int,    CToken::Long,     CToken::Float, 
        CToken::Double, CToken::Signed,   CToken::Unsigned, 
        //type qualifier keywords
        CToken::Const,  CToken::Volatile,
    ], PrecedenceLevel::Root, |parser: &mut dyn Parser<CToken>, tk: CToken, lbp: PrecedenceLevel| {
        Ok(Node::Simple(tk.clone()))
    })?;

    spec.add_left_associations(vec![
        //storage class keywords
        CToken::Auto,   CToken::Register, CToken::Static, 
        CToken::Extern, CToken::Typedef,  
        //type keywords
        CToken::Void,   CToken::Char,     CToken::Short,
        CToken::Int,    CToken::Long,     CToken::Float, 
        CToken::Double, CToken::Signed,   CToken::Unsigned, 
        //type qualifier keywords
        CToken::Const,  CToken::Volatile,
    ], PrecedenceLevel::First, |parser, tk, lbp, node| {
        let decl_specs_keywords = vec![
            //storage class keywords
            CToken::Auto,   CToken::Register, CToken::Static, 
            CToken::Extern, CToken::Typedef,  
            //type keywords
            CToken::Void,   CToken::Char,     CToken::Short,
            CToken::Int,    CToken::Long,     CToken::Float, 
            CToken::Double, CToken::Signed,   CToken::Unsigned, 
            //type qualifier keywords
            CToken::Const,  CToken::Volatile,
        ];
        match node {
            Node::Simple(ref ctk) if decl_specs_keywords.contains(ctk) => Ok(Node::Composite{token: CToken::DeclSpecs, children: vec![node.clone(), Node::Simple(tk.clone())]}), 
            Node::Simple(ref ctk) => Err(ParseError::MalformedSyntax{node: node.clone(), token: ctk.clone()}), 
            Node::Composite {
                token: CToken::DeclSpecs, 
                 mut children }  => Ok(Node::Composite{token: CToken::DeclSpecs, children: { children.push(Node::Simple(tk.clone())); children }}), 
            _ => Err(ParseError::MalformedSyntax{node: node, token: tk.clone()})
        }
    })?;

    spec.add_null_associations(
        vec![CToken::Struct, CToken::Union], 
        PrecedenceLevel::First, 
        |parser, tk, _| {
            let id = match parser.parse_expr(PrecedenceLevel::Highest) {
                Ok(id) => Some(id), 
                Err(pe) => None,
            };
            //assuming identifier and body
            let mut v = Vec::new();
            parser.consume(CToken::LCurly)?;
            while let Ok(decl) = parser.parse_expr(PrecedenceLevel::Second) {
                //ends at semicolon each time
                match parser.consume(CToken::Semicolon) {
                    Ok(_) => {v.push(decl); continue},
                    Err(ParseError::ConsumeFailed{expected: _, found: CToken::RCurly}) => {v.push(decl); break},
                    Err(pe) => return Err(pe),
                }
            }
            match id {
                Some(id) => Ok(Node::Composite{token: tk.clone(), children: vec![id, Node::Composite{token: CToken::Comma, children: v}]}), 
                None => Ok(Node::Composite{token: tk.clone(), children: vec![Node::Composite{token: CToken::Comma, children: v}]})
            }
        }
    )?;

    spec.add_left_assoc(CToken::Ident("".to_string()), PrecedenceLevel::Third, |_, tk, _, node| {
        Ok(Node::Composite{token: tk.clone(), children: vec![node]})
    })?;

    //ternary
    spec.add_left_assoc(CToken::Question, PrecedenceLevel::Second, |parser, tk, lbp, tern_expr| {
        let post_q_expr = parser.parse_expr(lbp)?;
        if let Err(ParseError::ConsumeFailed{expected: _, found: wrong_tk}) = parser.consume(CToken::Colon) {
            return Err(ParseError::MalformedSyntax{node: post_q_expr, token: wrong_tk});
        }
        let cond_expr = parser.parse_expr(lbp)?;
        Ok(Node::Composite{token: CToken::Ternary, children: vec![tern_expr, post_q_expr, cond_expr]})
    })?;

    spec.add_left_assoc(CToken::LogOr, PrecedenceLevel::Third, |parser, _, _, lhs| {
        let rhs = parser.parse_expr(PrecedenceLevel::Fourth)?;
        Ok(Node::Composite{token: CToken::LogOr, children: vec![lhs, rhs]})
    })?;
    spec.add_left_assoc(CToken::LogAnd, PrecedenceLevel::Fourth, |parser, _, _, lhs| {
        let rhs = parser.parse_expr(PrecedenceLevel::Fifth)?;
        Ok(Node::Composite{token: CToken::LogAnd, children: vec![lhs, rhs]})
    })?;
    spec.add_left_assoc(CToken::InclOr, PrecedenceLevel::Fifth, |parser, _, _, lhs| {
        let rhs = parser.parse_expr(PrecedenceLevel::Fourth)?;
        Ok(Node::Composite{token: CToken::InclOr, children: vec![lhs, rhs]})
    })?;
    spec.add_left_assoc(CToken::Xor, PrecedenceLevel::Fifth, |parser, tk, _, lhs| {
        let rhs = parser.parse_expr(PrecedenceLevel::Sixth)?;
        Ok(Node::Composite{token: tk.clone(), children: vec![lhs, rhs]})
    })?;
    spec.add_left_assoc(CToken::And, PrecedenceLevel::Sixth, |parser, tk, _, lhs| {
        let rhs = parser.parse_expr(PrecedenceLevel::Seventh)?;
        Ok(Node::Composite{token: tk.clone(), children: vec![lhs, rhs]})
    })?;
    spec.add_left_associations(vec![CToken::Eqs, CToken::NEqs], PrecedenceLevel::Seventh, |parser, tk, _, lhs| {
        let rhs = parser.parse_expr(PrecedenceLevel::Eighth)?;
        Ok(Node::Composite{token: tk.clone(), children: vec![lhs, rhs]})
    })?;
    spec.add_left_associations(vec![CToken::LT, CToken::GT, CToken::LTE, CToken::GTE], PrecedenceLevel::Ninth, |parser, tk, _, lhs| {
        let rhs = parser.parse_expr(PrecedenceLevel::Tenth)?;
        Ok(Node::Composite{token: tk.clone(), children: vec![lhs, rhs]})
    })?;
    spec.add_left_associations(vec![CToken::Shl, CToken::Shr], PrecedenceLevel::Tenth, |parser, tk, _, lhs| {
        let rhs = parser.parse_expr(PrecedenceLevel::Eleven)?;
        Ok(Node::Composite{token: tk.clone(), children: vec![lhs, rhs]})
    })?;
    spec.add_left_associations(vec![CToken::Add, CToken::Sub], PrecedenceLevel::Eleven, |parser, tk, _, lhs| {
        let rhs = parser.parse_expr(PrecedenceLevel::Twelve)?;
        Ok(Node::Composite{token: tk.clone(), children: vec![lhs, rhs]})
    })?;
    spec.add_left_associations(vec![CToken::Mul, CToken::Div, CToken::Mod], PrecedenceLevel::Twelve, |parser, tk, _, lhs| {
        let rhs = parser.parse_expr(PrecedenceLevel::Thirteen)?;
        Ok(Node::Composite{token: tk.clone(), children: vec![lhs, rhs]})
    })?;
    spec.add_left_right_associations(
        vec![CToken::LBrace, CToken::LParens], 
        PrecedenceLevel::Sixth, 
        PrecedenceLevel::First, 
        |parser, token, lbp, node| {
            let exprs = parser.parse_expr(lbp)?;
            let end_t = match token {
                CToken::LBrace => CToken::RBrace, 
                CToken::LParens => CToken::RParens, 
                _ => unreachable!()
            };
            parser.consume(end_t.clone())?;
            Ok(Node::Composite{token: CToken::Postfix, children: vec![
                node, Node::Simple(token.clone()), exprs, Node::Simple(end_t.clone())
            ]})
        }
    )?;
    spec.add_left_associations(vec![CToken::Dot, CToken::Deref], PrecedenceLevel::Sixth, 
        |parser, tk, lbp, node| {
            if let Err(ParseError::ConsumeFailed{expected: _, found: fnd}) = parser.consume(CToken::Ident("".to_string())) {
                parser.consume(fnd.clone())?;
                Ok(Node::Composite{token: tk.clone(), children: vec![node, Node::Simple(fnd)]})
            } else {
                Err(ParseError::MalformedSyntax{node: node, token: tk.clone()})
            }
        }
    )?;
    spec.add_left_associations(vec![CToken::Inc, CToken::Dec], PrecedenceLevel::Sixth, 
        |parser, tk, lbp, node| {
            Ok(Node::Composite{token: CToken::Postfix, children: vec![node, Node::Simple(tk.clone())]})
        }
    )?;
    spec.add_null_associations(
        vec![
            CToken::Inc, CToken::Dec, CToken::Sizeof, 
            CToken::And, CToken::Star, CToken::Add, 
            CToken::Sub, CToken::BitNeg, CToken::Not
        ], 
        PrecedenceLevel::First, 
        |parser, tk, lbp|{
            Ok(Node::Composite{token: tk, children: vec![parser.parse_expr(PrecedenceLevel::Second)?]})
        }
    )?;
    spec.add_null_assoc(CToken::LParens, PrecedenceLevel::First, |parser, _, _| {
        parser.parse_expr(PrecedenceLevel::Second)
    })?;

    spec.add_left_associations(
        vec![
            CToken::Equal, CToken::MulEq, CToken::DivEq, 
            CToken::ModEq, CToken::AddEq, CToken::SubEq, 
            CToken::ShlEq, CToken::ShrEq, CToken::AndEq, 
            CToken::XorEq, CToken::OrEq], 
        PrecedenceLevel::Seventh, 
        |parser, tk, lbp, node| {
            Ok(Node::Composite{token: tk.clone(), children: vec![node, parser.parse_expr(lbp)?]})
        }
    )?;

    Ok(spec)
}

fn c_spec_2() -> Result<ParserSpec<CToken>, SpecificationError<CToken>> {
    let mut spec = ParserSpec::new();
    //label (identifier : statement)
    //label ("case" expr : statement)
    //label ("default" : statement)
    spec.add_null_assoc(CToken::Default, PrecedenceLevel::Root, 
        |parser, tk, lbp| {
            if let Ok(()) = parser.consume(CToken::Colon) {
                Ok(Node::Composite{token: tk, children: vec![parser.parse_expr(lbp)?]})
            } else {
                Err(ParseError::MalformedSyntax{node: Node::Simple(tk), token: CToken::Semicolon})
            }
        }
    )?;

    //expr stmt (expr? ;)

    //select ("if" "(" expr ")" statement)
    //select ("if" "(" expr ")" "else" statement)
    //select ("switch" "(" expr ")" statement)

    //iter ("while" "(" expr ")" statement)
    //iter ("do" statement "while" "(" expr ")" ;)
    //iter ("for" "(" expr? ; expr? ; expr? ")" statement)

    //Jump ("goto" ident ;)
    spec.add_null_assoc(CToken::Goto, PrecedenceLevel::Root, 
        |parser, tk, _| {
            if let Err(ParseError::ConsumeFailed{expected: _, found: fnd}) = parser.consume(CToken::Ident("".to_string())) {
                parser.consume(fnd.clone())?;
                if let Ok(()) = parser.consume(CToken::Semicolon) {
                    Ok(Node::Composite{token: tk, children: vec![Node::Simple(fnd)]})
                } else {
                    Err(ParseError::MalformedSyntax{node: Node::Simple(tk), token: CToken::Semicolon})
                }
            } else {
                Err(ParseError::MalformedSyntax{node: Node::Simple(tk), token: CToken::Ident("missing ident".to_string())})
            }
        }
    )?;
    //Jump (("continue" | "break") ;)
    spec.add_null_associations(vec![CToken::Continue, CToken::Break], PrecedenceLevel::Root, 
        |parser, tk, _|{
            if let Ok(()) = parser.consume(CToken::Semicolon) {
                Ok(Node::Composite{token: tk, children: Vec::new()})
            } else {
                Err(ParseError::MalformedSyntax{node: Node::Simple(tk), token: CToken::Semicolon})
            }
        }
    )?;
    //Jump ("return" expression? ;)
    spec.add_null_assoc(CToken::Return, PrecedenceLevel::Root, |parser, tk, _| {
            match parser.consume(CToken::Semicolon) {
                Ok(_) => Ok(Node::Composite{token: CToken::Return, children: Vec::new()}), 
                Err(ParseError::ConsumeFailed{expected: _, found: _}) => unimplemented!(), 
                _ => unreachable!()
            }
        }
    )?;
    Ok(spec)
}

fn main() {
    let spec = c_spec_2().unwrap();

    let lexer = LexerVec::new(vec![
        CToken::Goto, 
        CToken::Ident("abc".to_string()), 
        CToken::Semicolon,
        CToken::Continue,
        CToken::Semicolon,
        CToken::Break,
        CToken::Semicolon,
        CToken::Return,
        CToken::Semicolon,
        CToken::Default, 
        CToken::Colon, 
        CToken::Return,
        CToken::Semicolon,
    ]);

    let mut parser = GeneralParser::new(spec, lexer);
    println!("{:?}", parser.parse());
    println!("{:?}", parser.parse());
    println!("{:?}", parser.parse());
    println!("{:?}", parser.parse());
    println!("{:?}", parser.parse());
}