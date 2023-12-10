use std::{
    collections::BTreeMap,
    fmt::Debug,
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

pub trait FilesExt {
    fn parse_file<T>(&self, name: &str) -> Result<T, String>
    where
        T: FromStr,
        T::Err: Debug;
}

impl FilesExt for BTreeMap<String, String> {
    fn parse_file<T>(&self, name: &str) -> Result<T, String>
    where
        T: FromStr,
        T::Err: Debug,
    {
        parse_file::<T>(
            self.get(name)
                .ok_or_else(|| format!("There is no file named {name}!"))?,
        )
    }
}

pub fn parse_file<T>(file: &str) -> Result<T, String>
where
    T: FromStr,
    T::Err: Debug,
{
    // Open file reader
    let file = File::open(file).map_err(|err| format!("Failed to open input file!\n{err}"))?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader
        .read_to_string(&mut data)
        .expect("Input file read failed!");
    Ok(data.parse().expect("Failed to parse input!"))
}
