/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the data
/// structure to work
/// with paths in a cross-platform
/// way.
use std::path::PathBuf;

/// Importing the data
/// structure for catching
/// and handling errors.
use super::err::MandyErr;

/// Importing the function to
/// read the string contents of
/// a file.
use std::fs::read_to_string;

/// Importing the structure to
/// make a new template store
/// and render HTML from a template
/// and a context.
use tinytemplate::TinyTemplate;

/// Importing the entity to 
/// specify to not escape HTML code
/// inside a template.
use tinytemplate::format_unescaped;

/// Importing the data structure
/// unifying page and site context
/// in one data structure to use
/// for rendering content into a
/// layout.
use super::units::SinglePageContext;

/// A function that attempts to read
/// a template at $PROJECT_DIR/layout/$NAME.layout
/// and return the template code within. If the
/// operation is successful, a string is returned.
/// If the operation fails, an error is returned.
pub fn read_template(
    name: &str,
    project_dir: &str
) -> Result<String, MandyErr>{
    let template_file: String = format!(
        "{}.layout",
        name
    );
    let mut template_buf: PathBuf = PathBuf::from(
        project_dir
    );
    template_buf.push("layouts");
    template_buf.push(template_file);
    let template: String = read_to_string(&template_buf)?;
    Ok(template)
}

/// A function that attempts to render content
/// into a string of templating code and return
/// the resulting HTML as a string. If the operation
/// is successful, this happens. In any other case,
/// an error is returned.
pub fn compile_html(
    name: &str,
    template: &str,
    context: &SinglePageContext,
) -> Result<String, MandyErr>{
    let mut tiny_t: TinyTemplate = TinyTemplate::new();
    tiny_t.set_default_formatter(&format_unescaped);
    tiny_t.add_template(name, template)?;
    let rendered = tiny_t.render(name, context)?;
    Ok(rendered)
}
