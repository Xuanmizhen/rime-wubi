#![forbid(unsafe_code)]

use log::{error, info};
use std::{error::Error, process::ExitCode};

pub mod db;
pub mod generate;
pub(crate) mod rime;

pub fn run() -> Result<(), Box<dyn Error>> {
    let dict = db::rime_data::load_and_merge_dicts()?;
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
