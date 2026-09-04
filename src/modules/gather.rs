/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

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

/// Importing the data structure
/// encapsulating information on
/// a Mandy project's configuration
/// values.
use super::units::Config;

/// Importing the function to
/// read optional data in a 
/// Mandy project.
use super::data::read_data;

/// Importing the structure
/// that models data saved
/// in an optional data file
/// in a Mandy project.
use super::units::DataFile;

/// Importing the data structure
/// representing all relevant data
/// in a Mandy project
/// for a template.
use super::units::SiteContext;

/// Importing the standard
/// data structure
/// for mapping keys to values
/// to save information on
/// iterative content.
use std::collections::HashMap;

/// Importing the data structure
/// encapsulating metadata on a
/// content file and the compiled
/// HTML content.
use super::units::ContentFile;

/// Importing the data structure
/// representing data on a page
/// for a template.
use super::units::PageContext;

/// Importing the function to
/// read and compile the content
/// in a file of the Extended Jirai
/// format.
use super::ejirai::read_ejirai;

/// Importing the function to
/// read the configuration values
/// of a Mandy project.
use super::config::read_config;

/// Importing the function to
/// generate HTML code from a 
/// content file's contents.
use super::render::compile_html;

/// Importing the function to
/// read template code from a 
/// template.
use super::render::read_template;

/// Importing the data structure
/// encapsulating data on a content
/// file consisting of the HTML
/// code it is compiled to and 
/// the path at which the file
/// containing this code is created.
use super::units::CompiledContent;

/// Importing the function to read
/// iterative content if the project
/// uses such content.
use super::iter::read_iter_content;

/// Importing the data structure
/// unifying page and site context
/// in one data structure to use
/// for rendering content into a
/// layout.
use super::units::SinglePageContext;

/// A function that attempts to
/// detect all files ending in
/// the "ejirai" file extension.
/// These files must all contain
/// content written in the Extended
/// Jirai format for the Mandy project
/// to build successfully. If any errors
/// happen during the detection of these
/// files, an error is returned.
pub fn gather_content_files(
    project_dir: &str
) -> Result<Vec<String>, MandyErr>{
    let buffer: PathBuf = PathBuf::from(project_dir);
    let walker: WalkDir = WalkDir::new(
        buffer.display().to_string()
    );
    let mut result: Vec<String> = Vec::new();
    for entity in walker{
        let entry: DirEntry = entity?;
        if entry.path().is_file() && 
           entry.path().extension().is_some_and(|ext| ext == "ejirai")
        {
            result.push(entry.path().to_path_buf().display().to_string());
        }
    }
    Ok(result)
}

/// A function that attempts to summarize
/// the data in a content file. The result of
/// this operation is a vector of instances of
/// the `CompiledContent` structure that each
/// contain the HTML code of the compiled content
/// and the path on disk where the file containing
/// this compiled content is placed. If any errors
/// happen along the way, these errors are returned.
pub fn gather_content_data(
    project_dir: &str
) -> Result<Vec<CompiledContent>, MandyErr>{
    let config: Config = read_config(project_dir)?;
    let iter_content: Option<HashMap<String, Vec<PageContext>>> = 
        match &config.iter_content{
            Some(dirs) => Some(read_iter_content(dirs,project_dir)?),
            None => None
        };
    let mut result: Vec<CompiledContent> = Vec::new();
    let files: Vec<String> = gather_content_files(project_dir)?;
    let site_data: Option<HashMap<String, DataFile>> = read_data(project_dir)?;
    for file in files{
        let content: ContentFile = read_ejirai(
            &config.dist_dir, 
            &file, 
            project_dir
        )?;
        let page_context: PageContext = PageContext{
            data: content.data.clone(),
            content: content.content,
            url: content.url,
        };
        let site_context: SiteContext = SiteContext {
            config: config.clone(),
            iter_content: iter_content.clone()
        };
        let ctx: SinglePageContext = SinglePageContext{
            site: site_context,
            page: page_context,
            data: site_data.clone()
        };
        let template_code: String = read_template(
            &content.data.layout, 
            project_dir
        )?; 
        let code: String = compile_html(
            &content.data.layout,
            &template_code,
            &ctx
        )?;
        let item: CompiledContent = CompiledContent{
            html_path: content.html_path,
            content: code
        };
        result.push(item);
    }
    Ok(result)
}

/// A function that attempts to 
/// retrieve the file extensio of a
/// file as a string. If the operation
/// is successful, the extension is returned
/// as a string. If the operation fails, an
/// error is returned.
pub fn get_extension(
    buf: &PathBuf
) -> Result<String, MandyErr>{
    let os_str: Option<&str> = match buf.as_path().extension(){
        Some(os_str) => os_str.to_str(),
        None => return Err::<String, MandyErr>(
            MandyErr::new(
                &format!(
                    "Could not get \"OsStr\" for path \"{}\"!", 
                    buf.display()
                )
            )
        )
    };
    let ext: String = match os_str{
        Some(ext) => ext.to_string(),
        None => return Err::<String, MandyErr>(
            MandyErr::new(
                &format!(
                    "Could not get extension for path \"{}\"!", 
                    buf.display()
                )
            )
        )
    };
    Ok(ext)
}
