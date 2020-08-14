use failure::Error;
use itertools::Itertools;
use log::{debug, info, trace, warn};
use rayon::prelude::*;
use regex::{NoExpand, Regex};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

// NOTE: config of a given linter
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinterConfig {
  pub name: String,
  pub cmd: String,
  pub args: Vec<String>,
  pub ext: Vec<String>,
}

pub fn get_linters_for_ext<'a>(
  extension: &str,
  linters: &'a Vec<LinterConfig>,
) -> Vec<&'a LinterConfig> {
  linters
    .iter()
    .filter(|linter| linter.ext.contains(&extension.to_string()))
    .collect()
}

fn lint_file(file: &PathBuf, linter: &LinterConfig) -> Result<String, Error> {
  let start = Instant::now();
  debug!("[{0:?}] STARTED for {1:?}", &linter.name, &file);

  // Insert the file in the cmd
  let file_re = Regex::new(r"\{file\}")?;
  let args: Vec<String> = linter
    .args
    .iter()
    .map(|arg| {
      file_re
        .replace(&arg, NoExpand(file.to_str().unwrap()))
        .to_string()
    })
    .collect();

  trace!("[{0:?}] args: {1:?}", &linter.name, &args);

  // Get the args split by whitespace
  let cmd_output = Command::new(&linter.cmd).args(args).output()?;

  // Figure where the output is
  let stdout = cmd_output.stdout;
  let stderr = cmd_output.stderr;

  let result = if stdout.is_empty() { stderr } else { stdout };

  let duration = start.elapsed();

  debug!(
    "[{0:?}] FINISHED for {1:?} after {2:?}",
    &linter.name, &file, duration
  );
  Ok(String::from_utf8(result)?)
}

pub fn lint_files(files: Vec<&PathBuf>, linters: &Vec<&LinterConfig>) -> Result<(), Error> {
  linters.into_par_iter().panic_fuse().try_for_each(|linter| {
    let start = Instant::now();
    info!("[{:?}] STARTED", &linter);

    for file in files.iter() {
      let lint_output = lint_file(&file, &linter)?;
      trace!("finished linting {0:?} with linter {1:?}", &file, &linter);

      println!("{}", lint_output);
    }

    let duration = start.elapsed();
    info!("[{0:?}] FINISHED after {1:?}", &linter, duration);

    Ok(())
  })
}
