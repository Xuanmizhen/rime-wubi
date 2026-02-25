use crate::rime::*;
use log::info;
use std::cmp::Ordering::*;
use std::{
    fs,
    io::{self, BufRead},
    num,
    path::{Path, PathBuf},
    result,
    str::FromStr,
    sync::LazyLock,
};
use thiserror::Error;

mod output;
pub mod wubi06_tygfhzb_cjbcq;
pub mod wubi06_tygfhzb_dz;

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::PATH.join("rime-data"));

pub fn load_and_merge_dicts() -> Result<Dict> {
    let (chars_ref, phrases) = wubi06_tygfhzb_cjbcq::load_dict()?.into_raw_parts();
    let (mut chars, empty_phrases) = wubi06_tygfhzb_dz::load_dict()?.into_raw_parts();
    if !empty_phrases.is_empty() {
        return Err(Error::DzDictContainsPhrases);
    }
    let mut i = 0;
    for mut ch_ref in chars_ref {
        if ch_ref.weight != Some(wubi06_tygfhzb_cjbcq::SINGLE_CHAR_WEIGHT) {
            return Err(Error::UnexpectedWeight(ch_ref));
        }
        while let Some(ch) = chars.get_mut(i) {
            match ch.code.cmp(&ch_ref.code) {
                Less => i += 1,
                Equal => {
                    if ch.phrase != ch_ref.phrase {
                        if ch.code.len() == 4 {
                            let phrase = ch.phrase.clone();
                            let ch = ch.clone();
                            if chars.iter().all(|entry| entry.phrase != phrase) {
                                return Err(Error::Contradict(ch, ch_ref));
                            }
                        } else {
                            ch.weight = Some(u32::MAX);
                            ch_ref.weight = None;
                            i += 1;
                            chars.insert(i, ch_ref);
                        }
                    } else {
                        i += 1;
                    }
                    break;
                }
                Greater => {
                    let (code, phrase) = (ch_ref.code.clone(), ch_ref.phrase.clone());
                    let weight = None;
                    chars.insert(
                        i,
                        DictEntry {
                            code,
                            weight,
                            phrase,
                        },
                    );
                    break;
                }
            }
        }
    }
    chars.sort(); // In fact, bubble sort is more efficient here since the list is almost sorted, but it's not a big deal.
    Ok(Dict::new(chars, phrases))
}

impl Dict {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("Loading Rime dict from {}", path.as_ref().display());
        let mut rdr = io::BufReader::new(fs::File::open(path)?);
        yaml::skip_until_dict_data(&mut rdr)?.ok_or(Error::Parse(ParseDictEntryError::Syntax))?;
        let dict = rdr
            .lines()
            .filter_map(|line| match line {
                Ok(line) => match DictEntry::from_line(&line) {
                    Ok(Some(entry)) => Some(Ok(entry)),
                    Ok(None) => None,
                    Err(e) => Some(Err(Error::Parse(e))),
                },
                Err(e) => Some(Err(e.into())),
            })
            .collect::<Result<_>>()?;
        Ok(dict)
    }
}

impl DictEntry {
    fn from_line(value: &str) -> result::Result<Option<Self>, ParseDictEntryError> {
        let value = value.trim();
        if value.is_empty() || value.starts_with('#') {
            return Ok(None);
        }
        Ok(Some(value.parse()?))
    }
}

impl FromStr for DictEntry {
    type Err = ParseDictEntryError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let mut items = s.split('\t');
        let phrase = items.next().ok_or(ParseDictEntryError::Syntax)?.to_string();
        let code = items.next().ok_or(ParseDictEntryError::Syntax)?.to_string();
        if code
            .bytes()
            .any(|byte| !byte.is_ascii_lowercase() || byte == b'z')
            || code.len() > 4
        {
            return Err(ParseDictEntryError::Code(code));
        }
        let weight = if let Some(weight) = items.next() {
            Some(weight.parse().map_err(ParseDictEntryError::ParseWeight)?)
        } else {
            None
        };
        Ok(Self {
            phrase,
            code,
            weight,
        })
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("IO error")]
    Io(#[from] io::Error),
    #[error("parse error")]
    Parse(#[from] ParseDictEntryError),
    #[error("dz dict contains phrases")]
    DzDictContainsPhrases,
    #[error("unexpected weight value in entry: {0}")]
    UnexpectedWeight(DictEntry),
    #[error("contradict entry: {0} and {1}")]
    Contradict(DictEntry, DictEntry),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ParseDictEntryError {
    #[error("invaid syntax")]
    Syntax,
    #[error("invalid code: {0}")]
    Code(String),
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
