#[cfg(test)]
mod tests
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

        let max_window_size = 32;
        let dictionary_size = 0;
        let flags = 0;
        let compressed =
            apultra::compress(&decompressed, max_window_size, dictionary_size, flags, None, None)
                .unwrap_err();

        assert_eq!(compressed.to_string(), "Size error: Input size of zero");
    }

    #[test]
    fn decompress_input_zero_error()
    {
        let compressed = vec![];
        let dictionary_size = 0;
        let flags = 0;
        let decompressed = apultra::decompress(&compressed, dictionary_size, flags).unwrap_err();

        assert_eq!(decompressed.to_string(), "Size error: Input size of zero");
    }

    #[test]
    fn decompress_reservation_error()
    {
        let compressed = vec![0];
        let dictionary_size = 0;
        let flags = 0;
        let decompressed = apultra::decompress(&compressed, dictionary_size, flags).unwrap_err();

        assert_eq!(
            decompressed.to_string(),
            "Reservation error: memory allocation failed because the memory allocator returned a error"
        );
    }
}
