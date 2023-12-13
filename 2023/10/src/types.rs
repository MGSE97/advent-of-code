// Types and impls for both parts
use std::{fmt::Display, str::FromStr};

use itertools::Itertools;
use lib::{impl_parse, Colorize, Matrix2D};

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
                Self::Ground => "░",
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
        write!(
            f,
            "{}",
            match self.tile.to_string() {
                val if self.tile == Tile::Start => val.yellow().bold(),
                val if self.end => val.red().bold(),
                val if self.path => val.green().bold(),
                val if self.distance.is_some() => val.cyan(),
                val => val.blue(),
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
