use crate::rime::Dict;
use std::{path::PathBuf, sync::LazyLock};
use thiserror::Error;
use unicode_cjk_wubi06::cjk::Table;

pub mod artificial_intelligence_terminology_database;
pub mod rime_data;
pub mod thuocl;
pub mod unicode_cjk_wubi06;

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| "db".into());

pub fn verify_with(dict: &mut Dict, table: &Table) -> bool {
    todo!()
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("failed to read from rime_data")]
    RimeData(#[from] rime_data::Error),
    #[error("failed to read from unicode_cjk_wubi06")]
    UnicodeCjkWubi06(#[from] unicode_cjk_wubi06::Error),
}
