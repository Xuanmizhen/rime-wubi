use std::{path::PathBuf, sync::LazyLock};

pub static PATH: LazyLock<PathBuf> =
    LazyLock::new(|| super::PATH.join("Artificial-Intelligence-Terminology-Database"));

pub static DATA_PATH: LazyLock<PathBuf> = LazyLock::new(|| PATH.join("data").join("All.md"));
