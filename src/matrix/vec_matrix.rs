use std::ops::{Index, IndexMut, Mul, Add};
use std::iter::Sum;
use super::{Matrix, MatrixConversion, VecMatrix};

// sources for information:
// the <Output = T> parts added onto Mul and Add:
// https://www.reddit.com/r/rust/comments/qlyn12/how_to_write_a_generic_function_for_only_numeric/
//  - Comment by Nilstrieb about requiring the output type


impl <T> Index <(usize, usize)> for VecMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.data[x + y * self.columns]
    }
}

impl <T> IndexMut <(usize, usize)> for VecMatrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.data[x + y * self.columns]  
    }
}


impl <T> VecMatrix<T>
where
    T: Default + Copy
{
    pub fn new(rows: usize, columns: usize) -> Self {
        VecMatrix {
            data: std::iter::repeat(Default::default()).take(columns * rows).collect(),
            columns,
            rows
        }
    }
}

impl <T> Mul for VecMatrix<T>
where 
    T: Default + Copy + Mul<Output = T> + Add<Output = T> + Sum
{
    type Output = VecMatrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.columns == rhs.rows);
        let mut new_matrix = VecMatrix::new(self.rows, rhs.columns);
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

// it seems the where requirements are due to adding Mul, Index, and IndexMut to the Matrix Trait
impl <'a, T> Matrix <'a, T> for VecMatrix<T>
where
    T: 'a + Copy + Default + Mul<Output = T> + Add<Output = T> + Sum
{
    fn rows(&self) -> usize {
        self.rows
    }
    fn columns(&self) -> usize {
        self.columns
    }
    fn get_col(&'a self, column: usize) -> impl Iterator<Item = &'a T> {
        self.data.iter().skip(column).step_by(self.columns)
    }
    fn get_row(&'a self, row: usize) -> impl Iterator<Item = &'a T> {
        self.data.iter().skip(row * self.columns).take(self.columns)
    }
}

impl <T> MatrixConversion for VecMatrix<T>
where
    T: Clone + Default,
{
    type Source = Vec<T>;
    type MatrixType = VecMatrix<T>;

    fn matrix_convert(new_data: Self::Source, rows: usize, columns: usize) -> Self::MatrixType {
        let data : Vec<T> = new_data.into_iter()
            .chain(std::iter::repeat(Default::default()))
            .take(rows * columns)
            .collect();
        VecMatrix {
            data,
            columns,
            rows,
        }
    }
}

