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

use super::{Interpreter, InterpreterError, InterpreterResult};

impl<'a> Interpreter<'a> {
    /// Pops a quantity from the stack and stores it in a variable.
    pub fn op_store(&mut self, arg: &str) -> InterpreterResult<()> {
        let symbol = arg.trim();

        let q = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        self.variables.insert(symbol.to_string(), q);

        Ok(())
    }
    /// Pushes a quantity from a variable onto the stack.
    pub fn op_recall(&mut self, arg: &str) -> InterpreterResult<()> {
        let symbol = arg.trim();

        let q = self
            .variables
            .get(symbol)
            .ok_or(InterpreterError::UndefinedVariable(symbol.to_string()))?
            .clone();

        self.stack.push(q);

        Ok(())
    }
}
