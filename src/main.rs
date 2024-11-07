use matrize::{matrix, Matrix};

fn main() {
    let zeros = Matrix::<2, 2>::zeros();
    println!("{zeros:#?}");
    let id = Matrix::<3, 3>::identity();
    println!("{id:#?}");
    let id = Matrix::<4, 4>::identity();
    println!("{id:#?}");
    let manual = matrix![[[1, 2, 3], [4, 5, 6]]];
    let squared = Matrix::<2, 2>::square(1);
    let rows = squared.rows();

    println!("{squared:?}");
}
