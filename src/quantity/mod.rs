use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use thiserror::Error;

pub mod units;

use units::{DerivedUnit, UnitCombo};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Quantity {
    pub number: BigRational,
    pub unit: UnitCombo,

    pub use_derived_unit: Vec<DerivedUnit>,
}

impl Serialize for Quantity {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("Quantity", 5)?;
        s.serialize_field("_str", &self.to_string())?;
        s.serialize_field("number_float", &self.number.to_f64())?;
        s.serialize_field("number", &self.number)?;
        s.serialize_field("unit", &self.unit)?;
        s.serialize_field("use_derived_unit", &self.use_derived_unit)?;
        s.end()
    }
}

#[derive(Error, Debug)]
pub enum QuantityError {
    #[error("Incompatible units")]
    IncompatibleUnits,
    #[error("Unknown unit")]
    UnknownUnit,
}

impl Quantity {
    pub fn new(number: BigRational, unit: UnitCombo) -> Self {
        Quantity {
            number,
            unit,
            use_derived_unit: Vec::new(),
        }
    }
    pub fn number_in_derived_unit(&self) -> BigRational {
        let mut number = self.number.clone();

        for d in &self.use_derived_unit {
            if d.exponents == self.unit {
                number -= d.offset.clone();
                number /= d.scale.clone();
                return number;
            }
        }

        number
    }
}

impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut number = self.number.clone();

        for d in &self.use_derived_unit {
            if d.exponents == self.unit {
                number -= d.offset.clone();
                number /= d.scale.clone();
                write!(f, "{} ({})", number.to_f64().unwrap_or(f64::NAN), d)?;
                return Ok(());
            }
        }

        write!(f, "{} ({})", number.to_f64().unwrap_or(f64::NAN), self.unit)?;

        Ok(())
    }
}

impl Add for Quantity {
    type Output = Result<Self, QuantityError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.unit != rhs.unit {
            return Err(QuantityError::IncompatibleUnits);
        }

        let number = self.number + rhs.number;
        let unit = self.unit;
        let use_derived_unit = rhs.use_derived_unit;

        Ok(Quantity {
            number,
            unit,
            use_derived_unit,
        })
    }
}

impl Sub for Quantity {
    type Output = Result<Self, QuantityError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.unit != rhs.unit {
            return Err(QuantityError::IncompatibleUnits);
        }

        let number = self.number - rhs.number;
        let unit = self.unit;
        let use_derived_unit = rhs.use_derived_unit;

        Ok(Quantity {
            number,
            unit,
            use_derived_unit,
        })
    }
}

impl Mul for Quantity {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let number = self.number * rhs.number;
        let unit = self.unit * rhs.unit;
        let use_derived_unit = rhs.use_derived_unit;

        Quantity {
            number,
            unit,
            use_derived_unit,
        }
    }
}

impl Div for Quantity {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let number = self.number / rhs.number;
        let unit = self.unit / rhs.unit;
        let use_derived_unit = rhs.use_derived_unit;

        Quantity {
            number,
            unit,
            use_derived_unit,
        }
    }
}
