use crate::rime::Dict;
use log::{info, trace, warn};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::LazyLock,
};
use thiserror::Error;
use unicode_cjk_wubi06::cjk::Table;

pub mod artificial_intelligence_terminology_database;
pub mod custom;
pub mod rime_data;
pub mod thuocl;
pub mod unicode_cjk_wubi06;

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| "db".into());

pub fn verify_with(dict: &mut Dict, table: &mut Table) -> bool {
    info!("Verifying characters");
    let mut matched_chs = HashSet::new();
    let mut incomplete_chs = HashSet::new();
    let mut not_found_chs = HashMap::new();
    for entry in &dict.chars {
        let ch = entry.phrase.chars().next().unwrap();
        if let Some(code) = table.char_to_code.get(&ch) {
            if entry.code != *code {
                if entry.code.len() >= code.len() {
                    warn!(
                        "contradictory code for character {ch}: {} and {code}",
                        entry.code
                    );
                    return false;
                }
                if !matched_chs.contains(&ch) {
                    incomplete_chs.insert(ch);
                }
            } else {
                incomplete_chs.remove(&ch);
                matched_chs.insert(ch);
            }
        } else {
            trace!("character {ch} not found in table");
            if not_found_chs.insert(ch, &entry.code).is_some() {
                warn!("same not-in-table character appearing multiple times not allowed");
                return false;
            }
        }
    }
    let len = incomplete_chs.len();
    if len != 0 {
        warn!("{} incomplete characters", len);
    }
    for (ch, code) in not_found_chs {
        table.insert(ch, code.clone());
    }
    dict.chars.reserve(incomplete_chs.len());
    for ch in incomplete_chs {
        let code = table.char_to_code[&ch].clone();
        dict.insert_char(ch, code);
    }

    for (ch, code) in &table.char_to_code {
        if !matched_chs.contains(ch) {
            trace!("{ch} ({code}) not in dict, putting in");
            dict.insert_char(*ch, code.clone());
        }
    }

    info!("Verifying phrases");
    for entry in dict.phrases() {
        if entry.code != table.get_phrase_code(&entry.phrase) {
            return false;
        }
    }

    true
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("failed to read from rime_data")]
    RimeData(#[from] rime_data::Error),
    #[error("failed to read from unicode_cjk_wubi06")]
    UnicodeCjkWubi06(#[from] unicode_cjk_wubi06::Error),
}
