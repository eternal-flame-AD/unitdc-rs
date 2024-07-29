use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::{Div, Mul, Neg},
};

use num_rational::BigRational;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct UnitSystem {
    base_units: HashMap<String, BaseUnit>,
    derived_units: HashMap<String, DerivedUnit>,
}

pub enum Unit<'a> {
    Base(&'a BaseUnit),
    Derived(&'a DerivedUnit),
}

impl Debug for UnitSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut base_units: Vec<&BaseUnit> = self.base_units.values().collect();
        base_units.sort_by(|a, b| a.symbol.cmp(&b.symbol));
        let mut derived_units: Vec<&DerivedUnit> = self.derived_units.values().collect();
        derived_units.sort_by(|a, b| a.symbol.cmp(&b.symbol));
        f.debug_struct("UnitSystem")
            .field("base_units", &base_units)
            .field("derived_units", &derived_units)
            .finish()
    }
}

impl Default for UnitSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl UnitSystem {
    pub fn new() -> Self {
        UnitSystem {
            base_units: HashMap::new(),
            derived_units: HashMap::new(),
        }
    }
    pub fn lookup_unit(&self, symbol: &str) -> Option<Unit> {
        self.lookup_base_unit(symbol)
            .map(Unit::Base)
            .or_else(|| self.lookup_derived_unit(symbol).map(Unit::Derived))
    }
    pub fn lookup_base_unit(&self, symbol: &str) -> Option<&BaseUnit> {
        self.base_units.get(symbol)
    }
    pub fn lookup_derived_unit(&self, symbol: &str) -> Option<&DerivedUnit> {
        self.derived_units.get(symbol)
    }
    pub fn push_base_unit(&mut self, unit: BaseUnit) {
        self.base_units.insert(unit.symbol.clone(), unit);
    }
    pub fn push_derived_unit(&mut self, unit: DerivedUnit) {
        self.derived_units.insert(unit.symbol.clone(), unit);
    }
    pub fn base_units(&self) -> Vec<BaseUnit> {
        let mut base_units = Vec::new();
        for unit in self.base_units.values() {
            base_units.push(unit.clone());
        }
        base_units.sort_by(|a, b| a.symbol.cmp(&b.symbol));
        base_units
    }
    pub fn derived_units(&self) -> Vec<DerivedUnit> {
        let mut derived_units = Vec::new();
        for unit in self.derived_units.values() {
            derived_units.push(unit.clone());
        }
        derived_units.sort_by(|a, b| format!("{}", a.exponents).cmp(&format!("{}", b.exponents)));
        derived_units
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct BaseUnit {
    pub symbol: String,
}

impl Display for BaseUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivedUnit {
    pub symbol: String,
    pub offset: BigRational,
    pub scale: BigRational,
    pub exponents: UnitCombo,
}

impl Mul for DerivedUnit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        DerivedUnit {
            symbol: format!("{}*{}", self.symbol, rhs.symbol),
            offset: self.offset * rhs.scale.clone() + rhs.offset,
            scale: self.scale * rhs.scale,
            exponents: self.exponents * rhs.exponents,
        }
    }
}

impl Div for DerivedUnit {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        DerivedUnit {
            symbol: format!("{}/{}", self.symbol, rhs.symbol),
            offset: self.offset * rhs.scale.clone() - rhs.offset,
            scale: self.scale / rhs.scale,
            exponents: self.exponents / rhs.exponents,
        }
    }
}

impl Display for DerivedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

/// A `UnitExponent` is the combination of a base unit and an exponent.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitExponent {
    pub unit: BaseUnit,
    pub exponent: i32,
}

/// A `UnitCombo` is a combination of `UnitExponent`s, which described an arbitrary "unit".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitCombo(pub Vec<UnitExponent>);

impl Default for UnitCombo {
    fn default() -> Self {
        Self::new()
    }
}

impl UnitCombo {
    pub fn new() -> Self {
        UnitCombo(Vec::new())
    }
    pub fn push_base_unit(&mut self, unit: BaseUnit, exponent: i32) {
        for exponents in self.0.iter_mut() {
            if exponents.unit == unit {
                exponents.exponent += exponent;
                if exponents.exponent == 0 {
                    self.0.retain(|e| e.unit != unit);
                }
                return;
            }
        }
        self.0.push(UnitExponent { unit, exponent });
    }
    pub fn push_derived_unit(&mut self, unit: DerivedUnit) {
        for c in unit.exponents.0 {
            self.push_base_unit(c.unit, c.exponent);
        }
    }
}

impl Display for UnitCombo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut exponents: Vec<&UnitExponent> = self.0.iter().filter(|e| e.exponent != 0).collect();
        if exponents.is_empty() {
            write!(f, "1")?;
            return Ok(());
        }
        exponents.sort_by(|a, b| b.exponent.cmp(&a.exponent));
        for exponent in exponents.iter() {
            if exponent.exponent == 1 {
                write!(f, "{}", exponent.unit.symbol)?;
            } else {
                write!(f, "({}^{})", exponent.unit.symbol, exponent.exponent)?;
            }
        }
        Ok(())
    }
}

impl PartialEq for UnitCombo {
    fn eq(&self, other: &Self) -> bool {
        let mut self_exponents = self.0.clone();
        let mut other_exponents = other.0.clone();
        self_exponents.sort_by(|a, b| a.unit.symbol.cmp(&b.unit.symbol));
        other_exponents.sort_by(|a, b| a.unit.symbol.cmp(&b.unit.symbol));
        self_exponents == other_exponents
    }
}

impl UnitCombo {
    pub fn is_unitless(&self) -> bool {
        self.0.is_empty()
    }
    pub fn reduce(&self) -> Self {
        let mut exponents: Vec<UnitExponent> = Vec::new();
        for component in self.0.iter() {
            let mut found = false;
            for exponent in exponents.iter_mut() {
                if exponent.unit == component.unit {
                    exponent.exponent += component.exponent;
                    found = true;
                    break;
                }
            }
            if !found {
                exponents.push(component.clone());
            }
        }
        UnitCombo(
            exponents
                .iter()
                .filter(|e| e.exponent != 0)
                .cloned()
                .collect(),
        )
    }
}

impl Mul for UnitCombo {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_exponents = self.0.clone();
        new_exponents.extend(rhs.0);
        UnitCombo(new_exponents).reduce()
    }
}

impl Neg for UnitCombo {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut new_exponents = self.0.clone();
        for exponent in new_exponents.iter_mut() {
            exponent.exponent *= -1;
        }
        UnitCombo(new_exponents).reduce()
    }
}

impl Div for UnitCombo {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * -rhs
    }
}
