use std::{path::PathBuf, sync::LazyLock};

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| "wubi_nc.dict.yaml".into());
