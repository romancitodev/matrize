mod helpers;

pub use helpers::*;

#[derive(Debug)]
pub struct Matrix<const N: usize, const M: usize, E: IsNumber = i32> {
    pub elements: [[E; M]; N],
}

impl<const N: usize, const M: usize, E: IsNumber + Default + Copy> Default for Matrix<N, M, E> {
    fn default() -> Self {
        Matrix::new(E::default())
    }
}

impl<const N: usize, E: IsNumber + Default + Copy> Matrix<N, N, E> {
    const _INVALID: () = {
        if N == 0 {
            panic!()
        };
    };

    pub const fn square(value: E) -> Matrix<N, N, E> {
        Matrix {
            elements: [[value; N]; N],
        }
    }

    pub const fn identity() -> Matrix<N, N, E> {
        let mut elements = [[E::ZERO; N]; N];
        let mut index = 0;
        while index < N {
            elements[index][index] = E::ONE;
            index += 1;
        }
        Matrix { elements }
    }
}

impl<const N: usize, const M: usize, E: IsNumber + Default + Copy> Matrix<N, M, E> {
    pub const fn new(value: E) -> Matrix<N, M, E> {
        Matrix {
            elements: [[value; M]; N],
        }
    }

    pub const fn zeros() -> Matrix<N, M, E> {
        Matrix {
            elements: [[E::ZERO; M]; N],
        }
    }

    pub const fn rows(&self) -> usize {
        N
    }

    pub const fn columns(&self) -> usize {
        M
    }

    pub fn say_hi(&self) {
        println!("Hi from this matrix");
    }
}
