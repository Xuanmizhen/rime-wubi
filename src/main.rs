#![forbid(unsafe_code)]

use log::{error, info};
use std::{error::Error as _, process::ExitCode};

pub mod db;
pub mod generate;
pub(crate) mod rime_yaml;

fn main() -> ExitCode {
    env_logger::init();
    if let Err(e) = generate::generate_yaml() {
        error!("Failed to generate YAML: {e}");
        let mut source = e.source();
        while let Some(s) = source {
            info!("Caused by: {s}");
            source = s.source();
        }
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
