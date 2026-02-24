use super::{Result, RimeDict};
use std::{path::PathBuf, sync::LazyLock};

pub static DICT_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| super::PATH.join("wubi06_tygfhzb_cjbcq.dict.yaml"));

pub fn load_dict() -> Result<RimeDict> {
    RimeDict::load(&*DICT_PATH)
}

pub fn is_valid(dict: &RimeDict) -> bool {
    !dict.data.is_empty()
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
