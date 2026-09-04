/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the data
/// structure to work
/// with paths in a cross-platform
/// way.
use std::path::PathBuf;

/// Importing the data
/// structure to catch 
/// and handle errors.
use super::err::MandyErr;

/// Importing the function to
/// easily copy files or 
/// directories.
use fs_extra::copy_items;

/// Importing the structure to
/// specifiy options for the copying
/// operation.
use fs_extra::dir::CopyOptions;

/// A function that takes a vector
/// of files and directories to copy
/// to the distribution directory and
/// attempts to do so. If the files do
/// not exist or cannot be copied, an 
/// error is returned. If the operation
/// is successful, nothing is returned.
pub fn copy_files(
    dist_dir: &str,
    project_dir: &str,
    copy_files: &Vec<String>
) -> Result<(), MandyErr>{
    for item in copy_files{
        let options: CopyOptions = CopyOptions::new();
        let mut from_buf: PathBuf = PathBuf::from(project_dir);
        let mut to_buf: PathBuf = PathBuf::from(project_dir);
        to_buf.push(dist_dir);
        from_buf.push(item);
        if from_buf.exists() {
            let _c: u64 = copy_items(
                &[&from_buf.as_path()],
                to_buf.as_path(),
                &options
            )?;
        }
        else {
            let e: String = format!(
                "The following path could not be processed: \"{}\"!",
                from_buf.display()
            );
            return Err::<(), MandyErr>(
                MandyErr::new(&e.to_string())
            );
        }
    }
    Ok(())
}
