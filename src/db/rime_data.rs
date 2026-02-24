use crate::rime_yaml;
use log::{debug, info};
use std::{
    fs,
    io::{self, BufRead},
    num,
    path::{Path, PathBuf},
    result,
    sync::LazyLock,
};
use thiserror::Error;

mod display;
pub mod wubi06_tygfhzb_cjbcq;
pub mod wubi06_tygfhzb_dz;

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::PATH.join("rime-data"));

#[derive(Debug)]
pub struct RimeDict {
    data: Vec<RimeDictEntry>,
}

impl RimeDict {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("Loading Rime dict from {}", path.as_ref().display());
        let mut rdr = io::BufReader::new(fs::File::open(path)?);
        rime_yaml::skip_until_dict_data(&mut rdr)?.ok_or(Error::Parse(ParseError::Syntax))?;
        let mut data = Vec::new();
        for line in rdr.lines() {
            if let Some(entry) = RimeDictEntry::build(line?)? {
                data.push(entry);
            }
        }
        let dict = RimeDict { data };
        debug!("Loaded {dict}");
        Ok(dict)
    }
}

#[derive(Debug)]
struct RimeDictEntry {
    phrase: String,
    code: String,
    weight: Option<u32>,
}

impl RimeDictEntry {
    fn build(value: String) -> result::Result<Option<Self>, ParseError> {
        let value = value.trim();
        if value.is_empty() || value.starts_with('#') {
            return Ok(None);
        }
        let mut items = value.split('\t');
        let phrase = items.next().ok_or(ParseError::Syntax)?.to_string();
        let code = items.next().ok_or(ParseError::Syntax)?.to_string();
        let weight = if let Some(weight) = items.next() {
            Some(weight.parse().map_err(ParseError::ParseWeight)?)
        } else {
            None
        };
        Ok(Some(Self {
            phrase,
            code,
            weight,
        }))
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] io::Error),
    #[error("parse error")]
    Parse(#[from] ParseError),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ParseError {
    #[error("invaid syntax")]
    Syntax,
    #[error("invalid weight value")]
    ParseWeight(#[source] num::ParseIntError),
}

pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_reports_overflow() {
        assert!(u32::MAX.to_string().parse::<u32>().is_ok());
        assert!((u32::MAX as u64 + 1).to_string().parse::<u32>().is_err());
    }
}
