/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the data
/// structure to work
/// with paths in a cross-platform
/// way.
use std::path::PathBuf;

/// Importing the function
/// to read optional data
/// for a Mandy project.
use super::data::read_data;

/// Importing the data structure
/// representing site-wide data
/// in a Mandy project to render
/// into a template.
use super::units::SiteContext;

/// Importing the data structure
/// representing data for a page
/// on a template.
use super::units::PageContext;

/// Importing the function to
/// read and compile the content
/// in a file of the Extended Jirai
/// format.
use super::ejirai::read_ejirai;

/// Importing the function to
/// read the configuration values
/// of a Mandy project to test it.
use super::config::read_config;

/// Importing the function to
/// generate HTML code from a 
/// content file's contents to
/// test it.
use super::render::compile_html;

/// Importing the function to
/// clean a compiled Mandy project
/// to test it.
use super::clear::clear_project;

/// Importing the function to
/// read template code from a 
/// template to test it.
use super::render::read_template;

/// Importing the function to
/// get HTML and URL paths of a file
/// to test it.
use super::ejirai::get_file_paths;

/// Importing the function to build
/// a project to test it.
use super::compiler::build_project;

/// Importing the function to gather
/// information on iterative content
/// to test it.
use super::iter::read_iter_content;

/// Importing the structure used for
/// template context for explicit 
/// typing.
use super::units::SinglePageContext;

/// Importing the function gathering a
/// list of all content files and their data
/// to test it.
use super::gather::gather_content_data;

/// Importing the function gathering a
/// list of all content files to test it.
use super::gather::gather_content_files;

/// Testing the "read_config" function.
#[test]
pub fn test_read_config(){
    let mut project_buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_buf.push("sample");
    let config = read_config(&project_buf.display().to_string())
        .expect("Cannot read config.");
    assert_eq!(config.dist_dir, "dist".to_string());
}

/// Testing the "gather_content_files" function.
#[test]
pub fn test_gather_content_files(){
    let mut project_buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_buf.push("sample");
    let files = gather_content_files(&project_buf.display().to_string())
        .expect("Cannot gather content files.");
    assert_eq!(files.len(), 4);
}

/// Testing the "read_ejirai" function.
#[test]
pub fn test_read_ejirai(){
    let mut project_buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_buf.push("sample");
    let mut file_buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_buf.push("sample");
    file_buf.push("content");
    file_buf.push("about.ejirai");
    let data = read_ejirai(
        "dist",
        &file_buf.display().to_string(),
        &project_buf.display().to_string(),
    ).expect("Cannot read Extended Jirai.");
    assert_eq!(data.data.layout, "page".to_string());
}

/// Testing the "read_template" function.
#[test]
pub fn test_read_template(){
    let mut project_buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_buf.push("sample");
    let template_str: String = read_template(
        "page", 
        &project_buf.display().to_string()
    ).expect("Could not read template.");
    assert_ne!(template_str.len(), 0);
}

/// Testing the "get_file_paths" function.
#[test]
pub fn test_get_file_paths(){
    let mut project_buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_buf.push("sample");
    let mut file_buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_buf.push("sample");
    file_buf.push("content");
    file_buf.push("about.ejirai");
    let data = get_file_paths(
        "dist",
        &file_buf.display().to_string(),
        &project_buf.display().to_string(),
    ).expect("Cannot get file paths.");
    assert_ne!(data.html_path.len(), 0);
    assert_ne!(data.url.len(), 0);
}

/// Testing the "gather_content_data" function.
#[test]
pub fn test_gather_content_data(){
    let mut project_buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_buf.push("sample");
    let content_vec = gather_content_data(
        &project_buf.display().to_string()
    ).expect("Could not get content data.");
    assert_eq!(content_vec.len(), 4);
}

/// Testing the "compile_html" function.
#[test]
pub fn test_compile_html(){
    let mut project_buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_buf.push("sample");
    let template_str: String = read_template(
        "page", 
        &project_buf.display().to_string()
    ).expect("Could not read template.");
    let config = read_config(
        &project_buf.display().to_string()
    ).expect("Cannot read config.");
    let mut file_buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_buf.push("sample");
    file_buf.push("content");
    file_buf.push("about.ejirai");
    let data = read_ejirai(
        "dist",
        &file_buf.display().to_string(),
        &project_buf.display().to_string(),
    ).expect("Cannot read Extended Jirai.");
    let site_data = read_data(&project_buf.display().to_string())
        .expect("Could not read data.");
    let ctx: SinglePageContext = SinglePageContext{
        site: SiteContext{
            config: config,
            iter_content: None
        },
        page: PageContext {
            data: data.data.clone(),
            content: data.content,
            url: "".to_string()
        },
        data: site_data
    };
    let code = compile_html(
        &data.data.layout,
        &template_str,
        &ctx
    ).expect("Could not compile HTML.");
    assert_ne!(code.len(), 0);
}

/// Testing the "build_project" and "clear_project"
/// functions.
#[test]
pub fn test_build_site(){
    let mut project_buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_buf.push("sample");
    let build = build_project(
        &project_buf.display().to_string()
    );
    let clear = clear_project(
        &project_buf.display().to_string()
    );
    assert_eq!(build.is_ok(), true);
    assert_eq!(clear.is_ok(), true);
}

/// Testing the "read_iter_content" function.
#[test]
pub fn test_read_iter_content(){
    let mut project_buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_buf.push("sample");
    let config = read_config(
        &project_buf.display().to_string()
    ).expect("Cannot read config.");
    let dirs: Vec<String> = match config.iter_content{
        Some(dirs) => dirs,
        None => panic!("No iter content set.")
    };
    let iter_content = read_iter_content(
        &dirs,
        &project_buf.display().to_string()
    ).expect("Could not read iterative content.");
    assert_ne!(iter_content.len(), 0);
}

/// Testing the "read_data" function.
#[test]
pub fn test_read_data(){
    let mut project_buf: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_buf.push("sample");
    let site_data = read_data(&project_buf.display().to_string())
        .expect("Could not read site data.");
    assert_eq!(site_data.is_some(), true);
}
