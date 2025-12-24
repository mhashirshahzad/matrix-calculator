use crate::matrix::{transpose::transpose, types::Matrix};

pub struct App {
    pub matrix_a: Matrix,
    pub matrix_b: Matrix,
}

impl App {
    pub fn new() -> Self {
        let a_rows = 2;
        let a_cols = 3;

        let b_rows = 3;
        let b_cols = 2;

        Self {
            matrix_a: Matrix {
                rows: a_rows,
                cols: a_cols,
                data: vec![vec!["0".into(); a_cols]; a_rows],
            },

            matrix_b: Matrix {
                rows: b_rows,
                cols: b_cols,

                data: vec![vec!["0".into(); b_cols]; b_rows],
            },
        }
    }

    pub fn get_matrix(&self, m: &Matrix) -> Result<Matrix, &'static str> {
        let mut data = Vec::with_capacity(m.rows);

        for row in &m.data {
            let mut r = Vec::with_capacity(m.cols);
            for cell in row {
                r.push(cell.into());
            }
            data.push(r);
        }

        Matrix::new(data)
    }
}
