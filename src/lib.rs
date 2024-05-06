pub mod matrix;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use matrix::Matrix;
        // test empty matrix to make sure index works
        let mut result : Matrix<u16> = Matrix::new(2,2);
        assert_eq!(result[(1,0)], 0);
        assert_eq!(result[(0,0)], 0);
        assert_eq!(result[(0,1)], 0);
        assert_eq!(result[(1,1)], 0);
        // modify matrix to have values
        result[(0,0)] = 42;
        result[(1,0)] = 75;
        result[(0,1)] = 21;
        result[(1,1)] = 100;
        assert_eq!(result[(1,0)], 75);
        assert_eq!(result[(0,0)], 42);
        assert_eq!(result[(0,1)], 21);
        assert_eq!(result[(1,1)], 100);
        // test identity matrix
        let two_identity : Matrix<u16> = Matrix::from(vec![1,0,0,1], 2, 2);
        assert_eq!(two_identity[(1,0)], 0);
        assert_eq!(two_identity[(0,0)], 1);
        assert_eq!(two_identity[(0,1)], 0);
        assert_eq!(two_identity[(1,1)], 1);
        let same_result = result * two_identity;
        assert_eq!(same_result[(1,0)], 75);
        assert_eq!(same_result[(0,0)], 42);
        assert_eq!(same_result[(0,1)], 21);
        assert_eq!(same_result[(1,1)], 100);
        let test_mat_one : Matrix<u16> = Matrix::from((1..7).collect(), 2, 3);
        let test_mat_two : Matrix<u16> = Matrix::from((7..13).collect(), 3, 2);
        //print!("{}\n", test_mat_two);
        let test_mat_result : Matrix<u16> = test_mat_one * test_mat_two;
        //print!("{}\n", test_mat_result);
        assert_eq!(test_mat_result[(0,0)], 58);
        assert_eq!(test_mat_result[(1,0)], 64);
        assert_eq!(test_mat_result[(0,1)], 139);
        assert_eq!(test_mat_result[(1,1)], 154);
    }
}
