/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the `derive`
/// macro to serialize
/// a Rust data structure
/// into a string.
use serde::Serialize;

/// Importing the `derive`
/// macro to deserialize
/// a string into a Rust
/// data structure.
use serde::Deserialize;

/// Importing the standard
/// key-value data structure.
use std::collections::HashMap;

/// A data structure to
/// store configuration
/// values of a Mandy
/// project.
#[derive(Deserialize, Clone, Serialize)]
pub struct Config {
    pub dist_dir: String,
    pub copy_files: Option<Vec<String>>, 
    pub iter_content: Option<Vec<String>>,
    pub user_config: Option<HashMap<String, String>>
}

/// A data structure to
/// store configuration
/// values of single
/// page of a Mandy
/// project.
#[derive(Deserialize, Serialize, Clone)]
pub struct PageData{
    pub title: String,
    pub layout: String,
    pub description: String,
    pub user_config: Option<HashMap<String, String>>
}

/// A data structure to
/// store template values
/// for a single instance
/// of template rendering
/// in the case of a single
/// page of a Mandy
/// project.
#[derive(Serialize)]
pub struct SinglePageContext{
    pub site: SiteContext,
    pub page: PageContext,
    pub data: Option<HashMap<String, DataFile>>
}

/// A data structure to
/// store extra template
/// context values 
/// of an entire Mandy
/// project.
#[derive(Serialize)]
pub struct SiteContext{
    pub config: Config,
    pub iter_content: Option<HashMap<String, Vec<PageContext>>>
}

/// A data structure to
/// store raw template 
/// context values of 
/// a single page of a 
/// Mandy project.
#[derive(Serialize, Clone)]
pub struct PageContext{
    pub data: PageData,
    pub content: Option<String>,
    pub url: String,
}

/// A data structure to
/// encapsulate data captured
/// from a content file written
/// in the Extended Jirai format.
#[derive(Clone)]
pub struct ContentFile{
    pub url: String,
    pub data: PageData,
    pub html_path: String,
    pub content: Option<String>
}

/// A data structure to
/// encapsulate data about
/// a page after all configuration
/// values and individual settings 
/// on a page have been processed.
#[derive(Debug)]
pub struct CompiledContent{
    pub content: String,
    pub html_path: String
}

/// A data structure
/// to encapsulate 
/// modified paths
/// for a content
/// file in a Mandy
/// project.
#[derive(Debug)]
pub struct FilePaths{
    pub url: String,
    pub html_path: String
}

/// A data structure to
/// store configuration
/// values of data of
/// a Mandy project.
#[derive(Deserialize, Serialize, Clone)]
pub struct DataFile {
    pub items: Vec<HashMap<String, String>>
}
