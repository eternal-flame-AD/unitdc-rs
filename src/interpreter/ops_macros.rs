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

use crate::quantity::units::{BaseUnit, DerivedUnit};

use super::{Interpreter, InterpreterError, InterpreterResult};

impl<'a> Interpreter<'a> {
    /// Defines a new base unit.
    ///
    /// For example to define a unit "usd" (US Dollar), you would do:
    /// `@base(usd)`
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
    /// Defines a new derived unit.
    ///
    /// This is done by popping a "scale" and then an "offset" from the stack.
    /// For example, to define a new unit "mpg" (miles per gallon), you would do:
    /// `0 (mi) 1 (gal) / 1 (mi) 1 (gal) / @derived(mpg)`
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
