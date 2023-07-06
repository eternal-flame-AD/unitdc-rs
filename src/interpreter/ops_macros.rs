use crate::quantity::units::{BaseUnit, DerivedUnit};

use super::{Interpreter, InterpreterError, InterpreterResult};

impl<'a> Interpreter<'a> {
    pub fn op_macro_baseunit(&mut self, arg: &str) -> InterpreterResult<()> {
        let symbol = arg.trim();

        if self.unit_system.lookup_unit(symbol).is_some() {
            return Err(InterpreterError::AlreadyDefined(symbol.to_string()));
        }

        self.unit_system.push_base_unit(BaseUnit {
            symbol: symbol.to_string(),
        });

        Ok(())
    }
    pub fn op_macro_derivedunit(&mut self, arg: &str) -> InterpreterResult<()> {
        let symbol = arg.trim();

        let scale = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
        let offset = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        if self.unit_system.lookup_unit(symbol).is_some() {
            return Err(InterpreterError::AlreadyDefined(symbol.to_string()));
        }

        self.unit_system.push_derived_unit(DerivedUnit {
            symbol: symbol.to_string(),
            scale: scale.number,
            offset: offset.number,
            exponents: offset.unit,
        });

        Ok(())
    }
}
