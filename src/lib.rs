/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// to handle errors.
pub use modules::err::*;

/// Re-exporting the module
/// that contains Mandy's CLI.
pub use modules::cli::*;

/// Re-exorting the module
/// that contains Mandy's
/// entities.
pub use modules::units::*;

/// Re-exporting the module
/// containing some 
/// utility functions.
pub use modules::utils::*;

/// Re-exporting the module
/// for finding and detecting all
/// relevant files in a project directory.
pub use modules::gather::*;

/// Re-exporting the module
/// containing extra useful functions
/// that Mandy needs.
pub use modules::extras::*;

/// Re-exporting the module that
/// compiles a Mandy project
/// into a static site.
pub use modules::compile::*;

/// Re-exporting the module
/// that handles parsing and
/// processing different formats
/// of data.
pub use modules::processors::*;