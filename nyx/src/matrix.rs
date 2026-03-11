use crate::error::{NyxError, NyxResult};

/// Generic matrix: 2D container for homogeneous data
#[derive(Clone, Debug)]
pub struct Matrix<T: Clone> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Clone> Matrix<T> {
    /// Create a new matrix from a flat vector, dimensions given
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> NyxResult<Self> {
        if data.len() != rows * cols {
            return Err(NyxError::InvalidInput(
                format!("Matrix dimensions mismatch: expected {}, got {}", rows * cols, data.len())
            ));
        }
        Ok(Matrix { data, rows, cols })
    }

    /// Create a matrix filled with a default value
    pub fn filled(value: T, rows: usize, cols: usize) -> Self {
        let data = vec![value; rows * cols];
        Matrix { data, rows, cols }
    }

    /// Get reference to element at (row, col)
    pub fn get(&self, row: usize, col: usize) -> NyxResult<&T> {
        if row >= self.rows || col >= self.cols {
            return Err(NyxError::InvalidInput(
                format!("Index out of bounds: ({}, {}) for matrix size {}x{}", row, col, self.rows, self.cols)
            ));
        }
        Ok(&self.data[row * self.cols + col])
    }

    /// Get mutable reference to element at (row, col)
    pub fn get_mut(&mut self, row: usize, col: usize) -> NyxResult<&mut T> {
        if row >= self.rows || col >= self.cols {
            return Err(NyxError::InvalidInput(
                format!("Index out of bounds: ({}, {}) for matrix size {}x{}", row, col, self.rows, self.cols)
            ));
        }
        Ok(&mut self.data[row * self.cols + col])
    }

    /// Set element at (row, col)
    pub fn set(&mut self, row: usize, col: usize, value: T) -> NyxResult<()> {
        *self.get_mut(row, col)? = value;
        Ok(())
    }

    /// Get matrix dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Get number of rows
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get number of columns
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get raw data as slice
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Get raw data as mutable slice
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Transpose the matrix
    pub fn transpose(&self) -> Self
    where
        T: Copy,
    {
        let mut transposed = vec![self.data[0].clone(); self.data.len()];
        for (i, &val) in self.data.iter().enumerate() {
            let row = i / self.cols;
            let col = i % self.cols;
            transposed[col * self.rows + row] = val;
        }
        Matrix {
            data: transposed,
            rows: self.cols,
            cols: self.rows,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let data = vec![1, 2, 3, 4];
        let m = Matrix::new(data, 2, 2).unwrap();
        assert_eq!(m.rows(), 2);
        assert_eq!(m.cols(), 2);
    }

    #[test]
    fn test_matrix_get() {
        let data = vec![1, 2, 3, 4];
        let m = Matrix::new(data, 2, 2).unwrap();
        assert_eq!(*m.get(0, 0).unwrap(), 1);
        assert_eq!(*m.get(1, 1).unwrap(), 4);
    }

    #[test]
    fn test_matrix_filled() {
        let m = Matrix::filled(0.0, 3, 3);
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 3);
    }

    #[test]
    fn test_matrix_transpose() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let m = Matrix::new(data, 2, 3).unwrap();
        let t = m.transpose();
        assert_eq!(t.rows(), 3);
        assert_eq!(t.cols(), 2);
        assert_eq!(*t.get(0, 0).unwrap(), 1);
        assert_eq!(*t.get(1, 0).unwrap(), 2);
    }
}
