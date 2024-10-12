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

use std::{
    collections::HashMap,
    io::{BufReader, Read},
};

use crate::{
    quantity::{
        units::{UnitCombo, UnitSystem},
        Quantity, QuantityError,
    },
    tokenizer::{token::Token, ReaderCursor, Tokenizer, TokenizerError},
};

use itertools::Itertools;
use num_bigint::BigInt;
use num_traits::Zero;
use thiserror::Error;

/// All other operations.
pub mod ops;
/// Macro operations.
pub mod ops_macros;
/// Variable I/O operations.
pub mod ops_variables;

pub struct Interpreter<'a> {
    variables: HashMap<String, Quantity>,
    unit_system: UnitSystem,
    stack: Vec<Quantity>,
    output: Box<dyn Fn(Output) + 'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    Quantity(Quantity),
    QuantityList(Vec<Quantity>),
    Message(String),
}

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Tokenizer error: {1} at {0}")]
    TokenizerError(ReaderCursor, TokenizerError),
    #[error("Quantity error: {0}")]
    QuantityError(QuantityError),
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Undefined unit: {0}")]
    UndefinedUnit(String),
    #[error("Undefined macro: {0}")]
    UndefinedMacro(String),
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
    #[error("Incompatible units: {0}")]
    IncompatibleUnits(UnitCombo),
    #[error("No solution: {0}")]
    NoSolution(String),
    #[error("Already defined: {0}")]
    AlreadyDefined(String),
}

pub type InterpreterResult<T> = Result<T, InterpreterError>;

impl<'a> Interpreter<'a> {
    pub fn new(output: Box<dyn Fn(Output) + 'a>) -> Self {
        Self {
            variables: HashMap::new(),
            unit_system: UnitSystem::new(),
            stack: Vec::new(),
            output,
        }
    }
    /// Warns about quantities with offset derived units that are used in multiple quantities.
    pub fn warn_confusing_unit_conversions(&self, qs: &[&Quantity]) {
        let offset_base_units = qs
            .iter()
            .flat_map(|q| q.use_derived_unit.iter())
            .filter(|d| *d.offset.numer() != BigInt::zero())
            .flat_map(|d| {
                d.exponents
                    .0
                    .iter()
                    .unique_by(|d| &d.unit)
                    .map(|e| (d.clone(), e.unit.clone()))
            });

        for (u, c) in offset_base_units.into_group_map_by(|(_, u)| u.clone()) {
            if c.len() > 1 {
                (self.output)(Output::Message(format!(
                    "Warning: {} it is used in multiple quantities with an offset. This may lead to unexpected results. Affected derived units: [{}]",
                    u,
                    c.iter().map(|(d, _)| d.symbol.clone()).join(", ")
                )));
            }
        }
    }
    /// Reads all tokens from the tokenizer and processes them.
    pub fn process_tokens<R: Read>(
        &mut self,
        tokenizer: &mut Tokenizer<R>,
    ) -> InterpreterResult<()> {
        while let Some(token) = tokenizer
            .parse_next_token()
            .map_err(|e| InterpreterError::TokenizerError(tokenizer.get_cursor(), e))?
        {
            match token {
                Token::Number(n) => self.op_number(n)?,
                Token::Unit(u) => self.op_unit(&u)?,
                Token::Add => self.op_add()?,
                Token::Sub => self.op_sub()?,
                Token::Mul => self.op_mul()?,
                Token::Div => self.op_div()?,
                Token::Operator('p') => self.op_p()?,
                Token::Operator('n') => self.op_n()?,
                Token::Operator('f') => self.op_f()?,
                Token::Operator('c') => self.op_c()?,
                Token::Operator('d') => self.op_d()?,
                Token::Operator('r') => self.op_r()?,
                Token::Operator('s') => self.op_s()?,
                Token::Operator('U') => self.op_upper_u()?,
                Token::VarRecall(name) => self.op_recall(&name)?,
                Token::VarStore(name) => self.op_store(&name)?,
                Token::MacroInvoke((name, args)) => match name.as_str() {
                    "base" => self.op_macro_baseunit(&args)?,
                    "derived" => self.op_macro_derivedunit(&args)?,
                    _ => return Err(InterpreterError::UndefinedMacro(name)),
                },
                Token::Comment(_) => {}
                _ => eprintln!("Unhandled token: {:?}", token),
            }
        }

        Ok(())
    }
    pub fn run_str(&mut self, input: &str) -> InterpreterResult<()> {
        let mut tokenizer = Tokenizer::new(BufReader::new(input.as_bytes()));
        self.process_tokens(&mut tokenizer)?;

        Ok(())
    }
}
