/*
Mandy by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the standard
/// "Result" enum.
use std::fmt::Result;

/// Importing the standard
/// "Display" trait.
use std::fmt::Display;

/// Importing the standard
/// "Error" trait.
use std::error::Error;

/// Importing the standard
/// "Formatter" trait.
use std::fmt::Formatter;

/// A data structure for
/// storing and handling errors.
#[derive(Clone,Eq,PartialEq, Debug)]
pub struct MandyErr {
    pub details: String
}

/// Implements functions
/// for the "MandyErr"
/// structure.
impl MandyErr {

    /// Implements a function to create
    /// a new instance of this data structure.
    pub fn new(details: &str) -> MandyErr {
        MandyErr {
            details: details.to_owned()
        }
    }

    /// Implements a function to return
    /// a string representation of this 
    /// data structure.
    pub fn to_string(self) -> String {
        self.details.to_string()
    }
}

/// Implements the error trait.
impl Error for MandyErr {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Implements the Display trait
/// for the "MandyErr" structure.
impl Display for MandyErr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,"{}",self.details)
    }
}