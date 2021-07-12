use apultra;

#[test]
fn test_compress_decompress() {
    let input_data = vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2];

    let flags = 0;
    let max_window_size = 32;
    let dictionary_size = 0;
    let progress = |original_size: i64, compressed_size: i64| {
        println!("{} {}", original_size, compressed_size);
    };
    let mut stats = apultra::Stats::default();
    let compressed = apultra::compress(
        &input_data,
        flags,
        max_window_size,
        dictionary_size,
        Some(Box::new(progress)),
        Some(&mut stats),
    );
    let decompressed = apultra::decompress(&compressed.unwrap(), dictionary_size, flags).unwrap();
    assert_eq!(input_data, decompressed);
    assert_eq!(stats.min_match_len, 8);
    assert_eq!(stats.max_match_len, 9);
    //assert_eq!(a, 1);
}

#[test]
fn test_compress_decompress2() {
    let input_data = vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2];

    let flags = 0;
    let max_window_size = 32;
    let dictionary_size = 0;
    let compressed = apultra::compress(
        &input_data,
        flags,
        max_window_size,
        dictionary_size,
        None,
        None,
    );

    let decompressed = apultra::decompress(&compressed.unwrap(), dictionary_size, flags).unwrap();
    assert_eq!(input_data, decompressed);
}
