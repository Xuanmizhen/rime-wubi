use log::info;
use std::{
    collections::{BTreeMap, HashSet},
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    result,
    sync::LazyLock,
};
use thiserror::Error;

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::PATH.join("CJK.txt"));

pub struct Table {
    code_to_char: BTreeMap<String, HashSet<char>>,
    pub(crate) char_to_code: BTreeMap<char, String>,
}

impl Table {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("Loading {}", path.as_ref().display());
        let rdr = io::BufReader::new(fs::File::open(path)?);
        Self::try_from_iter(rdr.lines().map(|line| Ok(line?)))
    }

    pub fn insert(&mut self, ch: char, code: String) {
        self.code_to_char
            .entry(code.clone())
            .and_modify(|chs| {
                chs.insert(ch);
            })
            .or_insert_with(|| [ch].into_iter().collect());
        self.char_to_code.insert(ch, code);
    }

    pub fn contains_char(&self, ch: char) -> bool {
        self.char_to_code.contains_key(&ch)
    }

    pub fn get_phrase_code(&self, phrase: &str) -> String {
        let mut chars = phrase.chars();
        let first = chars.next().unwrap();
        let first = self
            .char_to_code
            .get(&first)
            .unwrap_or_else(|| {
                dbg!(&first);
                panic!()
            })
            .as_bytes();
        match chars.next() {
            Some(second) => {
                let second = self.char_to_code[&second].as_bytes();
                match chars.next() {
                    Some(third) => {
                        let third = self.char_to_code[&third].as_bytes();
                        match chars.last() {
                            Some(last) => {
                                let last = self.char_to_code[&last].as_bytes();
                                format!(
                                    "{}{}{}{}",
                                    first[0] as char,
                                    second[0] as char,
                                    third[0] as char,
                                    last[0] as char
                                )
                            }
                            None => format!(
                                "{}{}{}{}",
                                first[0] as char,
                                second[0] as char,
                                third[0] as char,
                                third[1] as char
                            ),
                        }
                    }
                    None => format!(
                        "{}{}{}{}",
                        first[0] as char, first[1] as char, second[0] as char, second[1] as char
                    ),
                }
            }
            None => first.iter().map(|byte| *byte as char).collect(),
        }
    }
}

impl FromIterator<(char, String)> for Table {
    fn from_iter<T: IntoIterator<Item = (char, String)>>(iter: T) -> Self {
        let mut code_to_char: BTreeMap<String, HashSet<char>> = Default::default();
        let char_to_code = iter
            .into_iter()
            .map(|(ch, code)| {
                code_to_char
                    .entry(code.clone())
                    .and_modify(|chs| {
                        chs.insert(ch);
                    })
                    .or_insert_with(|| [ch].into_iter().collect());
                (ch, code)
            })
            .collect();
        Self {
            code_to_char,
            char_to_code,
        }
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
                    Ok((ch, code))
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
