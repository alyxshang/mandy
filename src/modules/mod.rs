/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting the module
/// to handle errors.
pub mod err;

/// Exporting the module
/// that contains Mandy's CLI.
pub mod cli;

/// Exporting the module
/// that contains Mandy's
/// entities.
pub mod units;

/// Exporting the module
/// containing some 
/// utility functions.
pub mod utils;

/// Exporting the module
/// to resets a Mandy project
/// post-compilation.
pub mod reset;

/// Exporting the module
/// for finding and detecting all
/// relevant files in a project directory.
pub mod gather;

/// Exporting the module
/// containing extra useful functions
/// that Mandy needs.
pub mod extras;

/// Exporting the module that
/// compiles a Mandy project
/// into a static site.
pub mod compile;

/// Exporting the module
/// that handles parsing and
/// processing different formats
/// of data.
pub mod processors;