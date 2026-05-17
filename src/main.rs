#![forbid(unsafe_code)]

use crate::{
    cli::Command,
    db::{
        artificial_intelligence_terminology_database, custom, lexicon, thuocl,
        unicode_cjk_wubi06::cjk::{self, Table},
    },
};
use clap::Parser;
use std::{error::Error, process::ExitCode};

pub mod cli;
pub mod db;
pub mod generate;
pub(crate) mod lookup;
pub(crate) mod rime;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut dict = db::rime_data::load_and_merge_dicts()?;
    let mut table = Table::load(&*cjk::PATH)?;
    if !db::verify_with(&mut dict, &mut table) {
        return Err("Verification failed".into());
    }
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

pub fn run_cli() -> Result<(), Box<dyn Error>> {
    let table = cjk::Table::load(&*cjk::PATH)?;
    let dict = db::rime_data::load_and_merge_dicts()?;

    match Command::parse() {
        Command::Generate => run()?,
        Command::Lookup { phrase } => lookup::lookup(&table, &dict, &phrase)?,
        Command::Add { phrase } => lookup::add(&table, &phrase)?,
    }

    Ok(())
}

fn main() -> ExitCode {
    let _ = env_logger::try_init();
    if let Err(e) = run_cli() {
        eprintln!("Task failed: {}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
