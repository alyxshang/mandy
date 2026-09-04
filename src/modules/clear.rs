/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the standard
/// data structure for
/// handling paths in a 
/// cross-platform way.
use std::path::PathBuf;

/// Importing the data
/// structure for catching
/// and handling errors.
use super::err::MandyErr;

/// Importing the data structure
/// containing configuration
/// options for a Mandy project.
use super::units::Config;

/// Importing the function from
/// the standard library for
/// removing a directory and
/// the items within it.
use std::fs::remove_dir_all;

/// Importing the function to
/// read the configuration values
/// of a Mandy project.
use super::config::read_config;

/// A function to clean
/// a compiled Mandy project.
/// The configuration value for the
/// `dist_dir` directory is read and
/// the directory is deleted from 
/// the project. If the operation 
/// is successful, nothing is returned. 
/// If the operation fails, an error
/// is returned.
pub fn clear_project(
    project_dir: &str,
) -> Result<(), MandyErr>{
    let config: Config = read_config(project_dir)?;
    let mut buf: PathBuf = PathBuf::from(project_dir);
    buf.push(config.dist_dir);
    if buf.exists(){
        let d: () = remove_dir_all(&buf)?;
        Ok(d)
    }
    else {
        let e: String = format!(
            "The path \"{}\" does not exist.",
            buf.display()
        );
        Err::<(), MandyErr>(
            MandyErr::new(&e.to_string())
        )
    }
}
