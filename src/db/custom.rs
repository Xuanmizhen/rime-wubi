use crate::{db::unicode_cjk_wubi06::cjk::Table, rime::Dict};
use log::info;
use std::{
    fs,
    io::{self, BufRead as _},
    path::{Path, PathBuf},
    sync::LazyLock,
};
use thiserror::Error;

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::PATH.join("custom.txt"));

pub fn update_dict(dict: &mut Dict, table: &Table, words: String) {
    let mut start = 0;
    let mut end = 0;
    for (index, ch) in words.char_indices() {
        if table.contains_char(ch) {
            let len = ch.len_utf8();
            if end == index {
                end += len;
            } else {
                dict.insert_phrase_if_missing(words[start..end].to_string(), table, Some(u32::MAX));
                (start, end) = (index, index + len);
            }
        }
    }
    dict.insert_phrase_if_missing(words[start..end].to_string(), table, Some(u32::MAX));
}

pub fn update_dict_from_path<P: AsRef<Path>>(
    dict: &mut Dict,
    table: &Table,
    path: P,
) -> Result<()> {
    info!("Updating dict from path {}", path.as_ref().display());
    let rdr = io::BufReader::new(fs::File::open(path)?);
    for line in rdr.lines() {
        update_dict(dict, table, line?);
    }
    Ok(())
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("cannot read Chinese phrases file: {}", PATH.display())]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
