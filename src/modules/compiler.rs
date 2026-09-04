/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// data structure
/// to work with files
/// on disk.
use std::fs::File;

/// Importing the trait
/// to write to files.
use std::io::Write;

/// Importing the data
/// structure to work
/// with paths in a cross-platform
/// way.
use std::path::PathBuf;

/// Importing the data
/// structure to catch 
/// and handle errors.
use super::err::MandyErr;

/// Importing the data structure
/// encapsulating information on
/// a Mandy project's configuration
/// values.
use super::units::Config;

/// Importing the function
/// that copies files from
/// the project directory to
/// the distribution directory.
use super::copy::copy_files;

/// Importing the function
/// to recursively create
/// directories and any
/// directories that might
/// be missing.
use std::fs::create_dir_all;

/// Importing the function to
/// read the configuration of
/// a Mandy project.
use super::config::read_config;

/// Importing the data structure
/// that encapsulates information
/// on which file is to be placed 
/// where and what that file contains.
use super::units::CompiledContent;

/// Importing the function gathering
/// information on each content file
/// in a Mandy project, including the
/// file's HTML content, URLs, and paths.
use super::gather::gather_content_data;

/// A function to compile a Mandy project
/// at the given path. If the operation is
/// successful, nothing is returned. If the
/// operation fails, an error is returned.
pub fn build_project(
    project_dir: &str
) -> Result<(), MandyErr>{
    let config: Config = read_config(project_dir)?;
    let compiled_content: Vec<CompiledContent> = gather_content_data(project_dir)?;
    for content in compiled_content{
        let mut file_dir: PathBuf = PathBuf::from(content.html_path); 
        let _mkdir: () = create_dir_all(&file_dir)?; 
        file_dir.push("index.html");
        let mut new_file: File = File::create(&file_dir)?; 
        let _w: () = new_file.write_all(content.content.as_bytes())?;
    } 
    if config.copy_files.is_some(){
        let files: Vec<String> = match config.copy_files{
            Some(files) => files,
            None => return Err::<(), MandyErr>(
                MandyErr::new("The \"copy_files\" option cannot be empty.")
            )
        };
        let _c: () = copy_files(
            &config.dist_dir, 
            project_dir, 
            &files
        )?;
    }
    Ok(())
}
