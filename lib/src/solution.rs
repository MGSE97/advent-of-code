use colored::Colorize;
use std::{collections::BTreeMap, env, fmt::Display};

pub struct Solution {
    pub files: BTreeMap<String, String>,
    pub question: String,
    pub answer: String,
}

const QUESTION: &str = "Question:";
const ANSWER: &str = "Answer:";
const FILE: &str = "file:";

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pad_to = [QUESTION, ANSWER]
            .iter()
            .map(|i| i.len())
            .chain(self.files.keys().map(|name| name.len() + FILE.len() + 1))
            .max()
            .unwrap_or_default();

        let width = [self.question.len(), self.answer.len()]
            .into_iter()
            .chain(self.files.values().map(|path| path.len()))
            .max()
            .unwrap_or_default()
            + 1
            + pad_to;

        // Separator
        if !self.files.is_empty() {
            writeln!(
                f,
                "{}",
                format!("{line:―>width$}", width = width, line = "",).bright_black()
            )?;
        }

        // Files
        for (name, path) in self.files.iter() {
            writeln!(
                f,
                "{name: >pad$} {file} {path}",
                pad = pad_to - FILE.len() - 1,
                file = FILE.blue(),
                name = name.blue(),
                path = path.yellow()
            )?;
        }

        // Separator
        if !self.files.is_empty() {
            writeln!(
                f,
                "{}",
                format!("{line:―>width$}", width = width, line = "",).bright_black()
            )?;
        }

        // Question
        writeln!(
            f,
            "{prefix: >pad$} {question}",
            pad = pad_to,
            prefix = QUESTION.blue(),
            question = self.question.cyan().bold()
        )?;

        // Answer
        writeln!(
            f,
            "{prefix: >pad$} {answer}",
            pad = pad_to,
            prefix = ANSWER.blue(),
            answer = self.answer.green().bold()
        )
    }
}

pub fn parse_args_solution() -> Result<usize, String> {
    Ok(env::args()
        .filter_map(|arg| arg.parse::<usize>().ok())
        .next()
        .ok_or_else(|| {
            println!(
                "\n{err}\n{help}\n",
                err = "Specify part to solve!".red().bold(),
                help = "> cargo run -- solve part 1".yellow()
            );
            "No part selected!"
        })?)
}
