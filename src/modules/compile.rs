/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing all entities
/// from the "grass" crate
/// to compile SASS.
use grass;

/// Importing the "coutils" crate
/// for various file-related operations.
use coutils;

/// Importing the standard library  
/// for working with the filesystem.
use std::fs;

/// Importing the function to read
/// environment variables from
/// Rust's standard library.
use std::env::var;

/// Importing the "PathBuf"
/// structure from Rust's standard
/// library.
use std::path::PathBuf;

/// Importing the function
/// to generate files for SEO
/// in a compiled Mandy project.
use super::extras::seo;

/// Importing Mandy's error-handling
/// structure to handle errors.
use super::err::MandyErr;

/// Importing the "PathInfo"
/// structure to store information
/// about paths.
use super::units::PathInfo;

/// Importing the "SiteInfo"
/// structure to store information
/// about the Mandy project.
use super::units::SiteInfo;

/// Importing the "Direction"
/// enum to specify in which
/// direction an insertion
/// into an instance of the
/// "PathBuf" structure should take
/// place.
use super::units::Direction;

/// Importing the "LayoutFile"
/// structure to save information
/// about detected files with
/// HTML templates in them.
use super::units::LayoutFile;

/// Importing the function to
/// clean specified user data.
use super::utils::clean_data;

/// Importing the structure
/// from Rust's standard library
/// to work with maps.
use std::collections::HashMap;

/// Importing the structure that supplies
/// the context needed for compiling
/// HTML templates into HTML.
use super::units::SiteContext;

/// Importing the structure to store
/// information about content files
/// in a Mandy project.
use super::units::MandyContent;

/// Importing the function to split an
/// instance of the "PathBuf" structure.
use super::utils::split_path_buf;

/// Importing the function to 
/// find all files containing SASS
/// code in a Mandy project.
use super::gather::find_sass_files;

/// Importing the function to split an
/// instance of the "PathBuf" structure
/// at the given item.
use super::utils::split_buf_at_item;

/// Importing the function to get the full parent
/// path from an instance of the "PathBuf" structure.
use super::utils::get_path_from_buf;

/// Importing the function get a layout
/// by the supplied name.
use super::utils::get_layout_by_name;

/// Importing the function to process HTML
/// templates.
use super::processors::process_liquid;

/// Importing the function that 
/// gathers information about the Mandy project
/// in the supplied directory.
use super::gather::gather_project_data;

/// Importing the function to find
/// all loop content files.
use super::gather::find_loop_content_files;

/// Importing the function to insert an entity
/// into an instance of the "PathBuf" structure
/// at a given item.
use super::utils::insert_entity_into_buf_at_item;

/// Compiles a Mandy project. Returns a string if the operation succeeds.
/// If the operation fails, an error is returned.
pub fn compile_project(dir: &String) -> Result<(), MandyErr>{
    if coutils::dir_is(&dir){
        let data: SiteInfo = match gather_project_data(dir){
            Ok(data) => data,
            Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
        };
        let dist_dir: &String = &data.config.contents.dist_dir;
        let mut dist_dir_buf: PathBuf = PathBuf::new();
        dist_dir_buf.push(dir);
        dist_dir_buf.push(dist_dir);
        let _create_dist_dir_op: () = match coutils::create_directory(&dist_dir_buf.display().to_string()){
            Ok(_create_dist_dir_op) => _create_dist_dir_op,
            Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
        };
        let _compile_md: () = match compile_markdown_files(dir){
            Ok(_compile_md) => _compile_md,
            Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
        };
        let _copy_files: () = match copy_files(dir){
            Ok(_copy_files) => _copy_files,
            Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
        };
        let _compile_sass: () = match compile_sass(dir){
            Ok(_compile_sass) => _compile_sass,
            Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
        };
        let _compile_loop_content: () = match compile_loop_content_files(dir){
            Ok(_compile_loop_content) => _compile_loop_content,
            Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
        };
        let seo_op: () = match seo(dir, &data.content_files){
            Ok(seo) => seo,
            Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
        };
        Ok(seo_op)
    }
    else {
        let e: String = format!("The directory \"{}\" does not exist.", dir);
        return Err::<(), MandyErr>(MandyErr::new(&e.to_string()));
    }
}

/// Compiles all content files in a Mandy project written in Markdown
/// that a Mandy project has. Returns a "Result" type with an empty closure.
pub fn compile_markdown_files(dir: &String) -> Result<(), MandyErr>{
    let data: SiteInfo = match gather_project_data(dir){
        Ok(data) => data,
        Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
    };
    let dist_dir: &String = &data.config.contents.dist_dir;
    let content_files: HashMap<PathBuf, MandyContent> = data.content_files;
    let baseurl_var: String = match var("MANDY_ENV"){
        Ok(baseurl_var) => baseurl_var,
        Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
    };
    let env_baseurl: String;
    if baseurl_var == "production"{
        env_baseurl = data.config.contents.prod_url.clone();
    }
    else if baseurl_var == "development"{
        env_baseurl = data.config.contents.dev_url.clone();
    }
    else {
        let e: String = format!("The environment variable \"$MANDY_ENV\" must be set to either \"production\" or \"development\"!");
        return Err::<(), MandyErr>(MandyErr::new(&e.to_string()));
    }
    for (content_file_path,content) in content_files {
        let path_info: PathInfo = match get_path_from_buf(&content_file_path){
            Ok(path_info) => path_info,
            Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
        };
        let with_dist: PathBuf = match insert_entity_into_buf_at_item(&path_info.path, dist_dir, dir, &Direction::After){
            Ok(with_dist) => with_dist,
            Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
        };
        let mut subdirs: PathBuf = match split_buf_at_item(&with_dist, dir, &false){
            Ok(subdirs) => subdirs,
            Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
        };
        if &path_info.path.display().to_string() == dir{}
        else {
            subdirs.push(path_info.file);
        }
        let subdirs_items: Vec<String> = match split_path_buf(&subdirs){
            Ok(subdirs_items) => subdirs_items,
            Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
        };
        let mut subdirs_items_buf: PathBuf = PathBuf::new();
        for subdir_item in subdirs_items.clone(){
            subdirs_items_buf.push(subdir_item);
            if subdirs_items_buf.exists(){}
            else {
                let _create_op: () = match coutils::create_directory(&subdirs_items_buf.display().to_string()){
                    Ok(_create_op) => _create_op,
                    Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
                };
            }
        }
        subdirs_items_buf.push("index.html");
        if subdirs_items_buf.exists(){
            let e: String = format!("Filesystem at \"{}\" already exists.", &subdirs_items_buf.display().to_string());
            return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
        }
        else {
            let _create_html_op: () = match coutils::create_file(&subdirs_items_buf.display().to_string()){
                Ok(_create_html_op) => _create_html_op,
                Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
            };
            let liquid_template: LayoutFile = match get_layout_by_name(&content.layout, &data.layouts){
                Ok(liquid_template) => liquid_template,
                Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
            };
            let compile_ctx: SiteContext;
            match &data.loop_content{
                Some(loop_content) => {
                    match &data.data_files{
                        Some(site_data) => {
                            let compile_data = clean_data(site_data);
                            compile_ctx = SiteContext { site: data.config.contents.clone(), page: content, loop_content: Some((*loop_content).clone()), data: Some(compile_data), baseurl: env_baseurl.clone()}
                        },
                        None => {
                            compile_ctx = SiteContext { site: data.config.contents.clone(), page: content, loop_content: Some(loop_content.clone()), data: None, baseurl: env_baseurl.clone()}
                        }
                    }
                    
                },
                None => {
                    match &data.data_files{
                        Some(site_data) => {
                            let compile_data = clean_data(&site_data);
                            compile_ctx = SiteContext { site: data.config.contents.clone(), page: content, loop_content: None, data: Some(compile_data), baseurl: env_baseurl.clone()}
                        },
                        None => {
                            compile_ctx = SiteContext { site: data.config.contents.clone(), page: content, loop_content: None, data: None, baseurl: env_baseurl.clone()}
                        }
                    }
                }
            }
            let html: String = match process_liquid(&liquid_template.contents, &compile_ctx, &data.partials){
                Ok(html) => html,
                Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
            };
            let _write_op: () = match coutils::write_to_file(&subdirs_items_buf.display().to_string(), &html){
                Ok(_write_op) => _write_op,
                Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
            };            
        }    
    }
    Ok(())
}

/// Compiles all SASS files the user of Mandy has in their project
/// directory. Returns a "Result" type with an empty closure.
pub fn compile_sass(dir: &String) -> Result<(), MandyErr>{
    let data: SiteInfo = match gather_project_data(dir){
        Ok(data) => data,
        Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
    };
    let dist_dir: &String = &data.config.contents.dist_dir;
    let sass_files: Option<PathBuf> = match find_sass_files(dir){
        Ok(sass_files) => sass_files,
        Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
    };
    match sass_files {
        Some(sass_dir) => {
            let css: String = match grass::from_path(sass_dir.display().to_string(), &grass::Options::default()){
                Ok(css) => css,
                Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
            };
            let mut css_dir_buf: PathBuf = PathBuf::new();
            css_dir_buf.push(dir);
            css_dir_buf.push(dist_dir);
            css_dir_buf.push("css");
            let _create_dir: () = match coutils::create_directory(&css_dir_buf.display().to_string()){
                Ok(_create_dir) => _create_dir,
                Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
            };
            let mut css_file_buf: PathBuf = PathBuf::new();
            css_file_buf.push(dir);
            css_file_buf.push(dist_dir);
            css_file_buf.push("css");
            css_file_buf.push("index.css");
            let _create_css_file: () = match coutils::create_file(&css_file_buf.display().to_string()){
                Ok(_create_css_file) => _create_css_file,
                Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
            };
            let write_css: () = match coutils::write_to_file(&css_file_buf.display().to_string(), &css){
                Ok(write_css) => write_css,
                Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
            };
            return Ok(write_css);
        },
        None => return Ok(())
    };
}

/// Copies any static assets from the project directory to
/// the directory containing the compiled Mandy project.
/// Returns a "Result" type with an empty closure.
pub fn copy_files(dir: &String) -> Result<(), MandyErr>{
    let data: SiteInfo = match gather_project_data(dir){
        Ok(data) => data,
        Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
    };
    if data.config.contents.copy_files{
        let dist_dir: &String = &data.config.contents.dist_dir;
        let copy_entities: &Option<Vec<String>> = &data.config.contents.copy_entities;
        match copy_entities{
            Some(entities) => {
                for entity in entities{
                    let mut old_path_buf: PathBuf = PathBuf::new();
                    let mut new_path_buf: PathBuf = PathBuf::new();
                    old_path_buf.push(dir);
                    old_path_buf.push(entity);
                    if old_path_buf.exists() && old_path_buf.is_dir() {
                        new_path_buf.push(dir);
                        new_path_buf.push(dist_dir);
                        let _dir_copy_op: () = match coutils::folder_copy(&old_path_buf.display().to_string(), &new_path_buf.display().to_string()){
                            Ok(_dir_copy_op) => _dir_copy_op,
                            Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
                        };
                    }
                    else if old_path_buf.exists() && old_path_buf.is_file() {
                        new_path_buf.push(dir);
                        new_path_buf.push(dist_dir);
                        new_path_buf.push(entity);
                        let _copy_op = match fs::copy(old_path_buf, new_path_buf){
                            Ok(_copy_op) => _copy_op,
                            Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
                        };
                    }
                    else {
                        let e: String = format!("The file at the path \"{}\" could not be found!", &old_path_buf.display().to_string());
                        return Err::<(), MandyErr>(MandyErr::new(&e.to_string()));
                    }
                }
                return Ok(());
            },
            None => {
                let e: String = format!("The \"copy_files\" option was set to \"true\" but no entities were supplied.");
                return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
            }
        }
       
    }
    else { return Ok(()); }
    
}

/// Compiles any loop content the user may have specified.
/// Returns a "Result" type with an empty closure.
pub fn compile_loop_content_files(dir: &String) -> Result<(), MandyErr>{
    let data: SiteInfo = match gather_project_data(dir){
        Ok(data) => data,
        Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
    };
    let loop_content = match find_loop_content_files(dir, &data.config.contents){
        Ok(loop_content) => loop_content,
        Err(e) => return Err::<(), MandyErr>(MandyErr::new(&e.to_string()))
    };
    let alc: HashMap<String,Vec<MandyContent>> = match loop_content{
        Some(alc) => alc,
        None => return Ok(())
    };
    let mut file_vec: Vec<String> = Vec::new();
    for (_key, value) in alc.iter().enumerate(){
        let (_direct,content) = value;
        for path in content {
            file_vec.push(path.path.clone());
        }
    }
    for file in file_vec{
        let file_clone: String = file.clone();
        let mut new_path_buf: PathBuf = PathBuf::new();
        new_path_buf.push(file);
        if new_path_buf.exists(){}
        else {
            let e: String = format!("The following file from the loop content directories could not be generated: \"{}\"", file_clone);
            return Err::<(), MandyErr>(MandyErr::new(&e.to_string()));
        }
    }
    Ok(())
}