use crate::rime::*;
use log::{info, trace};
use std::{
    fs,
    io::{Seek, Write},
    path::PathBuf,
    sync::LazyLock,
};
use thiserror::Error;

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| "wubi_nc.dict.yaml".into());

pub fn generate_yaml(dict: &Dict) -> Result<()> {
    info!("Generating YAML target file");
    let buf_to_write = format!("\n{}", String::from(dict));

    let file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open(&*PATH)
        .map_err(Error::TargetIo)?;
    trace!("Target file {} opened", PATH.display());
    let mut rdr = std::io::BufReader::new(file);
    yaml::skip_until_dict_data(&mut rdr)
        .map_err(Error::TargetIo)?
        .ok_or(Error::TargetSyntax)?;
    let buf_len = rdr.buffer().len() as i64;
    let mut file = rdr.into_inner();
    let mut pos = file
        .seek(std::io::SeekFrom::Current(-buf_len))
        .map_err(Error::TargetIo)?;
    file.write_all(buf_to_write.as_bytes())
        .map_err(Error::TargetIo)?;
    pos += buf_to_write.len() as u64;
    trace!("Truncating target file at position {pos}");
    file.set_len(pos).map_err(Error::TargetIo)?;
    Ok(())
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("failed to read from database")]
    Db(#[from] crate::db::Error),
    #[error("cannot manipulate target file: {}", PATH.display())]
    TargetIo(#[source] std::io::Error),
    #[error("invalid syntax in target file: {}", PATH.display())]
    TargetSyntax,
}

pub type Result<T> = std::result::Result<T, Error>;
