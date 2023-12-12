// Types and impls for both parts
use std::{fmt::Display, mem::replace, ops::Deref, str::FromStr};

use itertools::Itertools;
use lib::{impl_parse, Colorize};

#[derive(Debug, Clone)]
pub struct Input {
    pub tiles: Matrix2D<TileData>,
}

impl_parse! {
    Token {
        (Tile+ EOL?)+
    }
    without r"[ ]+"
    as {
        Tile Tile r"[|\-LJ7F\.S]"
        EOL r"[\s+]"
    }
}

impl FromStr for Input {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rows = Token::parse(input)?;
        Ok(Self {
            tiles: rows
                .into_iter()
                .map(|(row, _)| {
                    row.into_iter()
                        .filter_map(Token::into_tile)
                        .map(|tile| tile.into())
                        .collect_vec()
                })
                .collect_vec()
                .into(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Tile {
    PipeNtS,
    PipeEtW,
    PipeNtE,
    PipeNtW,
    PipeStW,
    PipeStE,
    #[default]
    Ground,
    Start,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::PipeNtS => "║",
                Self::PipeEtW => "═",
                Self::PipeNtE => "╚",
                Self::PipeNtW => "╝",
                Self::PipeStW => "╗",
                Self::PipeStE => "╔",
                Self::Ground => ".",
                Self::Start => "╬",
            }
        )
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(tile: &str) -> Result<Self, Self::Err> {
        Ok(match tile.chars().next().unwrap() {
            '|' => Self::PipeNtS,
            '-' => Self::PipeEtW,
            'L' => Self::PipeNtE,
            'J' => Self::PipeNtW,
            '7' => Self::PipeStW,
            'F' => Self::PipeStE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => Err("Wrong tile!")?,
        })
    }
}

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
                .map(|_| (0..cols).map(|_| value.clone()).collect_vec())
                .collect_vec(),
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

    pub(crate) fn size(&self) -> (usize, usize) {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TileData {
    pub tile: Tile,
    pub distance: Option<usize>,
    pub previous: Option<(usize, usize)>,
    pub end: bool,
    pub path: bool,
}

impl Display for TileData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.tile.to_string();
        write!(
            f,
            "{}",
            match (
                self.tile == Tile::Start,
                self.distance.is_some(),
                self.end,
                self.path
            ) {
                (true, _, _, _) => val.yellow().bold().underline(),
                (_, _, true, _) => val.red().bold().underline(),
                (_, _, _, true) => val.green().bold(),
                (_, true, _, _) => val.cyan(),
                _ => val.blue(),
            }
        )
    }
}

impl From<Tile> for TileData {
    fn from(value: Tile) -> Self {
        Self {
            tile: value,
            distance: None,
            previous: None,
            end: false,
            path: false,
        }
    }
}
