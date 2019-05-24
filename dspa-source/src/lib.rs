#![feature(atomic_min_max)]

#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;

use serde::Serialize;
use structopt::StructOpt;

use dspa_lib::Topic;

pub mod operators;

#[derive(Debug, StructOpt)]
#[structopt(name = "dspa-source")]
pub struct Args {
    #[structopt(parse(from_os_str))]
    /// Path to data directory
    pub path: PathBuf,
    #[structopt(long = "tables")]
    /// Populate database with static data
    pub tables: bool,
    #[structopt(long = "streams")]
    /// Emit stream data
    pub streams: bool,
    #[structopt(long = "speedup", default_value = "1")]
    pub speedup: u64,
    #[structopt(long = "delay", default_value = "10")]
    pub delay: u64,
}

lazy_static! {
    pub static ref ARGS: Args = Args::from_args();
}
