use itertools::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// NOTE: config of a given linter
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinterConfig {
  pub name: String,
  pub cmd: String,
  pub args: Vec<String>,
  pub ext: Vec<String>,
}

pub fn get_linters_for_ext<'a>(
  extension: &String,
  linters: &'a Vec<LinterConfig>,
) -> Vec<&'a LinterConfig> {
  linters
    .iter()
    .filter(|linter| linter.ext.contains(&extension))
    .collect()
}

// pub fn group_files_by_ext(files: &Vec<PathBuf>) -> std::vec::IntoIter<(&str, Vec<PathBuf>)> {
//   &files
//     .iter()
//     .group_by(|file| match &file.extension() {
//       Some(ext) => ext.to_str().unwrap(),
//       None => "None",
//     })
//     .into_iter()
// }
