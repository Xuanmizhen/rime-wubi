use std::{path::PathBuf, sync::LazyLock};

pub mod artificial_intelligence_terminology_database;
pub mod rime_data;
pub mod unicode_cjk_wubi06;

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| "db".into());
