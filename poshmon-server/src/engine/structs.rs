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
pub struct DataFieldNotFoundError;

impl fmt::Display for DataFieldNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The data field could not be found")
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