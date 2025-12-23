use super::types::Matrix;

pub fn transpose<T: Clone>(matrix: &Matrix<T>) -> Matrix<T> {
    let mut result = Vec::with_capacity(matrix.cols);

    for j in 0..matrix.cols {
        let mut row = Vec::with_capacity(matrix.rows);
        for i in 0..matrix.rows {
            row.push(matrix.data[i][j].clone());
        }
        result.push(row);
    }

    Matrix {
        rows: matrix.cols,
        cols: matrix.rows,
        data: result,
    }
}
