use std::{fmt, error::Error};
//Errors
#[derive(Debug, Clone)]
pub struct PokemonNotFoundError;

impl fmt::Display for PokemonNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pokemon not found")
    }
}

impl Error for PokemonNotFoundError {}

#[derive(Debug, Clone)]
pub struct DataFieldNotFoundError {
    field: String,
}

impl DataFieldNotFoundError {
    pub fn new(field: &str) -> Self {
        DataFieldNotFoundError { field: field.to_string() }
    }
}

impl fmt::Display for DataFieldNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The data field {} could not be found", self.field)
    }
}

impl Error for DataFieldNotFoundError {}

#[derive(Debug, Clone)]
pub struct DataLockError;

impl fmt::Display for DataLockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The data could not be locked")
    }
}

impl Error for DataLockError {}