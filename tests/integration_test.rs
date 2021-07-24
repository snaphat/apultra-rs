#[cfg(test)]
mod test
{
    use apultra;

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
        //assert_eq!(a, 1);
    }

    #[test]
    fn compress_decompress2()
    {
        let input_data = vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2];

        let max_window_size = 32;
        let dictionary_size = 0;
        let flags = 0;
        let compressed =
            apultra::compress(&input_data, max_window_size, dictionary_size, flags, None, None);

        let decompressed =
            apultra::decompress(&compressed.unwrap(), dictionary_size, flags).unwrap();
        assert_eq!(input_data, decompressed);
    }

    #[test]
    fn compression_error()
    {
        let e: Result<bool, apultra::Error> = Err(apultra::CompressionError());
        assert!(e.is_err());

        let func = || -> Result<bool, apultra::Error> { Err(apultra::CompressionError())? };

        match func()
        {
            | Err(e) =>
            {
                println!("{}", e);
            },
            | Ok(_) => (),
        }
    }

    #[test]
    fn decompression_error()
    {
        let e: Result<bool, apultra::Error> = Err(apultra::DecompressionError());
        assert!(e.is_err());
        let func = || -> Result<bool, apultra::Error> { Err(apultra::DecompressionError())? };

        match func()
        {
            | Err(e) =>
            {
                println!("{}", e);
            },
            | Ok(_) => (),
        }
    }
}
