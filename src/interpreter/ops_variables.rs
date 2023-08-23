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
