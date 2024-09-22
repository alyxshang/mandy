/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "ValueView"
/// trait to use the "SiteContext"
/// structure as a context for
/// rendering Liquid templates.
use liquid::ValueView;

/// Importing the 
/// "Deserialize" trait
/// to use it on different
/// structures in this module.
use serde::Deserialize;

/// Importing Rust's standard
/// API for working with paths.
use std::path::PathBuf;

/// Importing the "ObjectView"
/// trait to use the "SiteContext"
/// structure as a context for
/// rendering Liquid templates.
use liquid::ObjectView;

/// Importing Rust's standard API
/// for working with maps.
use std::collections::HashMap;

/// An enum that describes the two formats
/// of data file accepted by Mandy: JSON and YAML.
#[derive(Debug, Clone)]
pub enum UserDataFileType {
    JSON,
    YAML
}

/// A structure to supply context
/// to Liquid templates.
#[derive(ObjectView,ValueView, Debug)]
pub struct SiteContext{
    pub site: MandyConfig,
    pub page: MandyContent,
    pub loop_content: Option<HashMap<String, Vec<MandyContent>>>,
    pub data: Option<HashMap<String, Vec<HashMap<String,String>>>>,
    pub baseurl: String

}

/// A structure to hold all information
/// gathered on a Mandy project and compile the
/// project from this into a static site.
#[derive(Debug)]
pub struct SiteInfo {
    pub config: ConfigFile,
    pub content_files: HashMap<PathBuf, MandyContent>,
    pub data_files: Option<HashMap<String,UserDataFile>>,
    pub loop_content: Option<HashMap<String, Vec<MandyContent>>>,
    pub layouts: Vec<LayoutFile>,
    pub sass_dir: Option<PathBuf>,
    pub partials: HashMap<String,String>
}

/// A structure to hold information
/// on user-supplied layouts.
#[derive(Debug, Clone)]
pub struct LayoutFile {
    pub name: String,
    pub path: PathBuf,
    pub contents: String
}

/// A structure to hold information on a 
/// data file supplied by a user.
#[derive(Debug, Clone)]
pub struct UserDataFile {
    pub path: PathBuf,
    pub file_name: String,
    pub file_type: UserDataFileType,
    pub contents: Vec<HashMap<String, String>>
}

/// A structure to hold information
/// on a Mandy project's configuration
/// file.
#[derive(Debug, Clone)]
pub struct ConfigFile{
    pub path: PathBuf,
    pub file_type: UserDataFileType,
    pub contents: MandyConfig
}

/// A structure to hold information
/// on a Mandy project's configuration
/// options.
#[derive(Deserialize, ObjectView, ValueView, Debug, Clone)]
pub struct MandyConfig {
    pub tl_domain: String,
    pub seo: bool,
    pub title: String,
    pub dist_dir: String,
    pub description: String,
    pub prod_url: String,
    pub dev_url: String,
    pub copy_files: bool,
    pub has_loop_content: bool,
    pub copy_entities: Option<Vec<String>>,
    pub loop_content_dirs: Option<Vec<String>>,
    pub user_config: HashMap<String, String>
}

/// A structure to hold information
/// on a Markdown document in a Mandy
/// project.
#[derive(Deserialize, ObjectView, ValueView, Debug, Clone)]
pub struct ContentStore {
    pub layout: String,
    pub params: HashMap<String, String>,
    pub content: String
}

/// A structure that holds information
/// on a file in a Mandy project
/// containing content written in
/// the Markdown language.
#[derive(Deserialize, ObjectView, ValueView, Debug, Clone)]
pub struct MandyContent{
    pub layout: String,
    pub params: HashMap<String, String>,
    pub content: String,
    pub url: String,
    pub path: String
}

/// This structure
/// stores information
/// on the parent directories
/// and the file stem of a path
/// to a file.
pub struct PathInfo {
    pub path: PathBuf,
    pub file: String
}

/// An enum that describes
/// where to insert an item
/// in an instance of the
/// "PathBuf" structure.
pub enum Direction{
    Before,
    After
}

/// A structure
/// that stores all the URLs
/// a compiled Mandy
/// project has.
pub struct SiteMap{
    pub urls: Vec<SiteMapUrl>
}

/// Implementing functions
/// for the "SiteMap"
/// structure.
impl SiteMap{
   
    /// Returns aN XML representation
    /// of this structure.
    pub fn to_string(&self) -> String{
        let mut url_string_vec: Vec<String> = Vec::new();
        for url in &self.urls{
            url_string_vec.push(url.to_string());
        }
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{}</urlset>", 
            url_string_vec.join("\n")
        )
    }
}

/// A structure
/// that stores a URL
/// in a compiled Mandy
/// project.
pub struct SiteMapUrl {
    pub url: String
}

/// Implementing functions
/// for the "SiteMapUrl"
/// structure.
impl SiteMapUrl{

    /// Returns aN XML representation
    /// of this structure.
    pub fn to_string(&self) ->String {
        format!("<url><loc>{}</loc></url>", &self.url)
    }

}

/// A structure
/// to store information
/// about a modified instance
/// of the "PathBuf" structure.
pub struct ComplexPath{
    pub on_disk_html_url: String,
    pub web_link: String
}