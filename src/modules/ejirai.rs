/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the function
/// that deserializes a string
/// of Extended Jirai.
use jirai::from_ejirai;

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
/// that represents deserialized
/// data obtained from an Extended
/// Jirai document.
use jirai::ExtendedJirai;

/// Importing the data structure
/// representing settings inside
/// an individual content file.
use super::units::PageData;

/// Importing the function to
/// read the string contents of
/// a file.
use std::fs::read_to_string;

/// Importing the data structure
/// encapsulating the HTML path
/// and URL of a content file.
use super::units::FilePaths;

/// Importing the data structure
/// encapsulating metadata on a
/// content file and the compiled
/// HTML content.
use super::units::ContentFile;

/// A function to read and compile 
/// a content file written in the 
/// Extended Jirai format, gather 
/// some metadata, and return 
/// that information in an instance
/// of the `ContentFile` structure.
/// If the operation fails, an error
/// is returned.
pub fn read_ejirai(
    dist_dir: &str,
    file_path: &str,
    project_dir: &str
) -> Result<ContentFile, MandyErr>{
    let file_buf: PathBuf = PathBuf::from(
        file_path
    );
    let contents: String = read_to_string(&file_buf)?;
    let deserialized: ExtendedJirai<PageData> = from_ejirai(&contents)?;
    let paths: FilePaths = get_file_paths(
        dist_dir,
        &file_buf.display().to_string(),
        project_dir
    )?;
    let result: ContentFile = ContentFile{
        url: paths.url,
        data: deserialized.data,
        content: deserialized.code,
        html_path: paths.html_path
    };
    Ok(result)
}

/// A function to build the paths
/// for the HTML file to be created
/// and the URL of that HTML file.
/// This information is returned in
/// an instance of the `FilePaths`
/// structure. If the operation fails,
/// an error is returned.
pub fn get_file_paths(
    dist_dir: &str,
    file_path: &str,
    project_dir: &str
) -> Result<FilePaths, MandyErr>{
    let file_buf: PathBuf = PathBuf::from(file_path);
    let project_buf: PathBuf = PathBuf::from(project_dir);
    let file_stem: Option<&str> = match file_buf.file_stem(){
        Some(stem) => stem.to_str(),
        None => return Err::<FilePaths, MandyErr>(
            MandyErr::new("Could not get file stem.")
        )
    };
    let normalized_file_stem: String = match file_stem{
        Some(stem) => stem.to_string(),
        None => return Err::<FilePaths, MandyErr>(
            MandyErr::new("Could not get file stem.")
        )
    };
    let parent_dir: PathBuf = match file_buf.parent(){
        Some(parent_dir) => parent_dir.to_path_buf(),
        None => return Err::<FilePaths, MandyErr>(
            MandyErr::new("Could not get parent dir.")
        )
    };
    let file_dir_in_project = parent_dir.strip_prefix(
        project_buf
    )?; 
    let mut html_path_buf: PathBuf = PathBuf::from(project_dir);
    let mut url_path_buf: PathBuf = PathBuf::from("/");
    if file_dir_in_project.to_path_buf().display().to_string().is_empty() &&
       normalized_file_stem == "index"
    {
        html_path_buf.push(dist_dir);
    }
    else{
        html_path_buf.push(dist_dir);
        html_path_buf.push(file_dir_in_project);
        html_path_buf.push(normalized_file_stem.clone());
        url_path_buf.push(file_dir_in_project);
        url_path_buf.push(normalized_file_stem.clone());
    }
    let result: FilePaths = FilePaths{
        url: url_path_buf.display().to_string(),
        html_path: html_path_buf.display().to_string()
    };
    Ok(result)
}
