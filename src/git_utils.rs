use failure::Error;
use log::{info, trace};
use std::path::PathBuf;
use std::process::Command;

// NOTE: run git diff --staged --name-only
// and parse the stdout to a String
fn get_staged_file_names() -> Result<String, Error> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--staged")
        .arg("--diff-filter=ACMR")
        .arg("--name-only")
        .arg("-z")
        .output()?;

    let output_str: String = String::from_utf8(output.stdout)?;

    info!("staged files: \n{}", output_str);

    Ok(output_str)
}

// NOTE: get the root path of current git repository
pub fn get_root_path() -> Result<String, Error> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;

    let output_str: String = String::from_utf8(output.stdout)?;

    Ok(output_str.trim().to_string())
}

// NOTE: interpolate root_path and file names
// Result is a Vec<String> with absolute paths to staged files
pub fn get_staged_file_paths() -> Result<Vec<PathBuf>, Error> {
    let staged_filenames = get_staged_file_names()?;
    let root_path = get_root_path()?;

    let staged_files = staged_filenames
        .lines()
        .map(|file_name: &str| -> PathBuf {
            let mut path: PathBuf = PathBuf::new();
            path.push(&root_path);
            path.push(file_name);
            return path;
        })
        .collect();

    trace!("staged file paths: {:?}", staged_files);

    return Ok(staged_files);
}
