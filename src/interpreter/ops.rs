use num_rational::BigRational;

use crate::quantity::{
    units::{Unit, UnitCombo},
    Quantity,
};

use super::{Interpreter, InterpreterError, InterpreterResult, Output};

impl<'a> Interpreter<'a> {
    pub fn op_number(&mut self, number: BigRational) -> InterpreterResult<()> {
        self.stack.push(Quantity::new(number, UnitCombo::new()));
        Ok(())
    }
    pub fn op_unit(&mut self, unit: &str) -> InterpreterResult<()> {
        let mut q = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        // Unitless, remove unit
        if unit == "1" {
            q.unit = UnitCombo::new();
            self.stack.push(q);
            return Ok(());
        }

        let unit = self
            .unit_system
            .lookup_unit(unit)
            .ok_or(InterpreterError::UndefinedUnit(unit.to_string()))?;

        match unit {
            Unit::Base(base_unit) => {
                let mut new_unit = UnitCombo::new();
                new_unit.push_base_unit(base_unit.clone(), 1);
                if q.unit.is_unitless() {
                    q.unit = new_unit;
                } else if q.unit == new_unit {
                    q.use_derived_unit.clear();
                } else {
                    return Err(InterpreterError::IncompatibleUnits(q.unit));
                }
            }
            Unit::Derived(derived_unit) => {
                let mut new_unit = UnitCombo::new();
                new_unit.push_derived_unit(derived_unit.clone());
                if q.unit == new_unit {
                    q.use_derived_unit
                        .retain(|u| u.exponents != derived_unit.exponents);
                    q.use_derived_unit.push(derived_unit.clone());
                } else if q.unit.is_unitless() {
                    q.number *= derived_unit.scale.clone();
                    q.number += derived_unit.offset.clone();
                    q.unit = new_unit;
                    q.use_derived_unit
                        .retain(|u| u.exponents != derived_unit.exponents);
                    q.use_derived_unit.push(derived_unit.clone());
                } else {
                    return Err(InterpreterError::IncompatibleUnits(q.unit));
                }
            }
        }

        self.stack.push(q);

        Ok(())
    }
    pub fn op_add(&mut self) -> InterpreterResult<()> {
        let rhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
        let lhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        self.stack
            .push((lhs + rhs).map_err(InterpreterError::QuantityError)?);

        Ok(())
    }
    pub fn op_sub(&mut self) -> InterpreterResult<()> {
        let rhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
        let lhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        self.stack
            .push((lhs - rhs).map_err(InterpreterError::QuantityError)?);

        Ok(())
    }
    pub fn op_mul(&mut self) -> InterpreterResult<()> {
        let rhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
        let lhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        self.stack.push(lhs * rhs);

        Ok(())
    }
    pub fn op_div(&mut self) -> InterpreterResult<()> {
        let rhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
        let lhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        self.stack.push(lhs / rhs);

        Ok(())
    }
    pub fn op_p(&mut self) -> InterpreterResult<()> {
        let q = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        (self.output)(Output::Quantity(q.clone()));

        self.stack.push(q);

        Ok(())
    }
    pub fn op_n(&mut self) -> InterpreterResult<()> {
        let q = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        (self.output)(Output::Quantity(q));

        Ok(())
    }
    pub fn op_f(&mut self) -> InterpreterResult<()> {
        (self.output)(Output::QuantityList(self.stack.clone()));

        Ok(())
    }
    pub fn op_d(&mut self) -> InterpreterResult<()> {
        let q = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        self.stack.push(q.clone());
        self.stack.push(q);

        Ok(())
    }
    pub fn op_c(&mut self) -> InterpreterResult<()> {
        self.stack.clear();

        Ok(())
    }
    pub fn op_r(&mut self) -> InterpreterResult<()> {
        let a = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
        let b = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        self.stack.push(a);
        self.stack.push(b);

        Ok(())
    }
}
