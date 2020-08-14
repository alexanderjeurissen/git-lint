extern crate pretty_env_logger;

mod config;
mod git_utils;
mod lint;

// use atty::Stream;
use config::*;
use failure::Error;
use git_utils::get_staged_file_paths;
use lint::*;
use log::{debug, info, trace, warn};
use std::path::PathBuf;
use std::str::{FromStr, ParseBoolError};
use std::time::Instant;
use structopt::StructOpt;

fn parse_bool(v: &str) -> Result<bool, ParseBoolError> {
    match v {
        "1" => Ok(true),
        "0" => Ok(false),
        _ => Ok(<bool as FromStr>::from_str(v)?),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "lint-staged", about = "Explanation of bin-wrapper usage.")]
struct Cli {
    #[structopt(long, help = "specify custom config path", parse(from_os_str))]
    config: Option<PathBuf>,

    #[structopt(long, env = "LINT_STAGED", parse(try_from_str = parse_bool))]
    staged: bool,
}

fn main() -> Result<(), Error> {
    let start = Instant::now();

    pretty_env_logger::init_custom_env("LOG");

    let args = Cli::from_args();

    // NOTE: stop execution if LINT_STAGED is not set
    match args.staged {
        true => {
            trace!("LINT_STAGED set, resuming execution... {}", &args.staged);
        }
        false => {
            trace!("LINT_STAGED not set or invalid value, skipping...");
            return Ok(());
        }
    }

    let config = get_config(args.config)?;

    let linters = config.linters.unwrap();

    trace!("linters: {:?}", linters);

    let staged_files: Vec<PathBuf> = get_staged_file_paths()?;
    let file_extension = |file: PathBuf| match &file.extension() {
        Some(ext) => ext.to_str().unwrap().to_string(),
        None => "None".to_string(),
    };

    // let grouped_staged_files: std::vec::IntoIter<(&str, Vec<PathBuf>)> =
    // staged_files.iter().group_by(file_extension).into_iter();

    // for (ext, files) in grouped_staged_files {
    //     let linters_for_ext: Vec<&LinterConfig> = get_linters_for_ext(&ext, &linters);
    //     match linters_for_ext.is_empty() {
    //         true => warn!("No linters specified for [{}] skipping...", &ext),
    //         false => info!("we have linters for [{}] linting...", &ext),
    //     }
    // }

    // NOTE: we are done log total execution time
    let duration = start.elapsed();
    info!("'FINISHED after {:?}", duration);

    return Ok(());
}
