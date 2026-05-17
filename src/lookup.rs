use crate::{
    db::unicode_cjk_wubi06::cjk::Table,
    rime::Dict,
};
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
        None => println!("{}: {} (不在词库中)", phrase, code),
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
        eprintln!("错误: 词语包含不支持的字符或为空");
        return Ok(());
    }

    let code = table.get_phrase_code(phrase);
    let line = format!("{}\n", phrase);
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&*PATH)?;
    
    file.write_all(line.as_bytes())?;
    
    println!("已添加: {} -> {}", phrase, code);
    
    Ok(())
}

fn is_valid_phrase(phrase: &str, table: &Table) -> bool {
    if phrase.is_empty() {
        return false;
    }
    
    phrase.chars().all(|ch| table.contains_char(ch))
}
