use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum InputError {
    TooBigError,
    TooSmallError,
    ContainsNotRomanNumeral,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InputError::TooBigError => write!(f, "Integers above 3999 are not allowed."),
            InputError::TooSmallError => write!(f, "Integers below 1 are not allowed."),
            InputError::ContainsNotRomanNumeral => write!(f, "The input contains a character that is not a valid roman numeral"),
        }
    }
}

impl Error for InputError {}