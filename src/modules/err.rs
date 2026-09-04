/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the I/O
/// namespace to
/// convert its errors
/// to a Mandy error.
use std::io;

/// Importing the "walkdir"
/// namespace to
/// convert its errors
/// to a Mandy error.
use walkdir;

/// Importing the "fs_extra"
/// namespace to
/// convert its errors
/// to a Mandy error.
use fs_extra;

/// Importing the "JMUErr"
/// structure to
/// convert it
/// to a Mandy error.
use jmu::JMUErr;

/// Importing the "YueErr"
/// structure to
/// convert it
/// to a Mandy error.
use yue::YueErr;

/// Importing the "tinytemplate"
/// namespace to
/// convert its errors
/// to a Mandy error.
use tinytemplate;

/// Importing the "JiraiErr"
/// structure to
/// convert it
/// to a Mandy error.
use jirai::JiraiErr;

/// Importing the "Result"
/// type because it is needed
/// by the "Display" trait.
use std::fmt::Result;

/// Importing the "Display"
/// trait to implement for
/// the error structure.
use std::fmt::Display;

/// Importing the "Error"
/// trait to implement it
/// for the error structure.
use std::error::Error;

/// Importing the "Formatter"
/// entity because it is needed
/// by the "Display" trait.
use std::fmt::Formatter;

/// Importing the "StripPrefixError"
/// to perform a conversion on it.
use std::path::StripPrefixError;

/// A data structure to
/// store information about
/// errors.
#[derive(Clone,Eq,PartialEq, Debug)]
pub struct MandyErr {
    pub details: String
}

/// Implementing
/// function(s) for
/// the `MandyErr`
/// structure.
impl MandyErr {

    /// A function to create
    /// and return a new
    /// instance of the `MandyErr`
    /// structure.
    pub fn new(
        details: &str
    ) -> MandyErr {
        MandyErr {
            details: details.to_owned()
        }
    }

}

/// Implementing the `Error`
/// trait for the `MandyErr`
/// structure.
impl Error for MandyErr {

    /// The function that
    /// implements the `Error`
    /// trait for the `MandyErr`
    /// structure.
    fn description(
        &self
    ) -> &str {
        &self.details
    }
}

/// Implementing the `Display`
/// trait for the `MandyErr`
/// structure.
impl Display for MandyErr {

    /// The function that
    /// implements the `Display`
    /// trait for the `MandyErr`
    /// structure.
    fn fmt(
        &self, 
        f: &mut Formatter
    ) -> Result {
        write!(f,"{}",self.details)
    }
}

/// Implementing the `From`
/// trait to "convert" the `io::Error`
/// structure to the `MandyErr`
/// structure.
impl From<io::Error> for MandyErr {
    fn from(error: io::Error) -> Self {
        MandyErr::new(&format!("I/O error: {:?}", error.to_string()))
    }
}

/// Implementing the `From`
/// trait to "convert" the `walkdir::Error`
/// structure to the `MandyErr`
/// structure.
impl From<walkdir::Error> for MandyErr {
    fn from(error: walkdir::Error) -> Self {
        MandyErr::new(&format!("Walkdir error: {:?}", error.to_string()))
    }
}

/// Implementing the `From`
/// trait to "convert" the `StripPrefixError`
/// structure to the `MandyErr`
/// structure.
impl From<StripPrefixError> for MandyErr{
    fn from(error: StripPrefixError) -> Self {
        MandyErr::new(&format!("String prefix error: {:?}", error.to_string()))
    }
}

/// Implementing the `From`
/// trait to "convert" the `JiraiErr`
/// structure to the `MandyErr`
/// structure.
impl From<JiraiErr> for MandyErr{
    fn from(error: JiraiErr) -> Self {
        MandyErr::new(&format!("Jirai error: {:?}", error.to_string()))
    }
}

/// Implementing the `From`
/// trait to "convert" the `fs_extra::error::Error`
/// structure to the `MandyErr`
/// structure.
impl From<fs_extra::error::Error> for MandyErr{
    fn from(error: fs_extra::error::Error) -> Self {
        MandyErr::new(&format!("Filesystem error: {:?}", error.to_string()))
    }
}

/// Implementing the `From`
/// trait to "convert" the `tinytemplate::error::Error`
/// structure to the `MandyErr`
/// structure.
impl From<tinytemplate::error::Error> for MandyErr{
    fn from(error: tinytemplate::error::Error) -> Self {
        MandyErr::new(&format!("Templating error: {:?}", error.to_string()))
    }
}

/// Implementing the `From`
/// trait to "convert" the `JMUErr`
/// structure to the `MandyErr`
/// structure.
impl From<JMUErr> for MandyErr{
    fn from(error: JMUErr) -> Self {
        MandyErr::new(&format!("JMU error: {:?}", error.to_string()))
    }
}

/// Implementing the `From`
/// trait to "convert" the `YueErr`
/// structure to the `MandyErr`
/// structure.
impl From<YueErr> for MandyErr{
    fn from(error: YueErr) -> Self {
        MandyErr::new(&format!("Yue error: {:?}", error.to_string()))
    }
}
