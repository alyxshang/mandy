/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the main
/// Cliply API struct.
use cliply::App;

/// Importing Mandy's error-handling
/// structure to handle errors.
use super::err::MandyErr;

/// Importing the function to 
/// clean a compiled Mandy
/// project.
use super::reset::clean_project;

/// Importing the function to 
/// compile a Mandy project.
use super::compile::compile_project;

/// Mandy's light CLI. Returns a string with 
/// feedback or an error in a "Result" type.
pub fn cli() -> Result<String, MandyErr> {

    let mut mandy: App = App::new(
        &"Mandy",
        &"0.1.0",
        &"Alyx Shang"
    );
    mandy.add_arg(
        &"comps",
        &"  compile a Mandy project", 
        &true
    );
    mandy.add_arg(
        &"reset", 
        &"  clean a compiled Mandy project", 
        &true
    );
    if mandy.version_is(){
        Ok(mandy.version_info())
    }
    else if mandy.help_is(){
        Ok(mandy.help_info())
    }
    else if mandy.arg_was_used("comps"){
        let dir: String = match mandy.get_arg_data("comps"){
            Ok(dir) => dir,
            Err(e) => return Err::<String,MandyErr>(MandyErr::new(&e.to_string()))
        };
        let del_op: String = match compile_project(&dir){
            Ok(_op) => format!("The Mandy project at \"{}\" has been compiled.", &dir),
            Err(e) => return Err::<String,MandyErr>(MandyErr::new(&e.to_string()))
        };
        Ok(del_op)
    }
    else if mandy.arg_was_used("reset"){
        let dir: String = match mandy.get_arg_data("reset"){
            Ok(dir) => dir,
            Err(e) => return Err::<String,MandyErr>(MandyErr::new(&e.to_string()))
        };
        let del_op: String = match clean_project(&dir){
            Ok(_op) => format!("The Mandy project at \"{}\" has been cleaned.", &dir),
            Err(e) => return Err::<String,MandyErr>(MandyErr::new(&e.to_string()))
        };
        Ok(del_op)
    }
    else {
        Err::<String,MandyErr>(MandyErr::new(&format!("{}", mandy.help_info())))
    }
    
}