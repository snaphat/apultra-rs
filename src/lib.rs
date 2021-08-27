#![feature(try_reserve)] // Only supported in future Rust.

use libffi::high::ClosureMut2;

mod error;
mod ffi;
use error::ApultraError;

use crate::ffi::*;

pub type Stats = apultra_stats;
pub type Error = ApultraError;
pub use error::ApultraError::{CompressionError, DecompressionError, ReservationError};

///Compress memory
///
/// # Arguments
/// * `input_data` pointer to input(source) data to compress
/// * `max_window_size` maximum window size to use (0 for default)
/// * `dictionary_size` size of dictionary in front of input data (0 for none)
/// * `flags` compression flags (set to 0)
/// * `progress` progress function, called after compressing each block, or NULL for none
/// * `stats` pointer to compression stats that are filled if this function is successful, or NULL
///
/// # Returns
/// `Result` containing compressed buffer on success or `apultra::Error` on compression failure.
pub fn compress(
    input_data: &[u8],
    max_window_size: usize,
    dictionary_size: usize,
    flags: u32,
    mut maybe_progress: Option<Box<dyn FnMut(i64, i64)>>,
    stats: Option<&mut Stats>,
) -> Result<Vec<u8>, ApultraError>
{
    let progress = match &mut maybe_progress
    {
        | Some(progress) => Some(ClosureMut2::new(progress)),
        | _ => None,
    };

    let progress_ptr = progress.as_ref().map_or(None, |a| Some(*a.code_ptr()));

    // Try to allocate memory for compressed data.
    let max_size = get_max_compressed_size(input_data.len());
    let mut out_buffer: Vec<u8> = Vec::with_capacity(max_size);
    out_buffer.try_reserve(max_size)?;
    out_buffer.resize(max_size, 0);

    // Compress data.
    let size = unsafe {
        apultra_compress(
            input_data.as_ptr(),
            out_buffer.as_mut_ptr(),
            input_data.len(),
            out_buffer.len(),
            flags,
            max_window_size,
            dictionary_size,
            progress_ptr,
            stats,
        )
    };

    // Check for errors.
    match size
    {
        | -1 => Err(CompressionError()),
        | _ =>
        {
            out_buffer.resize(size as usize, 0);
            Ok(out_buffer)
        },
    }
}

/// Decompress data in memory
///
/// # Arguments
/// * `input_data` compressed data
/// * `dictionary_size` size of dictionary in front of input data (0 for none)
/// * `flags` compression flags (set to 0)
///
/// # Returns
/// `Result` containing decompressed buffer on success or `apultra::Error` on decompression failure.
pub fn decompress(
    input_data: &[u8],
    dictionary_size: usize,
    flags: u32,
) -> Result<Vec<u8>, ApultraError>
{
    // Try to allocate memory for decompressed data.
    let max_size = get_max_decompressed_size(input_data, flags);
    let mut out_buffer: Vec<u8> = Vec::new();
    out_buffer.try_reserve(max_size)?;
    out_buffer.resize(max_size, 0);

    // Decompress data.
    let size = unsafe {
        apultra_decompress(
            input_data.as_ptr(),
            out_buffer.as_mut_ptr(),
            input_data.len(),
            out_buffer.len(),
            dictionary_size,
            flags,
        )
    };

    // Check for errors.
    match size
    {
        | -1 => Err(DecompressionError()),
        | _ =>
        {
            out_buffer.resize(size as usize, 0);
            Ok(out_buffer)
        },
    }
}

/// Get maximum compressed size of input(source) data
///
/// # Arguments
/// * `input_size` input(source) size in bytes
///
/// # Returns
/// maximum compressed size
pub fn get_max_compressed_size(input_size: usize) -> usize
{
    unsafe { apultra_get_max_compressed_size(input_size) }
}

/// Get maximum decompressed size of compressed data
///
/// # Arguments
/// * `input_data` Reference to compressed data
/// * `flags` compression flags (set to 0)
///
/// # Returns
/// maximum decompressed size
pub fn get_max_decompressed_size(input_data: &[u8], flags: u32) -> usize
{
    unsafe { apultra_get_max_decompressed_size(input_data.as_ptr(), input_data.len(), flags) }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn compress()
    {
        let input_data = vec![0; 100];
        let max_window_size = 0;
        let dictionary_size = 0;
        let flags = 0;
        let progress = |a, b| {
            println!("{} {}", a, b);
        };
        let mut stats = apultra_stats::default();
        let compressed = super::compress(
            &input_data,
            max_window_size,
            dictionary_size,
            flags,
            Some(Box::new(progress)),
            Some(&mut stats),
        )
        .unwrap();
        assert_eq!(compressed.len(), 6);
        assert_eq!(compressed, [0, 173, 1, 86, 192, 0]);
    }

    #[test]
    fn decompress()
    {
        let input_data: Vec<u8> = vec![0, 173, 1, 86, 192, 0];
        let dictionary_size = 0;
        let flags = 0;
        let decompressed = super::decompress(&input_data, dictionary_size, flags).unwrap();
        assert_eq!(decompressed.len(), 100);
        assert_eq!(decompressed, [0; 100]);
    }

    #[test]
    fn decompress_error()
    {
        let input_data: Vec<u8> = vec![0];
        let flags = 0;
        let dictionary_size = 0;
        let err = super::decompress(&input_data, dictionary_size, flags).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Reservation error: memory allocation failed because the memory allocator returned a error"
        );
    }

    #[test]
    fn get_max_compressed_size()
    {
        assert_eq!(super::get_max_compressed_size(100), 114);
    }

    #[test]
    fn get_max_decompressed_size()
    {
        let input_data: Vec<u8> = vec![0, 173, 1, 86, 192, 0];
        assert_eq!(super::get_max_decompressed_size(&input_data, 0), 100);
    }
}
