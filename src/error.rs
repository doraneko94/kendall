use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum KendallError {
    DifferentLength { left: usize, right: usize },
    ShortArray { length: usize },
    AllElementsAreTheSame,
    NotBinary { n: usize },
}

impl fmt::Display for KendallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KendallError::DifferentLength { left, right } => {
                write!(f, "Different length!: left={}, right={}", left, right)
            }
            KendallError::ShortArray { length } => {
                write!(f, "Array is too short!: length={}", length)
            }
            KendallError::AllElementsAreTheSame => {
                write!(f, "All elements in an array are the same.")
            }
            KendallError::NotBinary { n } => {
                write!(f, "The number of unique elements in arrays should be 2 != n={}", n)
            }
        }
    }
}

impl error::Error for KendallError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}