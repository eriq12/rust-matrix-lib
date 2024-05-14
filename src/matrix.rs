use std::ops::{Index, IndexMut};
pub mod vec_matrix;

pub trait Matrix<'a, T> : Index<(usize,usize)> + IndexMut<(usize, usize)>
where
    Self: Sized,
    T: 'a
{
    fn rows(&self) -> usize;
    fn columns(&self) -> usize;
    fn get_row(&'a self, row: usize) -> impl Iterator<Item = &'a T>;
    fn get_col(&'a self, column: usize) -> impl Iterator<Item = &'a T>;
}

pub trait MatrixConversion{
    type Source;
    type MatrixType;
    fn matrix_convert(new_data: Self::Source, rows: usize, columns: usize) -> Self::MatrixType;
}

// matrix stucts
pub struct VecMatrix <T>{
    data: Vec<T>,
    columns: usize,
    rows: usize,
}