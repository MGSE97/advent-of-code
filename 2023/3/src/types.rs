use std::{cell::RefCell, fmt::Display, rc::Rc};

use itertools::Itertools;

#[derive(Debug, Default)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn get(&self, row: i32, col: i32) -> Option<&T> {
        if row < 0 || col < 0 {
            None
        } else {
            self.data
                .get(row as usize)
                .and_then(|r| r.get(col as usize))
        }
    }
    pub fn add(&mut self, row: Vec<T>) {
        self.data.push(row);
    }
    pub fn size(&self) -> (usize, usize) {
        (
            self.data.len(),
            self.data
                .iter()
                .map(|row| row.len())
                .max()
                .unwrap_or_default(),
        )
    }
}

#[derive(Debug)]

pub enum Part {
    Number(Rc<RefCell<u32>>),
    Symbol,
    None,
}

impl Default for Part {
    fn default() -> Self {
        Self::None
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            Part::Number(_) => "N",
            Part::Symbol => "+",
            Part::None => " ",
        };
        write!(f, "{fmt}")
    }
}

impl Display for Matrix<Part> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self
            .data
            .iter()
            .map(|row| row.iter().map(|i| i.to_string()).join(""))
            .join("\n");
        write!(f, "{data}")
    }
}
