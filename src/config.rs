use crate::git_utils::get_root_path;
use crate::lint::LinterConfig;
use failure::{bail, Error};
use log::{debug, error, trace};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub linters: Option<Vec<LinterConfig>>,
}

pub fn get_config(config: Option<PathBuf>) -> Result<Config, Error> {
    let root_path = get_root_path()?;

    trace!("root_path: {}", &root_path);

    let config_path: PathBuf = match config {
        Some(v) => {
            trace!("using user provided config path");
            v
        }
        None => {
            let mut config_path: PathBuf = PathBuf::new();
            config_path.push(&root_path);
            config_path.push("lint-staged.config.toml");

            trace!("fallback to default config_path");

            config_path
        }
    };

    debug!("config_path: {:?}", config_path);

    let config_string = std::fs::read_to_string(&config_path);

    match config_string {
        Ok(v) => {
            let serialized_toml_config = toml::from_str(&v)?;

            trace!("config: {:?}", serialized_toml_config);

            Ok(serialized_toml_config)
        }
        Err(e) => {
            error!("cant read config file from {:?}", &config_path);
            bail!(e)
        }
    }
}
