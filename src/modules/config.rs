/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the function
/// to read a JMU string and
/// deserialize that string
/// into a Rust data structure.
use jmu::from_str;

/// Importing the data
/// structure to work
/// with paths in a cross-platform
/// way.
use std::path::PathBuf;

/// Importing the data structure
/// encapsulating information on
/// a Mandy project's configuration
/// values.
use super::units::Config;

/// Importing the data
/// structure to catch 
/// and handle errors.
use super::err::MandyErr;

/// Importing the function to
/// read the string contents of
/// a file.
use std::fs::read_to_string;

/// A function to read the configuration
/// file of a Mandy project. If the operation
/// succeeds, this string is deserialized into
/// an instance of the `Config` data structure
/// and returned. If the operation fails, an
/// error is returned.
pub fn read_config(
    project_dir: &str
) -> Result<Config, MandyErr>{
    let mut config_buf: PathBuf = PathBuf::from(project_dir);
    config_buf.push("config.jmu");
    let contents: String = read_to_string(&config_buf)?;
    let config: Config = from_str(&contents)?;
    Ok(config)
}
