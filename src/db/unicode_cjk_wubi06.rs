use std::{path::PathBuf, sync::LazyLock};

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::PATH.join("UnicodeCJK-WuBi06"));
