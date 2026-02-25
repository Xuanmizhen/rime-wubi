use super::{Dict, Result};
use std::{path::PathBuf, sync::LazyLock};

pub static DICT_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| super::PATH.join("wubi06_tygfhzb_dz.dict.yaml"));

pub fn load_dict() -> Result<Dict> {
    Dict::load(&*DICT_PATH)
}

pub fn is_valid(dict: &Dict) -> bool {
    for entry in dict.entries() {
        if entry.code.is_empty() {
            return false;
        }
    }
    dict.contains_chars() && !dict.contains_phrases()
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
