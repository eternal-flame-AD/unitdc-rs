use num_rational::BigRational;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::{
    linear_system::{transpose, LinearSystem},
    quantity::{
        units::{Unit, UnitCombo},
        Quantity,
    },
};

use super::{Interpreter, InterpreterError, InterpreterResult, Output};

impl<'a> Interpreter<'a> {
    /// A literal number input, pushes a unit-less quantity to the stack.
    pub fn op_number(&mut self, number: BigRational) -> InterpreterResult<()> {
        self.stack.push(Quantity::new(number, UnitCombo::new()));
        Ok(())
    }
    /// A literal unit input.
    ///
    /// - If the unit is unit-less (1), the top of the stack will be converted to a unit-less quantity.
    /// - If the top of the stack is a unit-less quantity, it will be converted to the given unit.
    /// - If the top of the stack is a quantity with equivalent units, it will be converted to the given unit.
    /// - Otherwise, an error will be returned.
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
    /// Adds the top two quantities on the stack.
    pub fn op_add(&mut self) -> InterpreterResult<()> {
        let rhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
        let lhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        self.warn_confusing_unit_conversions(&[&lhs, &rhs]);

        self.stack
            .push((lhs + rhs).map_err(InterpreterError::QuantityError)?);

        Ok(())
    }
    /// Subtracts the top two quantities on the stack.
    pub fn op_sub(&mut self) -> InterpreterResult<()> {
        let rhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
        let lhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        self.warn_confusing_unit_conversions(&[&lhs, &rhs]);

        self.stack
            .push((lhs - rhs).map_err(InterpreterError::QuantityError)?);

        Ok(())
    }
    /// Multiplies the top two quantities on the stack.
    pub fn op_mul(&mut self) -> InterpreterResult<()> {
        let rhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
        let lhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        self.warn_confusing_unit_conversions(&[&lhs, &rhs]);

        self.stack.push(lhs * rhs);

        Ok(())
    }
    /// Divides the top two quantities on the stack.
    pub fn op_div(&mut self) -> InterpreterResult<()> {
        let rhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
        let lhs = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        self.warn_confusing_unit_conversions(&[&lhs, &rhs]);

        self.stack.push(lhs / rhs);

        Ok(())
    }
    /// Prints the top of the stack without altering it.
    pub fn op_p(&mut self) -> InterpreterResult<()> {
        let q = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        (self.output)(Output::Quantity(q.clone()));

        self.stack.push(q);

        Ok(())
    }
    /// Prints the top of the stack and pops it.
    pub fn op_n(&mut self) -> InterpreterResult<()> {
        let q = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        (self.output)(Output::Quantity(q));

        Ok(())
    }
    /// Prints the entire stack.
    pub fn op_f(&mut self) -> InterpreterResult<()> {
        (self.output)(Output::QuantityList(self.stack.clone()));

        Ok(())
    }
    /// Duplicates the top of the stack.
    pub fn op_d(&mut self) -> InterpreterResult<()> {
        let q = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        self.stack.push(q.clone());
        self.stack.push(q);

        Ok(())
    }
    /// Clears the stack.
    pub fn op_c(&mut self) -> InterpreterResult<()> {
        self.stack.clear();

        Ok(())
    }
    /// Swaps the top two elements of the stack.
    pub fn op_r(&mut self) -> InterpreterResult<()> {
        let a = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
        let b = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        self.stack.push(a);
        self.stack.push(b);

        Ok(())
    }
    /// Prints a summary of the unit system, including all base units, derived units, and their scale and offset.
    pub fn op_upper_u(&mut self) -> InterpreterResult<()> {
        let mut output = String::from("Base units:\n");

        for u in &self.unit_system.base_units() {
            output.push_str(&format!("{}, ", u.symbol));
        }
        output.pop();
        output.pop();
        output.push_str("\n\nDerived units:\n");

        for u in &self.unit_system.derived_units() {
            output.push_str(&format!(
                "{} = {} ({}) + {}\n",
                u.symbol, u.scale, u.exponents, u.offset
            ));
        }

        (self.output)(Output::Message(output));

        Ok(())
    }
    /// Invokes the unit solver. See [here](https://github.com/eternal-flame-AD/unitdc-rs/wiki/The-Unit-Solver) for instructions.
    pub fn op_s(&mut self) -> InterpreterResult<()> {
        let target = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;

        // first figure out how many known quantities we have
        let n_src_quantities = target.number_in_derived_unit().to_integer();
        if n_src_quantities.to_usize().unwrap_or(0) > self.stack.len() {
            return Err(InterpreterError::StackUnderflow);
        }
        let src_quantities = self
            .stack
            .split_off(self.stack.len() - n_src_quantities.to_usize().unwrap_or(0));
        let dst_unit = target.unit.reduce();
        // Check that all units involved are present
        let mut units_involved = Vec::new();
        for q in &src_quantities {
            for u in &q.unit.0 {
                if !units_involved.contains(&u.unit) {
                    units_involved.push(u.unit.clone());
                }
            }
        }
        for u in &dst_unit.0 {
            if !units_involved.contains(&u.unit) {
                return Err(InterpreterError::IncompatibleUnits(dst_unit));
            }
        }
        let mut result_coefs = Vec::with_capacity(units_involved.len());
        for u in &units_involved {
            let mut coef = 0;
            for u2 in &dst_unit.0 {
                if u2.unit == *u {
                    coef = u2.exponent;
                    break;
                }
            }
            result_coefs.push(coef);
        }
        let mut unit_coefs = Vec::with_capacity(src_quantities.len());
        for q in &src_quantities {
            let mut unit_coef = Vec::with_capacity(units_involved.len());
            for u in &units_involved {
                let mut coef = 0;
                for u2 in &q.unit.0 {
                    if u2.unit == *u {
                        coef = u2.exponent;
                        break;
                    }
                }
                unit_coef.push(coef);
            }
            unit_coefs.push(unit_coef);
        }
        let mut lin = LinearSystem::new_equation_system(transpose(&unit_coefs), result_coefs);
        let soln = lin.solve();
        if soln.is_none() {
            return Err(InterpreterError::NoSolution(
                "failed to solve unit conversion".to_string(),
            ));
        }
        if lin.is_overdetermined() {
            return Err(InterpreterError::NoSolution(
                "Linear system is overdetermined".to_string(),
            ));
        }
        if lin.is_underdetermined() {
            return Err(InterpreterError::NoSolution(
                "Linear system is underdetermined".to_string(),
            ));
        }
        let mut result = Quantity::new(BigRational::from_usize(1).unwrap(), dst_unit);
        for (q, coef) in src_quantities.into_iter().zip(soln.unwrap()) {
            result.number *= q.number.pow(coef);
        }
        result.use_derived_unit = target.use_derived_unit;
        self.stack.push(result);

        Ok(())
    }
}
