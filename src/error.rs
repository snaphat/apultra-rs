use std::collections::TryReserveError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApultraError
{
    // Normal Errors:
    #[error("Compression error: Internal API returned -1")]
    CompressionError(),
    #[error("Decompression error: Internal API returned -1")]
    DecompressionError(),
    #[error("Size error: Input size of zero")]
    InputSizeError(),
    #[error("Reservation error: {source}")]
    #[rustfmt::skip]
    ReservationError { #[from] source: TryReserveError, },
}
