extern crate thiserror;
use self::thiserror::Error;

#[derive(Error, Debug)]
pub enum ApultraError
{
    // Normal Errors:
    #[error("compression error: Internal API returned -1")]
    CompressionError(),
    #[error("decompression error: Internal API returned -1")]
    DecompressionError(),
}

#[test]
fn test_compression_error()
{
    let e: Result<bool, ApultraError> = Err(ApultraError::CompressionError());
    assert!(e.is_err());

    let func = || -> Result<bool, ApultraError> { Err(ApultraError::CompressionError())? };

    assert_eq!("compression error: Internal API returned -1", format!("{}", func().unwrap_err()));
}

#[test]
fn test_decompression_error()
{
    let e: Result<bool, ApultraError> = Err(ApultraError::DecompressionError());
    assert!(e.is_err());
    let func = || -> Result<bool, ApultraError> { Err(ApultraError::DecompressionError())? };

    assert_eq!("decompression error: Internal API returned -1", format!("{}", func().unwrap_err()));
}
