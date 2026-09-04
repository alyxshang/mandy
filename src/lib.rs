/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

#![deny(clippy::all)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::borrowed_box)]
#![allow(clippy::redundant_field_names)]

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// containing Mandy's CLI.
pub use modules::cli::*;

/// Re-exporting the module
/// containing the structure
/// responsible for handling
/// and catching errors.
pub use modules::err::*;

/// Re-exporting the module
/// containing the function
/// to read content that
/// should be loopable.
pub use modules::iter::*;

/// Exporting the module
/// containing the function
/// to read data files from 
/// the `data` directory
/// inside a Mandy project.
pub use modules::data::*;

/// Re-exporting the
/// module containing
/// functions to copy
/// any static files
/// over to the compiled
/// Mandy project.
pub use modules::copy::*;

/// Re-exporting the module
/// containing all of
/// Mandy's important
/// data structures.
pub use modules::units::*;

/// Re-exporting the module
/// containing a function
/// to clean a compiled
/// Mandy project.
pub use modules::clear::*;

/// Re-exporting the module
/// containing the function
/// to read configuration
/// values of a Mandy project.
pub use modules::config::*;

/// Re-exporting the module
/// containing functions
/// to read templates and
/// compile templates.
pub use modules::render::*;

/// Re-exporting the module
/// containing the function
/// to read an Extended Jirai
/// file, deserialize the 
/// contents, and return this
/// information.
pub use modules::ejirai::*;

/// Re-exporting the module
/// containing the functions
/// to gather information needed
/// to compile a Mandy project.
pub use modules::gather::*;

/// Re-exporting the module 
/// containing the functions
/// to compile a Mandy project
/// into a built site.
pub use modules::compiler::*;
