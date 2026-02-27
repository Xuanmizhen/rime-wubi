use std::{path::PathBuf, sync::LazyLock};

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| super::PATH.join("lexicon"));

pub static IT_DATA_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PATH.join("专业性词汇").join("IT.json"));

pub static MATH_DATA_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PATH.join("专业性词汇").join("数学专业词汇.json"));
