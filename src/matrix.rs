use core::ops::Index;
use core::ops::IndexMut;
use std::ops::Mul;
use std::ops::Add;
use std::iter::Sum;

// sources for information:
// the <Output = T> parts added onto Mul and Add:
// https://www.reddit.com/r/rust/comments/qlyn12/how_to_write_a_generic_function_for_only_numeric/
//  - Comment by Nilstrieb about requiring the output type

pub struct Matrix <T: Sized + Default + Copy + Mul<Output = T> + Add<Output = T> + Sum> {
    values : Vec<T>,
    pub rows : usize,
    pub columns : usize,
}

impl <T: Sized + Default + Copy + Mul<Output = T> + Add<Output = T> + Sum> Matrix <T> {
    pub fn new(rows: usize, columns : usize) -> Self {
        let values : Vec<T> = vec![Default::default();columns * rows];
        Matrix {
            values,
            rows,
            columns,
        }
    }

    pub fn from(new_values: Vec<T>, rows: usize, columns: usize) -> Self {
        let mut values : Vec<T> = vec![Default::default();columns * rows];
        for (index, val) in new_values.iter().enumerate() {
            values[index] = *val;
        }
        Matrix {
            values,
            rows,
            columns,
        }
    }

    pub fn get_row<'a>(&'a self, row: usize) -> impl Iterator<Item = &'a T> {
        self.values.iter().skip(row * self.columns).take(self.columns)
    }

    pub fn get_col<'a>(&'a self, col: usize) -> impl Iterator<Item = &'a T> {
        self.values.iter().skip(col).step_by(self.columns)
    }

}

impl <T: Sized + Default + Copy + Mul<Output = T> + Add<Output = T> + Sum> Index<(usize, usize)> for Matrix <T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (col, row) = index;
        &self.values[row * self.columns + col]
    }
}


impl <T: Sized + Default + Copy + Mul<Output = T> + Add<Output = T> + Sum> IndexMut<(usize, usize)> for Matrix <T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (col, row) = index;
        &mut self.values[row * self.columns + col]
    }
}

impl <T: Sized + Default + Copy + Mul<Output = T> + Add<Output = T> + Sum> Mul for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.columns == rhs.rows);
        let mut new_matrix = Matrix::new(self.rows, rhs.columns);
        for row in 0..self.rows {
            for col in 0..rhs.columns {
                let pairs = self.get_row(row)
                    .zip(rhs.get_col(col))
                    .map(|(&a, &b)| a * b);
                new_matrix[(col, row)] = pairs.sum();
            }
        }
        new_matrix
    }
}