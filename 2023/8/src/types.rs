// Types and impls for both parts
use std::{collections::BTreeMap, str::FromStr};

use itertools::Itertools;
use lib::impl_parse;

#[derive(Debug, Clone)]
pub struct Input {
    pub instructions: Vec<Instruction>,
    pub nodes: BTreeMap<String, (String, String)>,
}

impl_parse! {
    Token {
        Instructions

        (Node = (Node, Node))+
    }
    without r"[,=\(\)\s]+"
    as {
        Instructions String "[lLrR]+"
        Node String "[a-zA-Z0-9]{3}"
    }
}

impl FromStr for Input {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (instructions, rows) = Token::parse(input)?;
        Ok(Self {
            instructions: get_inscructions(instructions).unwrap_or_default(),
            nodes: BTreeMap::from_iter(
                rows.into_iter().map(|(node, (left, right))| {
                    (get_node(node), (get_node(left), get_node(right)))
                }),
            ),
        })
    }
}

fn get_node(token: Token) -> String {
    match token {
        Token::Node(val) => Some(val),
        _ => None,
    }
    .unwrap_or_default()
}

fn get_inscructions(token: Token) -> Option<Vec<Instruction>> {
    match token {
        Token::Instructions(val) => Some(val.chars().map(Instruction::from).collect_vec()),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'l' | 'L' => Instruction::Left,
            'r' | 'R' => Instruction::Right,
            _ => panic!("Wrong Instruction!"),
        }
    }
}

impl Input {
    pub fn distance(&self, from: &str, to: &str) -> usize {
        let mut current = &from.to_string();
        let search = &to.to_string();
        let mut instruction = self.instructions.iter().cycle();
        let mut steps = 0;
        while current != search {
            let (left, right) = self.nodes.get(current).unwrap();
            match instruction.next().unwrap() {
                Instruction::Left => current = left,
                Instruction::Right => current = right,
            }
            steps += 1;
        }
        steps
    }

    pub fn shadow_clones_distance(&self, from: char, to: char) -> usize {
        self.nodes
            .keys()
            .filter(|k| k.ends_with(from))
            .map(|node| {
                let mut current = node;
                let mut instruction = self.instructions.iter().cycle();
                let mut steps = 0;
                while !current.ends_with(to) {
                    let (left, right) = self.nodes.get(current).unwrap();
                    current = match instruction.next().unwrap() {
                        Instruction::Left => left,
                        Instruction::Right => right,
                    };
                    steps += 1;
                }
                println!("{steps}");
                steps
            })
            // Least Common Multiple
            .fold(1, |agg, steps| (agg * steps) / gdc(agg, steps))
    }
}

/// Greatest Common Divisor
fn gdc(a: usize, b: usize) -> usize {
    match a % b == 0 {
        true => b,
        false => gdc(b, a % b),
    }
}
