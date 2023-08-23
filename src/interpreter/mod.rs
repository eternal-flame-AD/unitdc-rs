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
