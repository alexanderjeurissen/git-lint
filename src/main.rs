extern crate pretty_env_logger;

mod config;
mod git_utils;
mod lint;

use atty::Stream;
use config::*;
use failure::Error;
use git_utils::get_staged_file_paths;
use lint::*;
use log::{debug, info, trace, warn};
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

    let linters = config.linters.unwrap();

    trace!("linters: {:?}", linters);

    let staged_files: Vec<PathBuf> = get_staged_file_paths()?;

    let grouped_staged_files: std::vec::IntoIter<(&str, Vec<PathBuf>)> =
        Box::new(staged_files.into_iter())
            .group_by(|file| match &file.extension() {
                Some(ext) => ext.to_str().unwrap(),
                None => "None",
            })
            .into_iter();

    for (ext, files) in grouped_staged_files {
        let linters_for_ext: Vec<&LinterConfig> = get_linters_for_ext(&ext, &linters);
        match linters_for_ext.is_empty() {
            true => warn!("No linters specified for [{}] skipping...", &ext),
            false => info!("we have linters for [{}] linting...", &ext),
        }
    }

    // NOTE: we are done log total execution time
    let duration = start.elapsed();
    info!("'FINISHED after {:?}", duration);

    return Ok(());
}
