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

use num_traits::{One, Zero};
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    for j in 0..matrix[0].len() {
        let mut row = Vec::new();
        for src_row in matrix {
            row.push(src_row[j].clone());
        }
        result.push(row);
    }
    result
}

#[derive(Debug, Clone)]
pub struct LinearSystem<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + Clone
        + Zero
        + One,
{
    matrix: Vec<Vec<T>>,
}

impl<T> LinearSystem<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + PartialOrd
        + Clone
        + Zero
        + One,
{
    pub fn n(&self) -> usize {
        self.matrix.len()
    }
    pub fn m(&self) -> usize {
        if self.matrix.is_empty() {
            return 0;
        }
        self.matrix[0].len()
    }
    pub fn new(matrix: Vec<Vec<T>>) -> Self {
        Self { matrix }
    }
    pub fn new_equation_system(left: Vec<Vec<T>>, right: Vec<T>) -> Self {
        let mut matrix = left;
        for (i, row) in matrix.iter_mut().enumerate() {
            row.push(right[i].clone());
        }
        Self { matrix }
    }
    pub fn is_pivoted(&self, row: usize) -> bool {
        let mut cur_col = 0;
        while cur_col < self.m() - 1 && self.matrix[row][cur_col] == T::zero() {
            cur_col += 1;
        }
        if self.matrix[row][cur_col] != T::one() {
            return false;
        }
        for i in 0..self.n() {
            if i != row && self.matrix[i][cur_col] != T::zero() {
                return false;
            }
        }
        true
    }
    pub fn n_pivoted(&self) -> usize {
        self.matrix
            .iter()
            .enumerate()
            .filter(|(i, _)| self.is_pivoted(*i))
            .count()
    }
    fn scale_row(&mut self, i: usize, factor: T) {
        for j in 0..self.m() {
            self.matrix[i][j] = self.matrix[i][j].clone() * factor.clone();
        }
    }
    fn add_row(&mut self, src: usize, dst: usize, factor: T) {
        for j in 0..self.m() {
            self.matrix[dst][j] =
                self.matrix[dst][j].clone() + self.matrix[src][j].clone() * factor.clone();
        }
    }
    fn reduce_column(&mut self, column: usize) {
        let mut pivot_row = 0;
        while pivot_row < self.n()
            && (self.is_pivoted(pivot_row) || self.matrix[pivot_row][column] == T::zero())
        {
            pivot_row += 1;
        }
        if pivot_row == self.n() || self.matrix[pivot_row][column] == T::zero() {
            return;
        }
        let n_pivoted = self.n_pivoted();
        if pivot_row != n_pivoted {
            self.matrix.swap(pivot_row, n_pivoted);
            pivot_row = n_pivoted;
        }
        let scale_factor = self.matrix[pivot_row][column].clone();
        self.scale_row(n_pivoted, T::one() / scale_factor);
        for row in 0..self.n() {
            if row != n_pivoted && self.matrix[row][column] != T::zero() {
                let factor = self.matrix[row][column].clone();
                self.add_row(n_pivoted, row, -factor);
            }
        }
    }
    pub fn is_underdetermined(&self) -> bool {
        self.n_pivoted() > self.m() - 1
    }
    pub fn is_overdetermined(&self) -> bool {
        self.n_pivoted() < self.m() - 1
    }
    pub fn solve(&mut self) -> Option<Vec<T>> {
        for col in 0..if self.n() < self.m() {
            self.n()
        } else {
            self.m()
        } {
            self.reduce_column(col);
        }
        let mut result = Vec::new();
        for row in 0..self.n() {
            if self.is_pivoted(row) {
                result.push(self.matrix[row][self.m() - 1].clone());
            } else if self.matrix[row][self.m() - 1] != T::zero() {
                return None;
            }
        }
        Some(result)
    }
}
