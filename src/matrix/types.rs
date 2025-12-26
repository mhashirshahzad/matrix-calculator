use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<String>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<String>>) -> Result<Self, &'static str> {
        let rows = data.len();
        let cols = data.get(0).map_or(0, |r| r.len());

        if data.iter().any(|r| r.len() != cols) {
            return Err("matrix is not rectangular");
        }

        Ok(Self { rows, cols, data })
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?; // new line to make it look good

        if self.rows == 0 || self.cols == 0 {
            return writeln!(f, "[]");
        }

        /* 2. Compute max width of each column */
        let mut col_widths = vec![0; self.cols];
        for row in &self.data {
            for (j, cell) in row.iter().enumerate() {
                col_widths[j] = col_widths[j].max(cell.len());
            }
        }

        /* 3. Print rows with brackets */
        for i in 0..self.rows {
            let (left, right) = match i {
                0 if self.rows == 1 => ("⎡", "⎤"),
                0 => ("⎡", "⎤"),
                x if x == self.rows - 1 => ("⎣", "⎦"),
                _ => ("⎢", "⎥"),
            };

            write!(f, "{left} ")?;
            for j in 0..self.cols {
                write!(f, "{:>width$} ", self.data[i][j], width = col_widths[j])?;
            }
            writeln!(f, "{right}")?;
        }

        Ok(())
    }
}

impl Matrix {
    /// Helper to convert a Matrix of Strings to f64 values
    fn to_f64(&self) -> Result<Vec<Vec<f64>>, &'static str> {
        let mut result = Vec::with_capacity(self.rows);
        for row in &self.data {
            let mut r = Vec::with_capacity(self.cols);
            for cell in row {
                r.push(cell.parse::<f64>().map_err(|_| "invalid number")?);
            }
            result.push(r);
        }
        Ok(result)
    }

    /// Helper to create Matrix from Vec<Vec<f64>>
    fn from_f64(data: Vec<Vec<f64>>) -> Matrix {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        let string_data = data
            .into_iter()
            .map(|r| r.into_iter().map(|v| v.to_string()).collect())
            .collect();
        Matrix {
            rows,
            cols,
            data: string_data,
        }
    }
}

// Addition
impl Add for Matrix {
    type Output = Result<Matrix, &'static str>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            return Err("Matrix dimensions do not match for addition");
        }

        let a = self.to_f64()?;
        let b = rhs.to_f64()?;
        let mut data = vec![vec![0.0; self.cols]; self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                data[i][j] = a[i][j] + b[i][j];
            }
        }

        Ok(Matrix::from_f64(data))
    }
}

// Subtraction
impl Sub for Matrix {
    type Output = Result<Matrix, &'static str>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            return Err("Matrix dimensions do not match for subtraction");
        }

        let a = self.to_f64()?;
        let b = rhs.to_f64()?;
        let mut data = vec![vec![0.0; self.cols]; self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                data[i][j] = a[i][j] - b[i][j];
            }
        }

        Ok(Matrix::from_f64(data))
    }
}

// Multiplication
impl Mul for Matrix {
    type Output = Result<Matrix, &'static str>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            return Err("Matrix dimensions do not match for multiplication");
        }

        let a = self.to_f64()?;
        let b = rhs.to_f64()?;
        let mut data = vec![vec![0.0; rhs.cols]; self.rows];

        for i in 0..self.rows {
            for j in 0..rhs.cols {
                for k in 0..self.cols {
                    data[i][j] += a[i][k] * b[k][j];
                }
            }
        }

        Ok(Matrix::from_f64(data))
    }
}

impl Matrix {
    pub fn determinant(&self) -> Result<f64, &'static str> {
        if self.rows != self.cols {
            return Err("Determinant is only defined for square matrices");
        }

        let n = self.rows;

        let mut a: Vec<Vec<f64>> = self.to_f64()?;
        let mut det = 1.0;

        for i in 0..n {
            // Pivot search
            let mut pivot = i;
            for r in i..n {
                if a[r][i].abs() > a[pivot][i].abs() {
                    pivot = r;
                }
            }

            // Singular matrix
            if a[pivot][i] == 0.0 {
                return Ok(0.0);
            }

            // Row swap changes determinant sign
            if pivot != i {
                a.swap(pivot, i);
                det = -det;
            }

            det *= a[i][i];

            // Normalize and eliminate
            for r in (i + 1)..n {
                let factor = a[r][i] / a[i][i];
                for c in i..n {
                    a[r][c] -= factor * a[i][c];
                }
            }
        }

        Ok(det)
    }
}

impl Matrix {
    pub fn add_row(&mut self) {
        self.data.push(vec!["0".to_string(); self.cols]);
        self.rows += 1;
    }

    pub fn remove_row(&mut self) {
        if self.rows > 0 {
            self.data.pop();
            self.rows -= 1;
        }
    }

    pub fn add_col(&mut self) {
        for row in &mut self.data {
            row.push("0".to_string());
        }
        self.cols += 1;
    }

    pub fn remove_col(&mut self) {
        if self.cols > 0 {
            for row in &mut self.data {
                row.pop();
            }
            self.cols -= 1;
        }
    }
}
