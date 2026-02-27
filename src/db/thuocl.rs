use crate::{db::unicode_cjk_wubi06::cjk::Table, rime::Dict};
use log::info;
use std::{
    fs,
    io::{self, BufRead as _},
    num::ParseIntError,
    path::{Path, PathBuf},
    sync::LazyLock,
};
use thiserror::Error;

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::PATH.join("THUOCL"));

pub static DATA_PATH: LazyLock<PathBuf> = LazyLock::new(|| PATH.join("data"));
pub static DATA_IT_PATH: LazyLock<PathBuf> = LazyLock::new(|| DATA_PATH.join("THUOCL_IT.txt"));
pub static DATA_CHENGYU_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| DATA_PATH.join("THUOCL_chengyu.txt"));
pub static DATA_LISHIMINGREN_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| DATA_PATH.join("THUOCL_lishimingren.txt"));
pub static DATA_DIMING_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| DATA_PATH.join("THUOCL_diming.txt"));
pub static DATA_POEM_PATH: LazyLock<PathBuf> = LazyLock::new(|| DATA_PATH.join("THUOCL_poem.txt"));

pub static RECOMMENDED_DATA_PATHS: LazyLock<Vec<&'static LazyLock<PathBuf>>> =
    LazyLock::new(|| {
        vec![
            &DATA_IT_PATH,
            &DATA_CHENGYU_PATH,
            &DATA_LISHIMINGREN_PATH,
            &DATA_DIMING_PATH,
            &DATA_POEM_PATH,
        ]
    });

pub fn update_dict(dict: &mut Dict, table: &Table, line: String) -> Result<()> {
    let mut line = line.split_whitespace();
    let phrase = line.next().unwrap().to_string();
    for ch in phrase.chars() {
        if !table.contains_char(ch) {
            return Ok(());
        }
    }
    let weight = if let Some(w) = line.next() {
        Some(w.parse().map_err(Error::ParseWeight)?)
    } else {
        None
    };
    dict.insert_phrase_if_missing(phrase, table, weight);
    Ok(())
}

pub fn update_dict_from_path<P: AsRef<Path>>(
    dict: &mut Dict,
    table: &Table,
    path: P,
) -> Result<()> {
    info!("Updating dict from path {}", path.as_ref().display());
    let rdr = io::BufReader::new(fs::File::open(path)?);
    for line in rdr.lines() {
        update_dict(dict, table, line?)?;
    }
    Ok(())
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("cannot read Chinese phrases file: {}", PATH.display())]
    Io(#[from] std::io::Error),
    #[error("cannot parse weight")]
    ParseWeight(#[source] ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;
