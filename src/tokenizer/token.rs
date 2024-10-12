// Copyright 2024 eternal-flame-AD <yume@yumechi.jp>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use num_rational::BigRational;
use num_traits::ToPrimitive;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(BigRational),
    Unit(String),
    Add,
    Sub,
    Mul,
    Div,
    VarStore(String),
    VarRecall(String),
    Operator(char),
    MacroInvoke((String, String)),
    Comment(String),
}

impl Token {
    pub fn roughly_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Number(a), Token::Number(b)) => a.to_f64().unwrap() == b.to_f64().unwrap(),
            (Token::Unit(a), Token::Unit(b)) => a == b,
            (Token::Add, Token::Add) => true,
            (Token::Sub, Token::Sub) => true,
            (Token::Mul, Token::Mul) => true,
            (Token::Div, Token::Div) => true,
            (Token::Operator(a), Token::Operator(b)) => a == b,
            _ => false,
        }
    }
}
