/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing Mandy's
/// CLI.
use mandy::cli;

/// The main point of 
/// entry for the Rust compiler.
fn main(){
    match cli(){
        Ok(feedback) => println!("{}", feedback),
        Err(e) => eprintln!("{}", e)
    };
}