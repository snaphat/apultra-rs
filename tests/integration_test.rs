#[cfg(test)]
mod tests
{
    use std::intrinsics::transmute;

    use apultra;

    #[test]
    fn main()
    {
        // Create some data.
        let ddata0 = vec![1, 2, 2, 2, 2, 3, 4, 5, 5, 5, 5, 5, 5];

        // Create other example parameters.
        let max_window_size = 1024;
        let dictionary_size = 0;
        let flags = 0; // Must be zero.
        let progress = |original_size: i64, compressed_size: i64| {
            println!("Original size: {}, Compressed Size: {}", original_size, compressed_size);
        };
        let mut stats = apultra::Stats::default();

        // Compress data.
        let cdata_res = apultra::compress(
            &ddata0,
            max_window_size,
            dictionary_size,
            flags,
            Some(Box::new(progress)), // Pass callback closure. None can also be passed.
            Some(&mut stats),         // Pass stats structure. None can also be passed.
        );

        // Check.
        let cdata0 = match cdata_res
        {
            | Err(err) =>
            {
                println!("Error: {}", err);
                return;
            },
            | Ok(cdata) => cdata,
        };

        // Decompress data.
        let ddata_res = apultra::decompress(&cdata0, dictionary_size, flags);

        // Check.
        let ddata1 = match ddata_res
        {
            | Err(err) =>
            {
                println!("Error: {}", err);
                return;
            },
            | Ok(ddata) => ddata,
        };

        // Verify result.
        assert_eq!(ddata0, ddata1);
    }

    #[test]
    fn compress_decompress()
    {
        let input_data = vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2];

        let max_window_size = 32;
        let dictionary_size = 0;
        let flags = 0;
        let progress = |original_size: i64, compressed_size: i64| {
            println!("{} {}", original_size, compressed_size);
        };
        let mut stats = apultra::Stats::default();
        let compressed = apultra::compress(
            &input_data,
            max_window_size,
            dictionary_size,
            flags,
            Some(Box::new(progress)),
            Some(&mut stats),
        );
        let decompressed =
            apultra::decompress(&compressed.unwrap(), dictionary_size, flags).unwrap();
        assert_eq!(input_data, decompressed);
        assert_eq!(stats.min_match_len, 8);
        assert_eq!(stats.max_match_len, 9);
    }

    #[test]
    fn compress_decompress2()
    {
        let input_data = vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2];

        let max_window_size = 64;
        let dictionary_size = 0;
        let flags = 0;
        let compressed =
            apultra::compress(&input_data, max_window_size, dictionary_size, flags, None, None);

        let decompressed =
            apultra::decompress(&compressed.unwrap(), dictionary_size, flags).unwrap();
        assert_eq!(input_data, decompressed);
    }

    #[test]
    fn compress_input_zero_error()
    {
        let decompressed = vec![];
        let err = apultra::compress(&decompressed, 32, 0, 0, None, None).unwrap_err();

        assert_eq!(err.to_string(), "Compression Error: Input size of zero");
    }

    #[test]
    fn decompress_input_zero_error()
    {
        let compressed = vec![];
        let err = apultra::decompress(&compressed, 0, 0).unwrap_err();

        assert_eq!(err.to_string(), "Decompression Error: Input size of zero");
    }

    #[test]
    fn compress_reserve_error()
    {
        let raw = [255, 255, 255, 255]; // 4 bytes of memory.
        let decompressed: &[u8] = unsafe { transmute(raw) }; // max size fat pointer.
        let err = apultra::compress(&decompressed, 32, 0, 0, None, None).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Compression Error: memory allocation failed because the memory allocator returned a error"
        );
    }

    #[test]
    fn decompress_reserve_error()
    {
        let compressed = vec![0];
        let err = apultra::decompress(&compressed, 0, 0).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Decompression Error: memory allocation failed because the computed capacity exceeded the collection's maximum"
        );
    }
}
