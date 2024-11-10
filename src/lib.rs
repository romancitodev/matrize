mod helpers;

use std::ops::{Add, AddAssign, Mul, Sub};

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
            panic!("The matrix must be 1x1 at least.");
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

    pub const fn transpose(&self) -> Matrix<M, N, E> {
        let mut m = Matrix::<M, N, E>::zeros();
        let mut i = 0;
        while i < N {
            let mut j = 0;
            while j < M {
                m.elements[j][i] = self.elements[i][j];
                j += 1;
            }
            i += 1;
        }
        m
    }
}

impl<const N: usize, const M: usize, E: IsNumber + Default + Copy + Add<Output = E>> std::ops::Add
    for Matrix<N, M, E>
{
    type Output = Matrix<N, M, E>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Matrix::<N, M, E>::zeros();
        for i in 0..N {
            for j in 0..M {
                result.elements[i][j] = self.elements[i][j] + rhs.elements[i][j];
            }
        }
        result
    }
}

impl<const N: usize, const M: usize, E: IsNumber + Default + Copy + Sub<Output = E>> std::ops::Sub
    for Matrix<N, M, E>
{
    type Output = Matrix<N, M, E>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Matrix::<N, M, E>::zeros();
        for i in 0..N {
            for j in 0..M {
                result.elements[i][j] = self.elements[i][j] - rhs.elements[i][j];
            }
        }
        result
    }
}

// Multiplying it by a scalar:
impl<const N: usize, const M: usize, E: IsNumber + Default + Copy + Mul<Output = E>> Mul<E>
    for Matrix<N, M, E>
{
    type Output = Matrix<N, M, E>;

    fn mul(self, scalar: E) -> Self::Output {
        let mut result = Matrix::<N, M, E>::zeros();
        for i in 0..N {
            for j in 0..M {
                result.elements[i][j] = self.elements[i][j] * scalar;
            }
        }
        result
    }
}

// Multiplying it by other matrix:
impl<
        const N: usize,
        const M: usize,
        const L: usize,
        E: IsNumber + Default + Copy + Mul<Output = E> + AddAssign,
    > Mul<Matrix<M, L, E>> for Matrix<N, M, E>
{
    type Output = Matrix<N, L, E>;

    fn mul(self, rhs: Matrix<M, L, E>) -> Self::Output {
        let mut result = Matrix::<N, L, E>::zeros();
        for i in 0..N {
            for j in 0..L {
                let mut sum = E::default();
                for k in 0..M {
                    sum += self.elements[i][k] * rhs.elements[k][j];
                }
                result.elements[i][j] = sum;
            }
        }
        result
    }
}
