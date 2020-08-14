use serde::{Deserialize, Serialize};

// NOTE: config of a given linter
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinterConfig {
  pub name: String,
  pub cmd: String,
  pub args: Vec<String>,
  pub ext: Vec<String>,
}
