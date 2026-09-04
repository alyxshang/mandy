/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the `App`
/// structure from the
/// "yue" crate to make
/// a new CLI app.
use yue::App;

/// Importing the data
/// structure for catching
/// and handling errors.
use super::err::MandyErr;

/// Importing the function to
/// clean a compiled Mandy project,
/// given the project's path.
use super::clear::clear_project;

/// Importing the data structure
/// containing the data that is
/// written to disk.
use super::units::CompiledContent;

/// Importing the function to
/// compile a Mandy project,
/// given the project's path.
use super::compiler::build_project;

/// Importing the function that reads
/// all data inside a Mandy project.
use super::gather::gather_content_data;

/// A function containing Mandy's CLI.
/// If there are no failures, a string
/// with a short message is returned.
/// If there are failures, an error
/// is returned.
pub fn cli(
) -> Result<String, MandyErr>{
    let mut mandy: App = App::new(
        "Mandy",
        "0.2.1"
    );
    mandy.add_arg(
        "build",
        &true,
        "builds a Mandy project",
    )?;
    mandy.add_arg(
        "test",
        &true,
        "tests whether a Mandy project can be built",
    )?;
    mandy.add_arg(
        "clean",
        &true,
        "cleans a Mandy project",
    )?;
    mandy.add_arg(
        "version",
        &false,
        "displays version information",
    )?;
    mandy.add_arg(
        "help",
        &false,
        "displays usage information",
    )?;
    let mut supplied_args: Vec<String> = std::env::args()
        .collect::<Vec<String>>();
    if supplied_args.len() > 1{
        supplied_args.remove(0);
        let _p: () = mandy.parse_args(&supplied_args)?;
        if mandy.arg_used("clean"){
            let data: String = mandy.get_arg_data("clean")?;
            let msg: String = format!(
                "Project at path \"{}\" cleaned.",
                data
            );
            let _c: () = clear_project(&data)?;
            Ok(msg)
        }
        else if mandy.arg_used("build"){
            let data: String = mandy.get_arg_data("build")?;
            let msg: String = format!(
                "Project at path \"{}\" built.",
                data
            );
            let _c: () = build_project(&data)?;
            Ok(msg)
        }
        else if mandy.arg_used("test"){
            let data: String = mandy.get_arg_data("test")?;
            let gathered: Vec<CompiledContent> = gather_content_data(&data)?;
            let msg: String = format!(
                "Project at path \"{}\" with {} files tested.",
                data,
                gathered.len()
            );
            Ok(msg)
        }
        else if mandy.arg_used("version"){
            Ok(mandy.version_info())
        }
        else if mandy.arg_used("help"){
            Ok(mandy.help_info()?)
        }
        else {
            let e: String = "Unrecognized argument(s).\nUse the \"--help\" flag.".to_string();
            Err::<String, MandyErr>(
                MandyErr::new(&e.to_string())
            )
        }
    }
    else {
        let e: String = "Unrecognized argument(s).\nUse the \"--help\" flag.".to_string();
        Err::<String, MandyErr>(
            MandyErr::new(&e.to_string())
        )
    }
}
