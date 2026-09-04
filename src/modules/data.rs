/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the function
/// to read a JMU string and
/// deserialize that string
/// into a Rust data structure.
use jmu::from_str;

/// Importing the structure
/// to recursively traverse
/// a directory and return
/// the files within.
use walkdir::WalkDir;

/// Importing the "DirEntry"
/// entity to examine items
/// in a directory.
use walkdir::DirEntry;

/// Importing the data
/// structure to work
/// with paths in a cross-platform
/// way.
use std::path::PathBuf;

/// Importing the data
/// structure to catch 
/// and handle errors.
use super::err::MandyErr;

/// Importing the structure
/// that models data saved
/// in an optional data file
/// in a Mandy project.
use super::units::DataFile;

/// Importing the function to
/// read the string contents of
/// a file.
use std::fs::read_to_string;

/// Importing the standard
/// map data structure from
/// the standard library.
use std::collections::HashMap;

/// A function to read data
/// saved in the JMU format
/// from JMU files saved in the
/// `data` directory of the root
/// of a Mandy project directory.
/// If the operation is successful
/// and this directory exists and 
/// is not empty, an instance of 
/// the `Some(HashMap<String, DataFile>)`
/// structure is returned. If the directory
/// is empty or the data cannot be read,
/// an error is returned. If the directory
/// does not exist, `None` is returned.
pub fn read_data(
    project_dir: &str
) -> Result<Option<HashMap<String,DataFile>>, MandyErr>{
    let mut buf: PathBuf = PathBuf::from(project_dir);
    buf.push("data");
    if buf.exists(){
        let mut jmu_files: Vec<PathBuf> = Vec::new();
        let walker: WalkDir = WalkDir::new(&buf);
        for item in walker {
            let entry: DirEntry = item?;
            if entry.path().is_file() && 
                entry.path().extension().is_some_and(|ext| ext == "jmu")
            {
                jmu_files.push(entry.path().to_path_buf());
            } 
        }
        if jmu_files.is_empty(){
            Err::<Option<HashMap<String,DataFile>>, MandyErr>(
                MandyErr::new("The \"data\" directory cannot be empty.")
            )
        }
        else {
            let mut result: HashMap<String,DataFile> = HashMap::new();
            for jmu_file in jmu_files {
                let contents: String = read_to_string(&jmu_file)?;
                let deserialized: DataFile = from_str(&contents)?;
                let os_str = match jmu_file.file_stem(){
                    Some(os_str) => os_str,
                    None => return Err::<Option<HashMap<String, DataFile>>, MandyErr>(
                        MandyErr::new("Could not parse OS string.")
                    )
                };
                let key: String = match os_str.to_str(){
                    Some(key_slice) => key_slice.to_string(),
                    None => return Err::<Option<HashMap<String, DataFile>>, MandyErr>(
                        MandyErr::new("Could not parse OS string.")
                    )
                };
                result.insert(key,deserialized);
            }
            Ok(Some(result))
        }
    }
    else {
        Ok(None)
    }
}
