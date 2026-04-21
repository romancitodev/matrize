mod helpers;

pub use helpers::*;
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Matrix<const N: usize, const M: usize, E: IsNumber = i32> {
    pub elements: [[E; M]; N],
}

impl<const N: usize, const M: usize, E: IsNumber> Default for Matrix<N, M, E> {
    fn default() -> Self {
        Self::zeros()
    }
}

impl<const N: usize, E: IsNumber> Matrix<N, N, E> {
    pub const fn identity() -> Matrix<N, N, E> {
        let () = assert!(N > 0, "The matrix must be 1x1 at least.");

        let mut elements = [[E::ZERO; N]; N];
        let mut i = 0;
        while i < N {
            elements[i][i] = E::ONE;
            i += 1;
        }
        Matrix { elements }
    }

    pub const fn diagonal_as_slice(&self) -> [E; N] {
        let mut d = [E::ZERO; N];
        let mut i = 0;
        while i < N {
            d[i] = self.elements[i][i];
            i += 1;
        }
        d
    }

    pub const fn diagonal(&self) -> Matrix<N, N, E> {
        let mut r = Matrix::zeros();
        let mut i = 0;
        while i < N {
            r.elements[i][i] = self.elements[i][i];
            i += 1;
        }
        r
    }

    pub const fn off_diagonal(&self) -> Matrix<N, N, E> {
        let mut r = *self;
        let mut i = 0;
        while i < N {
            r.elements[i][i] = E::ZERO;
            i += 1;
        }
        r
    }
}

impl<const N: usize, const M: usize, E: IsNumber> Matrix<N, M, E> {
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

    //TODO: finish this function
    pub const fn determinant() -> Matrix<1, 1, E> {
        todo!()
    }
}

impl<const N: usize, const M: usize, E: IsNumber> From<[[E; M]; N]> for Matrix<N, M, E> {
    fn from(elements: [[E; M]; N]) -> Self {
        Matrix { elements }
    }
}

impl<const M: usize, E: IsNumber> From<[E; M]> for Matrix<1, M, E> {
    fn from(row: [E; M]) -> Self {
        Matrix { elements: [row] }
    }
}

impl<const N: usize, const M: usize, E: IsNumber> Add for Matrix<N, M, E> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self; // Evita crear una matriz de ceros innecesariamente
        for i in 0..N {
            for j in 0..M {
                result.elements[i][j] = result.elements[i][j] + rhs.elements[i][j];
            }
        }
        result
    }
}

impl<const N: usize, const M: usize, E: IsNumber> Sub for Matrix<N, M, E> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for i in 0..N {
            for j in 0..M {
                result.elements[i][j] = result.elements[i][j] - rhs.elements[i][j];
            }
        }
        result
    }
}

impl<const N: usize, const M: usize, E: IsNumber> Mul<E> for Matrix<N, M, E> {
    type Output = Self;

    fn mul(self, scalar: E) -> Self::Output {
        let mut result = self;
        for i in 0..N {
            for j in 0..M {
                result.elements[i][j] = result.elements[i][j] * scalar;
            }
        }
        result
    }
}

impl<const N: usize, const M: usize, const L: usize, E: IsNumber + AddAssign> Mul<Matrix<M, L, E>>
    for Matrix<N, M, E>
{
    type Output = Matrix<N, L, E>;

    fn mul(self, rhs: Matrix<M, L, E>) -> Self::Output {
        let mut result = Matrix::<N, L, E>::zeros();
        for i in 0..N {
            for j in 0..L {
                let mut sum = E::ZERO;
                for k in 0..M {
                    sum += self.elements[i][k] * rhs.elements[k][j];
                }
                result.elements[i][j] = sum;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: Matrix<2, 3, i16> = matrix![[1, 2, 3], [4, 5, 6]];
    const B: Matrix<2, 3, i16> = matrix![[2, 4, 6], [8, 10, 12]];
    const C: Matrix<3, 4, i16> = matrix![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
    const D: Matrix<3, 2, i16> = B.transpose();

    #[test]
    fn sum_of_matrices() {
        let result = A + B;
        assert_eq!(result, matrix![[3, 6, 9], [12, 15, 18]]);
    }

    #[test]
    fn sub_of_matrices() {
        let result = B - A;
        assert_eq!(result, A);
    }

    #[test]
    fn mul_by_scalar() {
        let lambda = 2;
        let result = A * lambda;
        assert_eq!(result, B);
    }

    mod multiplication {
        use super::*;
        #[test]
        fn b_mul_c() {
            let result = A * C;
            assert_eq!(result, matrix![[38, 44, 50, 56], [83, 98, 113, 128]]);
        }

        #[test]
        fn a_d_not_eq_d_a() {
            let ad = A * D;
            let da = D * A;
            assert_ne!(ad.rows(), da.rows());
            assert_ne!(ad.columns(), da.columns());
        }
    }
}

pub mod iter {
    use super::*;
    pub fn jacobi<const N: usize, E: IsSigned>(
        a: &Matrix<N, N, E>,
        b: &[E; N],
        epsilon: f64,
        max_iter: usize,
    ) -> Option<[E; N]> {
        let mut x = [E::ZERO; N];

        for _ in 0..max_iter {
            let mut new_x = [E::ZERO; N];
            for i in 0..N {
                let diag = a.elements[i][i];
                if diag == E::ZERO {
                    return None;
                }

                let sigma: E = (0..N)
                    .filter(|&j| j != i)
                    .map(|j| a.elements[i][j] * x[j])
                    .sum();

                new_x[i] = (b[i] - sigma) / diag;
            }

            let max = |acc, val| if acc > val { acc } else { val };
            let error = (0..N).map(|i| (new_x[i] - x[i]).abs()).fold(E::ZERO, max);

            x = new_x;

            if error.to_f64()? < epsilon {
                return Some(x);
            }
        }

        None
    }
}
