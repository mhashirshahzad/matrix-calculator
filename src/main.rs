use matrix_calculator::matrix::{transpose::transpose, types::Matrix};

fn main() -> Result<(), &'static str> {
    let m = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]])?;

    let t = transpose(&m);
    println!("Matrix: {}", m);
    println!("Transpose: {}", t);
    Ok(())
}
