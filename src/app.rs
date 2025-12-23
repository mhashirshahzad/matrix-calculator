use crate::matrix::{transpose::transpose, types::Matrix};

pub struct App {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<String>>,
}

impl App {
    pub fn new() -> Self {
        let rows = 2;
        let cols = 3;

        Self {
            rows,
            cols,
            cells: vec![vec!["0".into(); cols]; rows],
        }
    }

    pub fn to_matrix(&self) -> Result<Matrix<f64>, &'static str> {
        let mut data = Vec::with_capacity(self.rows);

        for row in &self.cells {
            let mut r = Vec::with_capacity(self.cols);
            for cell in row {
                r.push(cell.parse::<f64>().map_err(|_| "invalid number")?);
            }
            data.push(r);
        }

        Matrix::new(data)
    }
}
