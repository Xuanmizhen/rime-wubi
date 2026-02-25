use super::{Dict, Result};
use std::{path::PathBuf, sync::LazyLock};

pub static DICT_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| super::PATH.join("wubi06_tygfhzb_cjbcq.dict.yaml"));

pub fn load_dict() -> Result<Dict> {
    Dict::load(&*DICT_PATH)
}

pub const SINGLE_CHAR_WEIGHT: u32 = 999999999;

pub fn is_valid(dict: &Dict) -> bool {
    dict.contains_both()
        && !dict.entries().any(|entry| {
            entry.phrase.chars().count() == 1 && entry.weight != Some(SINGLE_CHAR_WEIGHT)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_is_valid() {
        let dict = load_dict().unwrap();
        assert!(is_valid(&dict));
    }
}
