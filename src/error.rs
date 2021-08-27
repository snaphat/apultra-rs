use std::collections::TryReserveError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApultraError
{
    // Normal Errors:
    #[error("Compression error: Internal API returned -1")]
    CompressionInternalError(),
    #[error("Decompression error: Internal API returned -1")]
    DecompressionInternalError(),
    #[error("Compression error: Input size of zero")]
    CompressionSizeError(),
    #[error("Decompression error: Input size of zero")]
    DecompressionSizeError(),
    #[error("Reservation error: {source}")]
    #[rustfmt::skip]
    ReservationError { #[from] source: TryReserveError, },
}
