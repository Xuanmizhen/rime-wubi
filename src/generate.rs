use crate::rime_yaml;
use log::trace;
use std::{fs, io::Seek, path::PathBuf, sync::LazyLock};
use thiserror::Error;

pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| "wubi_nc.dict.yaml".into());

pub fn generate_yaml() -> Result<()> {
    let file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open(&*PATH)
        .map_err(Error::TargetIo)?;
    trace!("Target file {} opened", PATH.display());
    let mut rdr = std::io::BufReader::new(file);
    rime_yaml::skip_until_dict_data(&mut rdr)
        .map_err(Error::TargetIo)?
        .ok_or(Error::TargetSyntax)?;
    let buf_len = rdr.buffer().len() as i64;
    let mut file = rdr.into_inner();
    let mut pos = file
        .seek(std::io::SeekFrom::Current(-buf_len))
        .map_err(Error::TargetIo)?;
    todo!();
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
