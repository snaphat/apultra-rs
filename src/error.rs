use std::collections::TryReserveError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApultraError
{
    // Normal Errors:
    #[error("Compression Error: Internal API returned -1")]
    CompressInternalError(),
    #[error("Decompression Error: Internal API returned -1")]
    DecompressInternalError(),
    #[error("Compression Error: Input size of zero")]
    CompressSizeError(),
    #[error("Decompression Error: Input size of zero")]
    DecompressSizeError(),
    #[error("Compression Error: {0}")]
    CompressReserveError(TryReserveError),
    #[error("Decompression Error: {0}")]
    DecompressReserveError(TryReserveError),
}
