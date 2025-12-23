use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Result<Self, &'static str> {
        let rows = data.len();
        let cols = data.get(0).map_or(0, |r| r.len());

        if data.iter().any(|r| r.len() != cols) {
            return Err("matrix is not rectangular");
        }

        Ok(Self { rows, cols, data })
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?; // new line to make it look good

        if self.rows == 0 || self.cols == 0 {
            return writeln!(f, "[]");
        }

        /* 1. Convert all numbers to strings */
        let strings: Vec<Vec<String>> = self
            .data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| {
                        format!("{}", v)
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_string()
                    })
                    .collect()
            })
            .collect();

        /* 2. Compute max width of each column */

        let mut col_widths = vec![0; self.cols];
        for row in &strings {
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
                write!(f, "{:>width$} ", strings[i][j], width = col_widths[j])?;
            }
            writeln!(f, "{right}")?;
        }

        Ok(())
    }
}
