/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "coutils" crate
/// for various file-related operations.
use coutils;

/// Importing the "PathBuf"
/// structure to save results
/// from walking a directory.
use std::path::PathBuf;

/// Importing Mandy's error-handling
/// structure to handle errors.
use super::err::MandyErr;

/// Importing the structure that holds all
/// information on a configuration file in a
/// a Mandy project.
use super::units::ConfigFile;

/// Importing the function to read the configuration
/// file of a Mandy project.
use super::gather::read_config;

/// Cleans a Mandy project of any directory containing the compiled
/// static site. If this operation fails, an error is returned.
pub fn clean_project(dir: &String) -> Result<(), MandyErr>{
    let config_file: ConfigFile = match read_config(dir){
        Ok(config_file) => config_file,
        Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
    };
    let mut dist_buf: PathBuf = PathBuf::new();
    dist_buf.push(dir);
    dist_buf.push(config_file.contents.dist_dir);
    let del_op: () = match coutils::del_dir(&dist_buf.display().to_string()){
        Ok(del_op) => del_op,
        Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
    };
    Ok(del_op)
}