pub mod matrix;

// <VecMatrix<u16> as MatrixConversion> is how you could use "fully-qualified syntax" to specify
// a method from a specific trait (I used to use from for matrix conversion, but I think it's too annoying)
// source: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html, Listing 19-21

#[cfg(test)]
mod tests {
    use super::*;
    use matrix::{VecMatrix, MatrixConversion};

    #[test]
    fn it_works() {
        // test empty matrix to make sure index works
        let mut result : VecMatrix<u16> = VecMatrix::new(2,2);
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
        // test identity matrixW
        let two_identity_data : Vec<u16> = vec![1,0,0,1];
        let two_identity : VecMatrix<u16> = VecMatrix::matrix_convert(two_identity_data, 2, 2);
        assert_eq!(two_identity[(1,0)], 0);
        assert_eq!(two_identity[(0,0)], 1);
        assert_eq!(two_identity[(0,1)], 0);
        assert_eq!(two_identity[(1,1)], 1);
        let same_result = result * two_identity;
        assert_eq!(same_result[(1,0)], 75);
        assert_eq!(same_result[(0,0)], 42);
        assert_eq!(same_result[(0,1)], 21);
        assert_eq!(same_result[(1,1)], 100);
        let test_mat_one : VecMatrix<u16> = VecMatrix::matrix_convert((1..7).collect::<Vec<u16>>(), 2, 3);
        let test_mat_two : VecMatrix<u16> = VecMatrix::matrix_convert((7..13).collect::<Vec<u16>>(), 3, 2);
        //print!("{}\n", test_mat_two);
        let test_mat_result : VecMatrix<u16> = test_mat_one * test_mat_two;
        //print!("{}\n", test_mat_result);
        assert_eq!(test_mat_result[(0,0)], 58);
        assert_eq!(test_mat_result[(1,0)], 64);
        assert_eq!(test_mat_result[(0,1)], 139);
        assert_eq!(test_mat_result[(1,1)], 154);
    }
}
