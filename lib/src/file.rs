use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read},
};

use prse::Parse;

pub trait FilesExt {
    fn parse_file<T>(&self, name: &str) -> Result<T, String>
    where
        T: for<'a> Parse<'a>;
}

impl FilesExt for BTreeMap<String, String> {
    fn parse_file<T>(&self, name: &str) -> Result<T, String>
    where
        T: for<'a> Parse<'a>,
    {
        parse_file::<T>(
            self.get(name)
                .ok_or_else(|| format!("There is no file named {name}!"))?,
        )
    }
}

pub fn parse_file<T>(file: &str) -> Result<T, String>
where
    T: for<'a> Parse<'a>,
{
    // Open file reader
    let file = File::open(file).map_err(|err| format!("Failed to open input file!\n{err}"))?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader
        .read_to_string(&mut data)
        .expect("Input file read failed!");
    Ok(T::from_str(data.as_str()).expect("Failed to parse input!"))
}
