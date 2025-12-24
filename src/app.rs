use crate::matrix::{transpose::transpose, types::Matrix};

pub struct App {
    pub matrix_a: Matrix,
    pub matrix_b: Matrix,
}

impl App {
    pub fn new() -> Self {
        let rows = 2;
        let cols = 3;

        Self {
            matrix_a: Matrix {
                rows,
                cols,
                data: vec![vec!["0".into(); cols]; rows],
            },

            matrix_b: Matrix {
                rows,
                cols,

                data: vec![vec!["0".into(); cols]; rows],
            },
        }
    }

    pub fn to_matrix(&self) -> Result<Matrix, &'static str> {
        let mut data = Vec::with_capacity(self.matrix_a.rows);

        for row in &self.matrix_a.data {
            let mut r = Vec::with_capacity(self.matrix_a.cols);
            for cell in row {
                r.push(cell.into());
            }
            data.push(r);
        }

        Matrix::new(data)
    }
}
