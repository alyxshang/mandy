/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the function
/// containing Mandy's CLI.
use mandy::cli;

/// Importing the function to
/// set an exit status for
/// success or failure.
use std::process::exit;

/// The main point
/// of entry for the
/// Rust compiler.
fn main(){ 
    match cli(){
        Ok(f) => {
            println!("{}", f);
            exit(0);
        },
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
}
