#![forbid(unsafe_code)]

pub mod db;
pub mod generate;

fn main() -> Result<(), db::rime_data::Error> {
    env_logger::init();
    Ok(())
}
