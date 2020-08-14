extern crate pretty_env_logger;

mod config;
mod git_utils;
mod lint;

use atty::Stream;
use config::*;
use failure::Error;
use git_utils::get_staged_file_paths;
use lint::*;
use log::{info, trace};
use std::env;
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "lint-staged", about = "Explanation of bin-wrapper usage.")]
struct Cli {
    #[structopt(long, help = "specify custom config path")]
    config: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let start = Instant::now();

    pretty_env_logger::init_custom_env("LOG");

    let args = Cli::from_args();

    let config = get_config(args.config)?;

    trace!("config: {:?}", config);

    let staged_files: Vec<PathBuf> = get_staged_file_paths()?;

    for file in staged_files {
        match file.extension() {
            Some(v) => trace!("{:?}", v),
            None => trace!("no extension"),
        };
    }

    // NOTE: we are done log total execution time
    let duration = start.elapsed();
    info!("'FINISHED after {:?}", duration);

    return Ok(());
}
