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

/// Waits for a specific line in the given iterator of lines.
///
/// Returns `Ok(())` if the target line is found, or an error if the end of the iterator is reached without finding the target.
fn wait_for(lines: &mut impl Iterator<Item = io::Result<String>>, target: &str) -> Result<()> {
    for line in lines {
        if line? == target {
            return Ok(());
        }
    }
    Err(Error::Parse(ParseError::Syntax))
}

impl RimeDict {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("Loading Rime dict from {}", path.as_ref().display());
        let mut lines = io::BufReader::new(fs::File::open(path)?).lines();
        wait_for(&mut lines, "---")?;
        wait_for(&mut lines, "...")?;
        let mut data = Vec::new();
        for line in lines {
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
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("parse error: {0}")]
    Parse(#[from] ParseError),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ParseError {
    #[error("invaid syntax")]
    Syntax,
    #[error("invalid weight value: {0}")]
    ParseWeight(num::ParseIntError),
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
