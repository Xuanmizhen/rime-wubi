#![forbid(unsafe_code)]

use crate::db::{
    artificial_intelligence_terminology_database, custom, lexicon, thuocl,
    unicode_cjk_wubi06::cjk::{self, Table},
};
use log::{error, info};
use std::{error::Error, process::ExitCode};

pub mod db;
pub mod generate;
pub(crate) mod rime;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut dict = db::rime_data::load_and_merge_dicts()?;
    let mut table = Table::load(&*cjk::PATH)?;
    assert!(db::verify_with(&mut dict, &mut table));
    custom::update_dict_from_path(&mut dict, &table, &*custom::PATH)?;
    custom::update_dict_from_path(
        &mut dict,
        &table,
        &*artificial_intelligence_terminology_database::DATA_PATH,
    )?;
    custom::update_dict_from_path(&mut dict, &table, &*lexicon::IT_DATA_PATH)?;
    custom::update_dict_from_path(&mut dict, &table, &*lexicon::MATH_DATA_PATH)?;
    for path in &*thuocl::RECOMMENDED_DATA_PATHS {
        thuocl::update_dict_from_path(&mut dict, &table, path.as_path())?;
    }
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
