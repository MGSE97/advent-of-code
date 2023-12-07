use colored::Colorize;
use std::fmt::{Debug, Display};

pub struct Answer {
    pub value: String,
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.yellow().bold().underline())
    }
}

impl<T> From<T> for Answer
where
    T: Display + Debug,
{
    fn from(value: T) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}
