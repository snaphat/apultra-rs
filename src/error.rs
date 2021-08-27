use std::collections::TryReserveError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApultraError
{
    // Normal Errors:
    #[error("Compression error: Internal API returned -1")]
    CompressInternalError(),
    #[error("Decompression error: Internal API returned -1")]
    DecompressInternalError(),
    #[error("Compression error: Input size of zero")]
    CompressSizeError(),
    #[error("Decompression error: Input size of zero")]
    DecompressSizeError(),
    #[error("Compression error: {0}")]
    CompressReserveError(TryReserveError),
    #[error("Decompression error: {0}")]
    DecompressReserveError(TryReserveError),
}
