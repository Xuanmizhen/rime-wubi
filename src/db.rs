use std::{path::PathBuf, sync::LazyLock};
use thiserror::Error;

pub mod artificial_intelligence_terminology_database;
pub mod rime_data;
pub mod unicode_cjk_wubi06;

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| "db".into());

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("failed to read from database")]
    RimeData(#[from] rime_data::Error),
}
