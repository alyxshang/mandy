/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting the module
/// containing Mandy's CLI.
pub mod cli;

/// Exporting the module
/// containing the structure
/// responsible for handling
/// and catching errors.
pub mod err;

/// Exporting the module
/// containing the function
/// to read content that
/// should be loopable.
pub mod iter;

/// Exporting the module
/// containing the function
/// to read data files from 
/// the `data` directory
/// inside a Mandy project.
pub mod data;

/// Exporting the
/// module containing
/// functions to copy
/// any static files
/// over to the compiled
/// Mandy project.
pub mod copy;

/// Exporting the module
/// containing all of
/// Mandy's important
/// data structures.
pub mod units;

/// Exporting the module
/// containing a function
/// to clean a compiled
/// Mandy project.
pub mod clear;

/// Declaring the module
/// containing this crate's
/// unit tests.
#[cfg(test)]
pub mod tests;

/// Exporting the module
/// containing the function
/// to read configuration
/// values of a Mandy project.
pub mod config;

/// Exporting the module
/// containing functions
/// to read templates and
/// compile templates.
pub mod render;

/// Exporting the module
/// containing the function
/// to read an Extended Jirai
/// file, deserialize the 
/// contents, and return this
/// information.
pub mod ejirai;

/// Exporting the module
/// containing the functions
/// to gather information needed
/// to compile a Mandy project.
pub mod gather;

/// Exporting the module 
/// containing the functions
/// to compile a Mandy project
/// into a built site.
pub mod compiler;
