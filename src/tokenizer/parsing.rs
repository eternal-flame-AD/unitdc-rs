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
