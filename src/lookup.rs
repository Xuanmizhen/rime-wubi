use crate::{db::unicode_cjk_wubi06::cjk::Table, rime::Dict};
use std::{
    fs::OpenOptions,
    io::{Result as IoResult, Write},
    path::PathBuf,
    sync::LazyLock,
};

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| "db/custom.txt".into());

pub fn lookup(table: &Table, dict: &Dict, phrase: &str) -> IoResult<()> {
    let code = table.get_phrase_code(phrase);
    let weight = get_weight(dict, phrase);

    match weight {
        Some(w) => println!("{}: {} {}", phrase, code, w),
        None => println!("{}: {} (not in dictionary)", phrase, code),
    }
    Ok(())
}

fn get_weight(dict: &Dict, phrase: &str) -> Option<u32> {
    dict.phrases()
        .find(|entry| entry.phrase == phrase)
        .and_then(|entry| entry.weight)
}

pub fn add(table: &Table, phrase: &str) -> IoResult<()> {
    if !is_valid_phrase(phrase, table) {
        eprintln!("Error: phrase contains unsupported characters or is empty");
        return Ok(());
    }

    let code = table.get_phrase_code(phrase);
    let line = format!("{}\n", phrase);

    let mut file = OpenOptions::new().create(true).append(true).open(&*PATH)?;

    file.write_all(line.as_bytes())?;

    println!("Added: {} -> {}", phrase, code);

    Ok(())
}

fn is_valid_phrase(phrase: &str, table: &Table) -> bool {
    if phrase.is_empty() {
        return false;
    }

    phrase.chars().all(|ch| table.contains_char(ch))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rime::DictEntry;

    fn make_table() -> Table {
        let entries = vec![
            ('中', "kh".to_string()),
            ('国', "lgy".to_string()),
            ('人', "wwww".to_string()),
            ('工', "aaaa".to_string()),
            ('大', "dddd".to_string()),
            ('计', "yfh".to_string()),
            ('算', "thaj".to_string()),
            ('机', "smn".to_string()),
        ];
        Table::from_iter(entries.into_iter())
    }

    fn make_dict() -> Dict {
        let entries = vec![
            DictEntry {
                phrase: "中国".to_string(),
                code: "klgy".to_string(),
                weight: Some(100),
            },
            DictEntry {
                phrase: "计算".to_string(),
                code: "ytha".to_string(),
                weight: Some(200),
            },
            DictEntry {
                phrase: "计算机".to_string(),
                code: "ysmn".to_string(),
                weight: Some(300),
            },
        ];
        entries.into_iter().collect()
    }

    fn make_empty_dict() -> Dict {
        vec![].into_iter().collect()
    }

    #[test]
    fn get_weight_returns_none_for_missing_phrase() {
        let dict = make_dict();
        assert_eq!(get_weight(&dict, "不存在"), None);
    }

    #[test]
    fn get_weight_returns_none_for_empty_dict() {
        let dict = make_empty_dict();
        assert_eq!(get_weight(&dict, "中国"), None);
    }

    // #[test]
    // fn get_weight_returns_correct_weight() {
    //     let dict = make_dict();
    //     assert_eq!(get_weight(&dict, "中国"), Some(100));
    //     assert_eq!(get_weight(&dict, "计算"), Some(200));
    //     assert_eq!(get_weight(&dict, "计算机"), Some(300));
    // }

    #[test]
    fn is_valid_phrase_empty_is_false() {
        let table = make_table();
        assert!(!is_valid_phrase("", &table));
    }

    #[test]
    fn is_valid_phrase_all_supported() {
        let table = make_table();
        assert!(is_valid_phrase("中国", &table));
        assert!(is_valid_phrase("计算机", &table));
    }

    #[test]
    fn is_valid_phrase_with_unsupported_char() {
        let table = make_table();
        assert!(!is_valid_phrase("中国a", &table));
        assert!(!is_valid_phrase("hello", &table));
    }

    #[test]
    fn is_valid_phrase_mixed_supported_and_unsupported() {
        let table = make_table();
        assert!(!is_valid_phrase("中国人!", &table));
        assert!(!is_valid_phrase("  ", &table));
    }

    #[test]
    fn get_phrase_code_single_char() {
        let table = make_table();
        assert_eq!(table.get_phrase_code("中"), "kh");
        assert_eq!(table.get_phrase_code("人"), "wwww");
    }

    #[test]
    fn get_phrase_code_two_chars() {
        let table = make_table();
        assert_eq!(table.get_phrase_code("中国"), "khlg");
    }

    #[test]
    fn get_phrase_code_three_chars() {
        let table = make_table();
        assert_eq!(table.get_phrase_code("计算机"), "ytsw");
    }

    #[test]
    fn get_phrase_code_four_chars() {
        let table = make_table();
        assert_eq!(table.get_phrase_code("中国人民"), "klww");
    }

    #[test]
    fn get_phrase_code_more_than_four_chars() {
        let table = make_table();
        assert_eq!(table.get_phrase_code("中华人民共和国"), "khlw");
    }
}
