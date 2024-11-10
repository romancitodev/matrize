use matrize::{matrix, Matrix};

const A: Matrix<2, 3, i16> = matrix![[[1, 2, 3], [4, 5, 6]]];
const B: Matrix<2, 3, i16> = matrix![[[2, 4, 6], [8, 10, 12]]];
const C: Matrix<3, 4, i16> = matrix![[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]];
const D: Matrix<3, 2, i16> = Matrix::<2, 3, _>::transpose(&B);

fn main() {
    println!("A + B");
    sum_of_matrices();
    println!("A - B");
    sub_of_matrices();
    println!("A * 5");
    mul_by_scalar();
    println!("A * B");
    mul_of_matrices();
}

fn sum_of_matrices() {
    let result = A + B;
    println!("{result:?}");
}

fn sub_of_matrices() {
    let result = A - B;
    println!("{result:?}");
}

fn mul_by_scalar() {
    let lambda = 5;
    let result = A * lambda;
    println!("{result:?}");
}

fn mul_of_matrices() {
    let result = B * C;
    println!("{result:?}");
    // A * D != D * A
    let result = A * D;
    println!("{result:?}");
    let result = D * A;
    println!("{result:?}");
}
