use super::TokenizerError;
use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use num_traits::pow::Pow;

use std::collections::VecDeque;

pub fn parse_bigrational(s: &str) -> Result<BigRational, TokenizerError> {
    let mut numerator = BigInt::from(0);
    let mut is_decimal = false;
    let mut decimal_places = 0;
    let mut is_negative = false;
    let mut is_exponent = false;
    let mut exponent = BigInt::from(0);
    let mut exponent_is_negative = false;

    let mut s = s.chars().filter(|c| *c != '_').collect::<VecDeque<_>>();

    if s[0] == '-' {
        is_negative = true;
        s.pop_front();
    }

    while let Some(ch) = s.pop_front() {
        match ch {
            '0'..='9' => {
                if is_exponent {
                    let digit = ch.to_digit(10).unwrap();
                    let exponent_digit = BigInt::from(digit);
                    exponent *= BigInt::from(10);
                    exponent += exponent_digit;
                } else if is_decimal {
                    let digit = ch.to_digit(10).unwrap();
                    let numerator_digit = BigInt::from(digit);
                    numerator *= BigInt::from(10);
                    numerator += numerator_digit;
                    decimal_places += 1;
                } else {
                    let digit = ch.to_digit(10).unwrap();
                    let numerator_digit = BigInt::from(digit);
                    numerator *= BigInt::from(10);
                    numerator += numerator_digit;
                }
            }
            '.' => {
                if is_decimal {
                    return Err(TokenizerError::InvalidCharacter(ch));
                }
                is_decimal = true;
            }
            'e' | 'E' => {
                if is_exponent {
                    return Err(TokenizerError::InvalidCharacter(ch));
                }
                is_exponent = true;
                if let Some(ch) = s.pop_front() {
                    if ch == '-' {
                        exponent_is_negative = true;
                    } else {
                        s.push_front(ch);
                    }
                }
            }
            _ => return Err(TokenizerError::InvalidCharacter(ch))?,
        }
    }

    if is_negative {
        numerator = -numerator;
    }
    if exponent_is_negative {
        exponent = -exponent;
    }

    let without_exponent = BigRational::new_raw(numerator, BigInt::from(1));

    let exponent = exponent - BigInt::from(decimal_places);

    let exp = if exponent >= BigInt::from(0) {
        BigRational::new_raw(
            BigInt::from(10).pow(BigUint::try_from(exponent).unwrap()),
            BigInt::from(1),
        )
    } else {
        BigRational::new_raw(
            BigInt::from(1),
            BigInt::from(10).pow(BigUint::try_from(-exponent).unwrap()),
        )
    };

    Ok((without_exponent * exp).reduced())
}
