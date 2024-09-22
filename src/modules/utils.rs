/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Path" structure
/// to extract information from
/// the filesystem.
use std::path::Path;

/// Importing the "WalkDir"
/// structure to make a new
/// instance and walk a directory.
use walkdir::WalkDir;

/// Importing the "DirEntry"
/// structure for more explicit
/// typing.
use walkdir::DirEntry;

/// Importing the "PathBuf"
/// structure to save results
/// from walking a directory.
use std::path::PathBuf;

/// Importing the "ComplexPath"
/// structure to store information
/// about file URLs.
use crate::ComplexPath;

/// Importing Mandy's error-handling
/// structure to handle errors.
use super::err::MandyErr;

/// Importing the "PathBuf"
/// structure to work with paths.
use super::units::PathInfo;

/// Importing the "Direction"
/// enum to specify in which
/// direction an insertion
/// into an instance of the "PathBuf"
/// structure should take place.
use super::units::Direction;

/// Importing the "LayoutFile"
/// structure to work with layouts
/// inside a Mandy project.
use super::units::LayoutFile;

/// Importing the "HashMap" structure from the
/// Rust standard library.
use std::collections::HashMap;

/// Importing the "UserData"
/// structure to mutate it.
use super::units::UserDataFile;

/// Searches for files ending in the supplied file extension in a directory.
/// If this operation succeeds, a vector containing instances of the "PathBuf" structure is
/// returned. If this operation fails, an error is returned. A "Result" type is
/// returned.
pub fn find_files_with_ending(project_dir: &String, ext: &str) -> Result<Option<Vec<PathBuf>>, MandyErr>{
    let mut res: Vec<PathBuf> = Vec::new();
    let parent = WalkDir::new(project_dir);
    for entry in parent{
        let dir_entry: DirEntry = match entry{
            Ok(dir_entry) => dir_entry,
            Err(e) => return Err::<Option<Vec<PathBuf>>, MandyErr>(MandyErr::new(&e.to_string()))
        };
        if dir_entry.file_type().is_file(){
            let path = dir_entry.path();
            match path.extension(){
                Some(file_ext) => {
                    let my_ext = file_ext.to_str();
                    match my_ext {
                        Some(cleaned) => {
                            if cleaned == ext{
                                let mut buf = PathBuf::new();
                                buf.push(path);
                                res.push(buf);
                            }
                            else {}
                        },
                        None => {}
                    } 
                }
                None => {}
            };
        }
        else {}
    }
    if res.is_empty(){ Ok(None) }
    else { Ok(Some(res)) }
}

/// Extracts the stem of a filename given the string from an instance
/// of a "PathBuf" structure. Returns an error if this operation fails.
/// A "Result" type is returned.
pub fn extract_stem(subject: &String) -> Result<String, MandyErr>{
    let file = match Path::new(&subject).file_name(){
        Some(file) => file.to_str(),
        None => {
            let e: String = format!("Could not extract filename from path \"{}\"!", &subject);
            return Err::<String, MandyErr>(MandyErr::new(&e.to_string()))
        }
    };
    let fname: &str = match file{
        Some(fname) => fname,
        None => {
            let e: String = format!("Could not extract filename from path \"{}\"!", &subject);
            return Err::<String, MandyErr>(MandyErr::new(&e.to_string()))
        }
    };
    let stem_opt = match Path::new(fname).file_stem(){
        Some(stem_opt) => stem_opt,
        None => {
            let e: String = format!("Could not extract filename from path \"{}\"!", &fname);
            return Err::<String, MandyErr>(MandyErr::new(&e.to_string()))
        }
    };
    let stem: &str = match stem_opt.to_str(){
        Some(stem) => stem,
        None => {
            let e: String = format!("Could not extract filename from path \"{}\"!", &subject);
            return Err::<String, MandyErr>(MandyErr::new(&e.to_string()))
        }
    };
    Ok(stem.to_string())

}

/// Returns a "Result" type containing either an instance of the "PathInfo"
/// structure or an error. This functions attempts to extract the file stem
/// and the parent path of the given path. Returns an error if this operation fails.
pub fn get_path_from_buf(subject: &PathBuf) -> Result<PathInfo, MandyErr>{
    let parent: PathBuf = match subject.parent(){
        Some(par) => par.to_path_buf(),
        None => {
            let e: String = format!("Could not retrieve parent directory of path \"{}\".", subject.display().to_string());
            return Err::<PathInfo, MandyErr>(MandyErr::new(&e.to_string()))
        }
    };
    let stem: String = match extract_stem(&subject.display().to_string()){
        Ok(stem) => stem,
        Err(e) => return Err::<PathInfo, MandyErr>(MandyErr::new(&e.to_string()))
    };
    Ok(PathInfo{ path: parent, file: stem})
}

/// Retrieves a layout with a certain name from a vector
/// of instances of the "LayoutFile" structure. A "Result"
/// type is returned.
pub fn get_layout_by_name(layout: &String, layouts: &Vec<LayoutFile>) -> Result<LayoutFile, MandyErr>{
    let mut requested: Vec<LayoutFile> = Vec::new();
    for user_layout in layouts{
        if layout == &user_layout.name {
            requested.push(user_layout.to_owned());
        }
        else {}
    }
    if requested.is_empty(){
        let e: String = format!("The requested layout \"{}\" could not be found.", layout);
        return Err::<LayoutFile, MandyErr>(MandyErr::new(&e.to_string()));
    }
    else {
        Ok(requested[0].clone())
    }
}

/// This function attempts to split an instance
/// of the "PathBuf" structure into a vector of strings.
/// A "Result" type is returned.
pub fn split_path_buf(buf: &PathBuf) -> Result<Vec<String>, MandyErr>{
    let mut result: Vec<String> = Vec::new();
    let buf_components = buf.as_path().components();
    for component in buf_components{
        let component_string: String = match component.as_os_str().to_str(){
            Some(comp) => comp.to_string(),
            None => return Err::<Vec<String>, MandyErr>(MandyErr::new(&format!("Error splitting path \"{}\".",&buf.display().to_string())))
        };
        result.push(component_string);
    }
    Ok(result)
}

/// Makes a vector of strings into an instance of the 
/// "PathBuf" structure.
pub fn string_vec_to_path_buf(subject: &Vec<String>) -> PathBuf{
    let mut sub_buf: PathBuf = PathBuf::new();
    for item in subject{
        sub_buf.push(item);
    }
    sub_buf
}

/// Attempts to split an instance of the "PathBuf" structure at the given item.
/// A "Result" type is returned.
pub fn split_buf_at_item(buf: &PathBuf, item: &String, including: &bool) -> Result<PathBuf, MandyErr>{
    let buf_items: Vec<String> = match split_path_buf(buf){
        Ok(buf_items) => buf_items,
        Err(e) => return Err::<PathBuf, MandyErr>(MandyErr::new(&e.to_string()))
    };
    let mut cloned: Vec<String> = buf_items.clone();
    for (index,buf_item) in buf_items.iter().enumerate(){
        if buf_item == item && *including{
            let idx_to_be_removed: usize = index - 1;
            if coutils::has_index(&buf_items, &idx_to_be_removed){
                cloned.remove(index-1);
                break;
            }
            else {
                let e: String = format!("The array \"{:?}\" is too short to remove index \"{}\"!", &buf_items, &idx_to_be_removed);
                return Err::<PathBuf, MandyErr>(MandyErr::new(&e.to_string()))
            }
        }
        else if buf_item == item && !*including{
            break;
        }
        else {
            cloned.remove(index);
        }
    }
    let result: PathBuf = string_vec_to_path_buf(&cloned);
    Ok(result)
}

/// Attempts to insert an entity into an instance of the 
/// "PathBuf" structure at the given item.
/// A "Result" type is returned.
pub fn insert_entity_into_buf_at_item(
    buf: &PathBuf,
    entity: &String, 
    item: &String,
    direction: &Direction
) -> Result<PathBuf, MandyErr>{
    let item_vec: Vec<String> = match split_path_buf(buf){
        Ok(item_vec) => item_vec,
        Err(e) => return Err::<PathBuf, MandyErr>(MandyErr::new(&e.to_string()))
    };
    let mut cloned: Vec<String> = item_vec.clone();
    let item_idx: usize = match coutils::get_index(&item_vec, item){
        Ok(item_idx) => item_idx,
        Err(e) => return Err::<PathBuf, MandyErr>(MandyErr::new(&e.to_string()))
    };
    match direction {
        Direction::Before => {
            let new_idx: usize = item_idx - 1;
            cloned.insert(new_idx, entity.to_owned());
        },
        Direction::After => {
            let new_idx: usize = item_idx + 1;
            cloned.insert(new_idx, entity.to_owned());
        }
    };
    let result: PathBuf = string_vec_to_path_buf(&cloned);
    Ok(result)
}

/// Cleans and converts a series of the "UserDataFile" structure into a vector of string maps.
/// A "Result" type is returned.
pub fn clean_data(subject: &HashMap<String, UserDataFile>) -> HashMap<String, Vec<HashMap<String,String>>>{
    let mut result: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    for (_key,value) in subject{
        let new_key: String = value.file_name.clone();
        let new_value: Vec<HashMap<String,String>> = value.contents.clone();
        result.insert(new_key, new_value);
    }
    result
}

/// Attempts to return a string containing an instance of the "PathBuf"
/// structure that has been reformatted to be an URL path.
pub fn make_web_friendly(path: &PathBuf) -> Result<String, MandyErr> {
    let strings: Vec<String> = match split_path_buf(path){
        Ok(strings) => strings,
        Err(e) => return Err::<String,MandyErr>(MandyErr::new(&e.to_string()))
    };
    let orig: String = strings.join("/");
    Ok(format!("/{}", orig))    
}

/// Attempts to prepare an instance of the "PathBuf" structure linking to a content file
/// for use with routing to HTML pages. A "Result" type is returned.
pub fn produce_complex_path(path: &PathBuf, dist_dir: &String) -> Result<ComplexPath, MandyErr>{
    
    // Cleaning up the path of the HTML file on disk.
    let stem: String = match extract_stem(&path.display().to_string()){
        Ok(stem) => stem,
        Err(e) => return Err::<ComplexPath,MandyErr>(MandyErr::new(&e.to_string()))
    };
    let mut orig_path_clone: PathBuf = path.clone();
    orig_path_clone.set_extension("");
    let mut new_path_buf: PathBuf = orig_path_clone;
    if stem == "index".to_string(){
        new_path_buf.set_extension("html");
    }
    else {
        new_path_buf.push("index.html");
    }

    // Cleaning up the path of the weblink.
    let split_buf: PathBuf = match split_buf_at_item(&new_path_buf.clone(), dist_dir, &true){
        Ok(split_buf) => split_buf,
        Err(e) => return Err::<ComplexPath,MandyErr>(MandyErr::new(&e.to_string()))
    };
    let web_link: String = match make_web_friendly(&split_buf){
        Ok(web_link) => web_link,
        Err(e) => return Err::<ComplexPath,MandyErr>(MandyErr::new(&e.to_string()))
    };
    Ok(ComplexPath{ on_disk_html_url: new_path_buf.display().to_string(), web_link: web_link})
}