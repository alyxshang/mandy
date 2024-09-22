/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "serde_yml"
/// library to deserialize YML data.
use serde_yml;

/// Importing the "serde_json"
/// library to deserialize JSON data.
use serde_json;

/// Importing the "object""
/// macro from the "liquid"
/// crate to supply context
/// to templates.
use liquid::object;

/// Importing the "Parser"
/// structure from the "liquid"
/// crate to parse templates.
use liquid::Parser;

/// Importing the "Template"
/// structure from the "liquid"
/// crate to render templates.
use liquid::Template;

/// Importing the function to 
/// parse Markdown code into
/// HTML code.
use markdown::to_html;

/// Importing Mandy's error-handling
/// structure to handle errors.
use super::err::MandyErr;

/// Importing the "ParserBuilder"
/// structure from the "liquid"
/// crate to parse Liquid code.
use liquid::ParserBuilder;

/// Importing Rust's standard API
/// for working with maps.
use std::collections::HashMap;

/// Importing the "SiteContext" structure
/// to use as context for templates
/// written in the Liquid templating
/// language.
use super::units::SiteContext;

/// Importing the "MandyConfig"
/// structure to deserialize YML or
/// JSON data into it.
use super::units::MandyConfig;

/// Importing the
/// structure to store content
/// parsed from a content file
/// written in Markdown.
use super::units::ContentStore;

/// Importing the function to deserialize
/// Markdown documents into a Rust
/// data structure.
use serde_frontmatter::deserialize;

/// Importing the "EagerCompiler"
/// entity to parse and process
/// partial HTML templates.
use liquid::partials::EagerCompiler;

/// Importing the "InMemorySource"
/// entity to parse and process
/// partial HTML templates.
use liquid::partials::InMemorySource;


/// Parses and deserializes source from a JSON data file into a 
/// "HashMap". Returns an error if this fails. A "Result" type is returned.
pub fn parse_json_data(src: &String) -> Result<Vec<HashMap<String,String>>, MandyErr>{
    let data: Vec<HashMap<String,String>> = match serde_json::from_str(src){
        Ok(data) => data,
        Err(e) => return Err::<Vec<HashMap<String,String>>, MandyErr>(MandyErr::new(&e.to_string()))
    };
    Ok(data)
}

/// Parses and deserializes source from a YAML data file into a 
/// "HashMap". Returns an error if this fails. A "Result" type is returned.
pub fn parse_yml_data(src: &String) -> Result<Vec<HashMap<String,String>>, MandyErr>{
    let data: Vec<HashMap<String, String>> = match serde_yml::from_str(src){
        Ok(data) => data,
        Err(e) => return Err::<Vec<HashMap<String, String>>, MandyErr>(MandyErr::new(&e.to_string()))
    };
    Ok(data)
}

/// Parses and deserializes source from a YAML config file into an instance 
/// of the "MandyConfig" structure. Returns an error if this fails. 
/// A "Result" type is returned.
pub fn parse_yml_config(src: &String) -> Result<MandyConfig, MandyErr>{
    let data: MandyConfig = match serde_yml::from_str(src){
        Ok(data) => data,
        Err(e) => return Err::<MandyConfig, MandyErr>(MandyErr::new(&e.to_string()))
    };
    Ok(data)
}

/// Parses and deserializes source from a JSON config file into an instance 
/// of the "MandyConfig" structure. Returns an error if this fails. 
/// A "Result" type is returned.
pub fn parse_json_config(src: &String) -> Result<MandyConfig, MandyErr>{
    let data: MandyConfig = match serde_json::from_str(src){
        Ok(data) => data,
        Err(e) => return Err::<MandyConfig, MandyErr>(MandyErr::new(&e.to_string()))
    };
    Ok(data)
}

/// Parses and deserializes source from a Markdown content file into an instance 
/// of the "ContentStore" structure. Returns an error if this fails. 
/// A "Result" type is returned.
pub fn parse_document(src: &String) -> Result<ContentStore, MandyErr>{
    let (data,content): (HashMap<String,String>,String) = match deserialize::<HashMap<String,String>>(src){
        Ok(data) => data,
        Err(_e) => {
            let e: String = format!("Error parsing \"{}\".", src);
            return Err::<ContentStore, MandyErr>(MandyErr::new(&e))
        }
    };
    if data.contains_key("layout"){
        Ok(ContentStore{ layout: data["layout"].clone(), params: data, content: to_html(&content) })
    }
    else {
        let e: String = format!("The \"layout\" variable was not set in the following Markdown source code: \"{}\".", src);
        Err::<ContentStore, MandyErr>(MandyErr::new(&e))
    }
}

/// Processes Liquid code with an instance
/// of the "SiteContext" structure and outputs HTML code.
/// Returns an error if this operation fails.
/// A "Result" type is returned.
pub fn process_liquid(liquid_code: &String, ctx: &SiteContext, partials: &HashMap<String,String>) -> Result<String, MandyErr>{
    type Partials =  EagerCompiler<InMemorySource>;
    let mut partial_source = Partials::empty();
    for (k,v) in partials.into_iter() {
        partial_source.add(k,v);
    }
    let parser: Parser = match ParserBuilder::with_stdlib().partials(partial_source).build(){
        Ok(parser) => parser,
        Err(e) => return Err::<String, MandyErr>(MandyErr::new(&e.to_string()))
    };
    let parsed: Template = match parser.parse(liquid_code){
        Ok(parsed) => parsed,
        Err(e) => return Err::<String, MandyErr>(MandyErr::new(&e.to_string()))
    };
    let globals = object!(ctx);
    let html: String = match parsed.render(&globals){
        Ok(html) => html,
        Err(e) => return Err::<String, MandyErr>(MandyErr::new(&e.to_string()))
    };
    Ok(html)
}