pub use cjk::{Error, Result};
use std::{path::PathBuf, sync::LazyLock};

pub mod cjk;

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::PATH.join("UnicodeCJK-WuBi06"));
