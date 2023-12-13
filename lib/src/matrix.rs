use std::{mem::replace, ops::Deref};

#[derive(Debug, Clone)]
pub struct Matrix2D<T> {
    rows: Vec<Vec<T>>,
}

impl<T> Matrix2D<T> {
    pub fn new(cols: usize, rows: usize) -> Self
    where
        T: Default + Clone,
    {
        Self::fill(cols, rows, T::default())
    }

    pub fn fill(cols: usize, rows: usize, value: T) -> Self
    where
        T: Clone,
    {
        Self {
            rows: (0..rows)
                .map(|_| (0..cols).map(|_| value.clone()).collect())
                .collect(),
        }
    }

    pub fn get(&self, col: usize, row: usize) -> Result<&T, String> {
        self.rows
            .get(row)
            .ok_or(format!("Row {row} not found!"))?
            .get(col)
            .ok_or(format!("Col {col} not found!"))
    }

    pub fn get_mut(&mut self, col: usize, row: usize) -> Result<&mut T, String> {
        self.rows
            .get_mut(row)
            .ok_or(format!("Row {row} not found!"))?
            .get_mut(col)
            .ok_or(format!("Col {col} not found!"))
    }

    pub fn set(&mut self, col: usize, row: usize, value: T) -> Result<&Self, String> {
        let _ = replace(
            self.rows
                .get_mut(row)
                .ok_or(format!("Row {row} not found!"))?
                .get_mut(col)
                .ok_or(format!("Col {col} not found!"))?,
            value,
        );
        Ok(self)
    }

    pub fn size(&self) -> (usize, usize) {
        (
            self.rows.first().map(|r| r.len()).unwrap_or_default(),
            self.rows.len(),
        )
    }
}

impl<T> From<Vec<Vec<T>>> for Matrix2D<T> {
    fn from(rows: Vec<Vec<T>>) -> Self {
        Self { rows }
    }
}

impl<T> Deref for Matrix2D<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.rows
    }
}
