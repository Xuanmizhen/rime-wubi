use log::info;
use std::{
    collections::BTreeMap,
    fs,
    io::{self, BufRead},
    ops::Index,
    path::{Path, PathBuf},
    result,
    sync::LazyLock,
};
use thiserror::Error;

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::PATH.join("CJK.txt"));

pub struct Table {
    code_to_char: BTreeMap<String, char>,
}

impl Table {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("Loading {}", path.as_ref().display());
        let rdr = io::BufReader::new(fs::File::open(path)?);
        Self::try_from_iter(rdr.lines().map(|line| Ok(line?)))
    }
}

impl Index<&String> for Table {
    type Output = char;

    fn index(&self, index: &String) -> &Self::Output {
        &self.code_to_char[index]
    }
}

impl FromIterator<(String, char)> for Table {
    fn from_iter<T: IntoIterator<Item = (String, char)>>(iter: T) -> Self {
        let code_to_char = iter.into_iter().collect();
        Self { code_to_char }
    }
}

impl Table {
    fn try_from_iter<T: IntoIterator<Item = Result<String>>>(iter: T) -> Result<Self> {
        iter.into_iter()
            .map(|s| {
                let mut s = s?;
                if s.len() <= 7 {
                    return Err(ParseTableError {
                        kind: ErrorKind::TooShort,
                        parsing: s,
                    }
                    .into());
                }
                let mut ch = s.split_off(7);
                let code = ch.split_off(ch.ceil_char_boundary(1));
                let ch = ch.chars().next().expect("len of s should exceeds 7");
                let mut code = code.chars();
                if let Some('\t') = code.next() {
                    let code = code.collect();
                    Ok((code, ch))
                } else {
                    Err(ParseTableError {
                        kind: ErrorKind::Format,
                        parsing: s,
                    }
                    .into())
                }
            })
            .collect()
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to parse table")]
    ParseTable(#[from] ParseTableError),
    #[error("io error")]
    Io(#[from] io::Error),
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Error)]
#[error("{}: {parsing}", match kind {
    ErrorKind::Format => "invalid format",
    ErrorKind::TooShort => "too short string to parse",
})]
pub struct ParseTableError {
    kind: ErrorKind,
    parsing: String,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    Format,
    TooShort,
}
