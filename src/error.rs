extern crate thiserror;
use self::thiserror::Error;

#[derive(Error, Debug)]
pub enum ApultraError
{
    // Normal Errors:
    #[error("Compression error: Internal API returned -1")]
    CompressionError(),
    #[error("Decompression error: Internal API returned -1")]
    DecompressionError(),
}

#[cfg(test)]
mod test
{
    use super::ApultraError;
    #[test]
    fn compression_error()
    {
        let e: Result<bool, ApultraError> = Err(ApultraError::CompressionError());
        assert!(e.is_err());

        let func = || -> Result<bool, ApultraError> { Err(ApultraError::CompressionError())? };

        assert_eq!(
            "Compression error: Internal API returned -1",
            format!("{}", func().unwrap_err())
        );
    }

    #[test]
    fn decompression_error()
    {
        let e: Result<bool, ApultraError> = Err(ApultraError::DecompressionError());
        assert!(e.is_err());
        let func = || -> Result<bool, ApultraError> { Err(ApultraError::DecompressionError())? };

        assert_eq!(
            "Decompression error: Internal API returned -1",
            format!("{}", func().unwrap_err())
        );
    }
}
