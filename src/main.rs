use matrize::{iter, matrix, Matrix};

const A: Matrix<2, 2, f64> = matrix![[4.0, 1.0], [-1.0, 3.0]];

fn main() {
    let b = [3.0, 2.0];
    match iter::jacobi(&A, &b, 1e-12, 100) {
        Some(x) => println!("Solucion: {x:?}"),
        None => println!("No hay solucion"),
    };

    println!("hello");
}
