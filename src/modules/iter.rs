/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "DirEntry"
/// entity from the standard
/// library to examine items
/// in a directory.
use std::fs::DirEntry;

/// Importing the "read_dir"
/// function to get a list of
/// the items in a directory.
use std::fs::read_dir;

/// Importing the data
/// structure to work
/// with paths in a cross-platform
/// way.
use std::path::PathBuf;

/// Importing the data
/// structure for catching
/// and handling errors.
use super::err::MandyErr;

/// Importing the data structure
/// encapsulating information on
/// a Mandy project's configuration
/// values.
use super::units::Config;

/// Importing the data structure
/// encapsulating metadata on a
/// content file and the compiled
/// HTML content.
use super::units::ContentFile;

/// Importing the standard
/// data structure
/// for mapping keys to values
/// to save information on
/// iterative content.
use std::collections::HashMap;

/// Importing the data structure
/// representing data on a page
/// for a template.
use super::units::PageContext;

/// Importing the function
/// that deserializes a string
/// of Extended Jirai.
use super::ejirai::read_ejirai;

/// Importing the function to
/// read the configuration values
/// of a Mandy project.
use super::config::read_config;

/// Importing the function to
/// retrieve the file extension
/// of a file.
use super::gather::get_extension;

/// A function that attempts to read
/// any iterative content stored in
/// a Mandy project and specified in
/// the configuration file. If the 
/// operation is successful and iterative
/// content was found, read, and compiled,
/// an instance of the `HashMap` structure
/// is returned. If the operation fails, an
/// error is returned.
pub fn read_iter_content(
    dirs: &Vec<String>,
    project_dir: &str
) -> Result<HashMap<String, Vec<PageContext>>, MandyErr>{
    let config: Config = read_config(project_dir)?;
    let mut result: HashMap<String, Vec<PageContext>> = HashMap::new();
    for dir in dirs {
        let mut dir_buf: PathBuf = PathBuf::from(project_dir);
        dir_buf.push(dir);
        if dir_buf.is_dir(){
            let mut items: Vec<PageContext> = Vec::new();
            let contents = read_dir(&dir_buf)?;
            for item in contents{
                let entry: DirEntry = item?;
                let buffed_entry: PathBuf = entry
                    .path()
                    .to_path_buf();
                let ext: String = get_extension(&buffed_entry)?;
                if buffed_entry.is_file() && ext == "ejirai"{
                    let contents: ContentFile = read_ejirai(
                        &config.dist_dir,
                        &buffed_entry.display().to_string(),
                        project_dir
                    )?;
                    let ctx: PageContext = PageContext{
                        data: contents.data,
                        content: contents.content,
                        url: contents.url
                    };
                    items.push(ctx);
                }
            }
            result.insert(dir.clone(), items);
        }
    }
    Ok(result)
}
