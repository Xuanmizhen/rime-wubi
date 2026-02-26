#![forbid(unsafe_code)]

use crate::db::unicode_cjk_wubi06::cjk::{self, Table};
use log::{error, info};
use std::{error::Error, process::ExitCode};

pub mod db;
pub mod generate;
pub(crate) mod rime;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut dict = db::rime_data::load_and_merge_dicts()?;
    let mut table = Table::load(&*cjk::PATH)?;
    assert!(db::verify_with(&mut dict, &mut table));
    generate::generate_yaml(&dict)?;
    Ok(())
}

fn main() -> ExitCode {
    env_logger::init();
    if let Err(e) = run() {
        let mut error = e.as_ref();
        error!("Task failed: {e}");
        while let Some(source) = error.source() {
            info!("Caused by: {source}");
            error = source;
        }
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
