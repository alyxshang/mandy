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

/// Importing the "SiteInfo"
/// structure to store information
/// about the Mandy project.
use super::units::SiteInfo;

/// Importing the enum to specify
/// in which direction an insertion 
/// should take place.
use super::units::Direction;

/// Importing the structure that holds all
/// information on a configuration file in a
/// a Mandy project.
use super::units::ConfigFile;

/// Importing the "SASSFile"
/// structure to store info
/// about a Liquid file.
use super::units::LayoutFile;

/// Importing the structure
/// to store information about
/// a modified instance of the
/// "PathBuf" structure.
use super::units::ComplexPath;

/// Importing Rust's standard API
/// for working with maps.
use std::collections::HashMap;

/// Importing the structure that holds all
/// configuration options set for a Mandy
/// project.
use super::units::MandyConfig;

/// Importing the structure
/// to store information about
/// a file containing content
/// written in Markdown.
use super::units::ContentStore;

/// Importing the structure that holds
/// information on any data files that a
/// Mandy project might have.
use super::units::UserDataFile;

/// Importing the function to
/// extract the basic
/// name of a file.
use super::utils::extract_stem;

/// Importing the structure to
/// store information
/// about parsed Markdown files.
use super::units::MandyContent;

/// Importing the enum that describes types of config
/// and data files a Mandy project can have.
use super::units::UserDataFileType;

/// Importing the function to parse a Markdown
/// document with frontmatter into an instance
/// of the "MandyContent" structure.
use super::processors::parse_document;

/// Importing the function to parse
/// YAML data files.
use super::processors::parse_yml_data;

/// Importing a function to parse
/// and modify an instance of the "PathBuf"
/// structure.
use super::utils::produce_complex_path;

/// Importing the function to parse YAML
/// from a configuration file into an instance
/// of the "MandyConfig" structure.
use super::processors::parse_yml_config;

/// Importing the function to find files
/// with the provided ending in the provided
/// directory.
use super::utils::find_files_with_ending;

/// Importing the function to parse JSON
/// from a configuration file into an instance
/// of the "MandyConfig" structure.
use super::processors::parse_json_config;

/// Importing the function to insert an entity
/// into an instance of the "PathBuf" structure
/// at the given item.
use super::utils::insert_entity_into_buf_at_item;

/// Attempts to find all files ending in ".scss" in the project directory's "sass"
/// directory. If this operation fails, an error is returned. A "Result" type is returned.
pub fn find_sass_files(dir: &String) -> Result<Option<PathBuf>,MandyErr> {
    let mut sass_buf: PathBuf = PathBuf::new();
    sass_buf.push(dir);
    sass_buf.push("sass");

    let mut sass_file_buf: PathBuf = PathBuf::new();
    sass_file_buf.push(dir);
    sass_file_buf.push("sass");
    sass_file_buf.push("index.scss");
    if sass_buf.exists() && sass_file_buf.exists(){
        Ok(Some(sass_file_buf))
    }
    else if sass_buf.exists() && !sass_file_buf.exists(){
        let e: String = format!(
            "The SASS directory at the path \"{}\" exists but does not contain an \"index.scss\" file.", 
            &sass_buf.display().to_string()
        );
        Err::<Option<PathBuf>,MandyErr>(MandyErr::new(&e.to_string()))
    }
    else {
        Ok(None)
    }
}

/// Attempts to find all files ending in ".liquid" in the project directory's "layouts"
/// directory. If this operation fails, an error is returned. A "Result" type is returned.
pub fn find_layout_files(dir: &String) -> Result<Vec<LayoutFile>, MandyErr> {
    let mut result: Vec<LayoutFile> = Vec::new();
    let mut layouts_buf: PathBuf = PathBuf::new();
    layouts_buf.push(dir);
    layouts_buf.push("layouts");
    if layouts_buf.exists(){
        let layouts_files: Option<Vec<PathBuf>> = match find_files_with_ending(&layouts_buf.display().to_string(), "liquid"){
            Ok(layouts_files) => layouts_files,
            Err(e) => return Err::<Vec<LayoutFile>, MandyErr>(MandyErr::new(&e.to_string()))
        };
        let files: Vec<PathBuf> = match layouts_files {
            Some(files) => files,
            None => {
                let e: String = format!("No layout files found at the following path: \"{}\"!", &layouts_buf.display().to_string());
                return Err::<Vec<LayoutFile>, MandyErr>(MandyErr::new(&e.to_string())) 
            }
        };
        for file in files {
            let file_path: String = file.display().to_string();
            let stem: String = match extract_stem(&file_path){
                Ok(stem) => stem,
                Err(e) => return Err::<Vec<LayoutFile>, MandyErr>(MandyErr::new(&e.to_string()))
            };
            let contents: String = match coutils::read_file(&file_path){
                Ok(contents) => contents,
                Err(e) => return Err::<Vec<LayoutFile>, MandyErr>(MandyErr::new(&e.to_string()))
            };
            result.push(LayoutFile{ name: stem, path: file, contents: contents});
        }
        Ok(result)
    }
    else {
        let e: String = format!("The directory for layouts \"{}\" does not exist.", &layouts_buf.display().to_string());
        Err::<Vec<LayoutFile>, MandyErr>(MandyErr::new(&e.to_string()))
    }
    
}

/// Parses the configuration file in a Mandy project and returns an 
/// instance of the "ConfigFile" structure. If this operation fails,
/// an error is returned. A "Result" type is returned.
pub fn read_config(dir: &String) -> Result<ConfigFile, MandyErr>{
    let mut json_config_buf: PathBuf = PathBuf::new();
    json_config_buf.push(dir);
    json_config_buf.push("config.json");
    let mut yml_config_buf: PathBuf = PathBuf::new();
    yml_config_buf.push(dir);
    yml_config_buf.push("config.yml");
    if json_config_buf.exists(){
        let src: String = match coutils::read_file(&json_config_buf.display().to_string()){
            Ok(src) => src,
            Err(e) => return Err::<ConfigFile, MandyErr>(MandyErr::new(&e.to_string()))
        };
        let config: MandyConfig = match parse_json_config(&src){
            Ok(config) => config,
            Err(e) => return Err::<ConfigFile, MandyErr>(MandyErr::new(&e.to_string()))
        };
        Ok(ConfigFile{ path: json_config_buf, file_type: UserDataFileType::JSON, contents: config})

    }
    else if yml_config_buf.exists(){
        let src: String = match coutils::read_file(&yml_config_buf.display().to_string()){
            Ok(src) => src,
            Err(e) => return Err::<ConfigFile, MandyErr>(MandyErr::new(&e.to_string()))
        };
        let config: MandyConfig = match parse_yml_config(&src){
            Ok(config) => config,
            Err(e) => return Err::<ConfigFile, MandyErr>(MandyErr::new(&e.to_string()))
        };
        Ok(ConfigFile{ path: yml_config_buf, file_type: UserDataFileType::YAML, contents: config})
    }
    else {
        let e: String = format!("No config file found in the directory \"{}\".", dir);
        Err::<ConfigFile, MandyErr>(MandyErr::new(&e.to_string()))
    }
}

/// Reads the "$project_dir/data" directory for files in either
/// JSON or YAML format and retrieves the data held in these files.
/// If this operation fails, an error is returned.
pub fn read_data_files(dir: &String) -> Result<Option<HashMap<String,UserDataFile>>, MandyErr>{
    let mut data_buf: PathBuf = PathBuf::new();
    let e: String = format!("The project's \"data\" directory cannot be empty.");
    data_buf.push(dir);
    data_buf.push("data");
    if data_buf.exists(){
        let pos_yaml_files: Option<Vec<PathBuf>> = match find_files_with_ending(&data_buf.display().to_string(), "yml"){
            Ok(yaml_files) => yaml_files,
            Err(e) => return Err::<Option<HashMap<String,UserDataFile>>, MandyErr>(MandyErr::new(&e.to_string()))
        };
        let pos_json_files: Option<Vec<PathBuf>> = match find_files_with_ending(&data_buf.display().to_string(), "json"){
            Ok(yaml_files) => yaml_files,
            Err(e) => return Err::<Option<HashMap<String,UserDataFile>>, MandyErr>(MandyErr::new(&e.to_string()))
        };


        if pos_yaml_files.is_some(){
            let mut result: HashMap<String,UserDataFile> = HashMap::new();
            match pos_yaml_files{
                Some(yaml_files) => {
                    for yaml_file in yaml_files {
                        let yaml_file_path: String = yaml_file.display().to_string();
                        let stem: String = match extract_stem(&yaml_file_path){
                            Ok(stem) => stem,
                            Err(e) => return Err::<Option<HashMap<String,UserDataFile>>, MandyErr>(MandyErr::new(&e.to_string()))
                        };
                        let contents: String = match coutils::read_file(&yaml_file_path){
                            Ok(contents) => contents,
                            Err(e) => return Err::<Option<HashMap<String,UserDataFile>>, MandyErr>(MandyErr::new(&e.to_string()))
                        };
                        
                        let deserialized: Vec<HashMap<String,String>> = match parse_yml_data(&contents){
                            Ok(deserialized) => deserialized,
                            Err(e) => return Err::<Option<HashMap<String,UserDataFile>>, MandyErr>(MandyErr::new(&e.to_string()))
                        };
                        
                        result.insert(stem.clone(), UserDataFile { path: yaml_file, file_name: stem.clone(), file_type: UserDataFileType::YAML, contents: deserialized}); 
                        
                    }
                    Ok(Some(result))
                },
                None => return Err::<Option<HashMap<String,UserDataFile>>, MandyErr>(MandyErr::new(&e.to_string()))
            }
        }        
        else if pos_json_files.is_some(){
            let mut result: HashMap<String,UserDataFile> = HashMap::new();
            match pos_json_files{
                Some(json_files) => {
                    for json_file in json_files{
                        let json_file_path: String = json_file.display().to_string();
                        let stem: String = match extract_stem(&json_file_path){
                            Ok(stem) => stem,
                            Err(e) => return Err::<Option<HashMap<String,UserDataFile>>, MandyErr>(MandyErr::new(&e.to_string()))
                        };
                        let contents: String = match coutils::read_file(&json_file_path){
                            Ok(contents) => contents,
                            Err(e) => return Err::<Option<HashMap<String,UserDataFile>>, MandyErr>(MandyErr::new(&e.to_string()))
                        };
                        let deserialized: Vec<HashMap<String,String>> = match parse_yml_data(&contents){
                            Ok(deserialized) => deserialized,
                            Err(e) => return Err::<Option<HashMap<String,UserDataFile>>, MandyErr>(MandyErr::new(&e.to_string()))
                        };
                        result.insert(stem.clone(), UserDataFile { path: json_file, file_name: stem.clone(), file_type: UserDataFileType::JSON, contents: deserialized});  
                    }
                    Ok(Some(result))
                },
                None => { return Err::<Option<HashMap<String,UserDataFile>>, MandyErr>(MandyErr::new(&e.to_string())) }
            }
        }
        else {
            Err::<Option<HashMap<String,UserDataFile>>, MandyErr>(MandyErr::new(&e.to_string())) 
        }
    }
    else {
        Ok(None)
    }
}

/// Finds all the Markdown files in a Mandy project and parses the content
/// of these files. If this operation fails, an error is returned. A "Result" type
/// is returned.
pub fn find_markdown_files(
    dir: &String,
    config: &MandyConfig
) -> Result<HashMap<PathBuf, MandyContent>, MandyErr>{
    let mut result: HashMap<PathBuf,MandyContent> = HashMap::new();
    let pos_md_files: Option<Vec<PathBuf>> = match find_files_with_ending(dir, "markdown"){
        Ok(md_files) => md_files,
        Err(e) => return Err::<HashMap<PathBuf,MandyContent>, MandyErr>(MandyErr::new(&e.to_string()))
    };
    let md_files: Vec<PathBuf> = match pos_md_files{
        Some(md_files) => md_files,
        None => {
            let e: String = format!("No files ending in \".markdown\" found at the path \"{}\".", dir);
            return Err::<HashMap<PathBuf,MandyContent>, MandyErr>(MandyErr::new(&e.to_string()))
        }
    };
    for md_file in md_files {
        let md_path: String = md_file.display().to_string();
        let contents: String = match coutils::read_file(&md_path){
            Ok(contents) => contents,
            Err(e) => return Err::<HashMap<PathBuf,MandyContent>, MandyErr>(MandyErr::new(&e.to_string()))
        };
        let content_store: ContentStore = match parse_document(&contents){
            Ok(mandy_content) => mandy_content,
            Err(e) => return Err::<HashMap<PathBuf,MandyContent>, MandyErr>(MandyErr::new(&e.to_string()))
        };
        let modified_path: PathBuf = match insert_entity_into_buf_at_item(&md_file, &config.dist_dir, dir, &Direction::After){
            Ok(modified_path) => modified_path,
            Err(e) => return Err::<HashMap<PathBuf, MandyContent>, MandyErr>(MandyErr::new(&e.to_string()))
        };
        let complex_path: ComplexPath = match produce_complex_path(&modified_path, &config.dist_dir){
            Ok(complex_path) => complex_path,
            Err(e) => return Err::<HashMap<PathBuf, MandyContent>, MandyErr>(MandyErr::new(&e.to_string()))
        };
        let mandy_content: MandyContent = MandyContent{ content: content_store.content, layout: content_store.layout, params: content_store.params, url: complex_path.web_link, path: complex_path.on_disk_html_url };
        result.insert(md_file.clone(), mandy_content);
        
    }
    Ok(result)
}

/// Finds all the Markdown files in a given Mandy project and the given directories containing
/// loop content and parses the content of these files. If this operation fails, an error is returned. A "Result" type
/// is returned.
pub fn find_loop_content_files(
    dir: &String, 
    config: &MandyConfig
) -> Result<Option<HashMap<String, Vec<MandyContent>>>, MandyErr>{
    if config.has_loop_content{
        let loop_content_dirs: Vec<String> = match config.loop_content_dirs.clone(){
            Some(loop_content_dirs) => loop_content_dirs,
            None => { return Err::<Option<HashMap<String, Vec<MandyContent>>>,MandyErr>(MandyErr::new("The \"has_loop_content\" flag was set to \"true\" but directories containing such content were not specified.")) }
        };
        let mut result: HashMap<String,Vec<MandyContent>> = HashMap::new();
        for loop_content_dir in loop_content_dirs.clone(){
            let mut loop_content_path_buf: PathBuf = PathBuf::new();
            loop_content_path_buf.push(dir);
            loop_content_path_buf.push(loop_content_dir.clone());
            let loop_content_dir_path: String = loop_content_path_buf.display().to_string();
            let pos_md_files: Option<Vec<PathBuf>> = match find_files_with_ending(&loop_content_dir_path, "markdown"){
                Ok(pos_md_files) => pos_md_files,
                Err(e) => return Err::<Option<HashMap<String,Vec<MandyContent>>>, MandyErr>(MandyErr::new(&e.to_string()))
            };
            let md_files: Vec<PathBuf> = match pos_md_files{
                Some(md_files) => md_files,
                None => {
                    let e: String = format!("No files ending in \".markdown\" found at the path \"{}\".", &loop_content_dir_path);
                    return Err::<Option<HashMap<String, Vec<MandyContent>>>, MandyErr>(MandyErr::new(&e.to_string()))
                }
            };
            let mut mandy_content_vec: Vec<MandyContent> = Vec::new();
            for md_file in md_files {
                let md_path: String = md_file.display().to_string();
                let contents: String = match coutils::read_file(&md_path){
                    Ok(contents) => contents,
                    Err(e) => return Err::<Option<HashMap<String,Vec<MandyContent>>>, MandyErr>(MandyErr::new(&e.to_string()))
                };
                let content_store: ContentStore = match parse_document(&contents){
                    Ok(mandy_content) => mandy_content,
                    Err(e) => return Err::<Option<HashMap<String,Vec<MandyContent>>>, MandyErr>(MandyErr::new(&e.to_string()))
                };
                let modified_path: PathBuf = match insert_entity_into_buf_at_item(&md_file, &config.dist_dir, dir, &Direction::After){
                    Ok(modified_path) => modified_path,
                    Err(e) => return Err::<Option<HashMap<String,Vec<MandyContent>>>, MandyErr>(MandyErr::new(&e.to_string()))
                };
                let complex_path: ComplexPath = match produce_complex_path(&modified_path, &config.dist_dir){
                    Ok(complex_path) => complex_path,
                    Err(e) => return Err::<Option<HashMap<String, Vec<MandyContent>>>, MandyErr>(MandyErr::new(&e.to_string()))
                };
                let mandy_content: MandyContent = MandyContent{ content: content_store.content, layout: content_store.layout, params: content_store.params, url: complex_path.web_link, path: complex_path.on_disk_html_url };
                mandy_content_vec.push(mandy_content);
            }
            result.insert(loop_content_dir.clone(), mandy_content_vec);
        }
        Ok(Some(result))
    }
    else {
        Ok( None )
    }
    
    
}

/// Retrieves all the HTML template code in a Mandy project's "partials"
/// directory. A "Result" type is returned.
pub fn retrieve_partials(dir: &String) -> Result<HashMap<String, String>, MandyErr>{
    let mut partials_dir_buf: PathBuf = PathBuf::new();
    partials_dir_buf.push(dir);
    partials_dir_buf.push("partials");
    if partials_dir_buf.exists(){
        let mut result: HashMap<String, String> = HashMap::new();
        let pos_liquid_files: Option<Vec<PathBuf>> = match find_files_with_ending(&partials_dir_buf.display().to_string(), "liquid"){
            Ok(liquid_files) => liquid_files,
            Err(e) => return Err::<HashMap<String,String>, MandyErr>(MandyErr::new(&e.to_string()))
        };
        let liquid_files: Vec<PathBuf> = match pos_liquid_files{
            Some(liquid_files) => liquid_files,
            None => {
                let e: String = format!("The directory containing partial templates cannot be empty.");
                return Err::<HashMap<String,String>, MandyErr>(MandyErr::new(&e.to_string()))
            }
        };
        for item in liquid_files {
            let contents: String = match coutils::read_file(&item.display().to_string()){
                Ok(contents) => contents,
                Err(e) => return Err::<HashMap<String,String>, MandyErr>(MandyErr::new(&e.to_string()))
            };
            let stem: String = match extract_stem(&item.display().to_string()){
                Ok(stem) => stem,
                Err(e) => return Err::<HashMap<String,String>, MandyErr>(MandyErr::new(&e.to_string()))
            };
            result.insert(stem, contents);

        }
        Ok(result)
    }
    else {
        let e: String = format!("The directory for partial templates was not found.");
        Err::<HashMap<String,String>, MandyErr>(MandyErr::new(&e.to_string()))
    }
}

/// A function that gathers all the information on a Mandy project.
/// If the operation is successful, an instance of the "SiteInfo" structure 
/// is returned. If the operation fails, an error is returned.
pub fn gather_project_data(dir: &String) -> Result<SiteInfo, MandyErr> {
    let config: ConfigFile = match read_config(dir){
        Ok(config) => config,
        Err(e) => return Err::<SiteInfo, MandyErr>(MandyErr::new(&e.to_string()))
    };
    let content_files: HashMap<PathBuf, MandyContent> = match find_markdown_files(dir, &config.contents){
        Ok(content_files) => content_files,
        Err(e) => return Err::<SiteInfo, MandyErr>(MandyErr::new(&e.to_string()))
    };
    let data_files: Option<HashMap<String,UserDataFile>> = match read_data_files(dir){
        Ok(data_files) => data_files,
        Err(e) => return Err::<SiteInfo, MandyErr>(MandyErr::new(&e.to_string()))
    };
    let loop_content: Option<HashMap<String, Vec<MandyContent>>> = match find_loop_content_files(dir, &config.contents){
        Ok(loop_content) => loop_content,
        Err(e) => return Err::<SiteInfo, MandyErr>(MandyErr::new(&e.to_string()))
    };
    let layouts: Vec<LayoutFile> = match find_layout_files(dir){
        Ok(layouts) => layouts,
        Err(e) => return Err::<SiteInfo, MandyErr>(MandyErr::new(&e.to_string()))
    };
    let sass_files: Option<PathBuf> = match find_sass_files(dir){
        Ok(sass_files) => sass_files,
        Err(e) => return Err::<SiteInfo, MandyErr>(MandyErr::new(&e.to_string()))
    };
    let partials: HashMap<String,String> = match retrieve_partials(dir){
        Ok(partials) => partials,
        Err(e) => return Err::<SiteInfo, MandyErr>(MandyErr::new(&e.to_string()))
    };        
    Ok(SiteInfo { config: config, content_files: content_files, data_files: data_files, loop_content: loop_content, layouts: layouts, sass_dir: sass_files, partials: partials})
}