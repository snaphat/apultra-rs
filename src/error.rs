use std::{error::Error, fmt};

#[derive(Debug)]
pub struct CompressionError;
#[derive(Debug)]
pub struct DecompressionError;

impl Error for CompressionError {}
impl Error for DecompressionError {}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "compression error. Internal API returned -1.")
    }
}

impl fmt::Display for DecompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decompression error. Internal API returned -1.")
    }
}
