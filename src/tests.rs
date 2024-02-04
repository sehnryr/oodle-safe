use crate as oodle_safe;

#[test]
#[rustfmt::skip]
fn test_constants() {
    assert_eq!(oodle_safe::BLOCK_LEN, oodle_sys::OODLELZ_BLOCK_LEN);
    assert_eq!(oodle_safe::FAILED, oodle_sys::OODLELZ_FAILED);
    assert_eq!(oodle_safe::LOCALDICTIONARYSIZE_MAX, oodle_sys::OODLELZ_LOCALDICTIONARYSIZE_MAX);
}

#[test]
fn test_default_compress_options() {
    let options = oodle_safe::CompressOptions::default();
    assert_eq!(options.min_match_len, 0);
    assert_eq!(options.seek_chunk_reset, false);
    assert_eq!(options.seek_chunk_len, 1 << 18);
    assert_eq!(options.profile, oodle_safe::Profile::Main);
    assert_eq!(options.dictionary_size, 0);
    assert_eq!(options.space_speed_tradeoff_bytes, 256);
    assert_eq!(options.send_quantum_crcs, false);
    assert_eq!(options.max_local_dictionary_size, 1 << 24);
    assert_eq!(options.make_long_range_matcher, true);
    assert_eq!(options.match_table_size_log2, 0);
    assert_eq!(options.jobify, oodle_safe::Jobify::Default);
    assert_eq!(options.jobify_user_ptr, 0x0 as *mut std::ffi::c_void);
    assert_eq!(options.far_match_min_len, 0);
    assert_eq!(options.far_match_offset_log2, 0);
}

#[test]
#[rustfmt::skip]
fn test_compress_options_validate() {
    let mut options = oodle_safe::CompressOptions::default();
    options.validate();
    assert_eq!(options.min_match_len, 2);

    options.seek_chunk_len = 0;
    options.validate();
    assert_eq!(options.seek_chunk_len, oodle_safe::BLOCK_LEN);

    options.max_local_dictionary_size = oodle_safe::LOCALDICTIONARYSIZE_MAX;
    options.validate();
    assert_eq!(options.max_local_dictionary_size, oodle_safe::LOCALDICTIONARYSIZE_MAX >> 1);
}

#[test]
#[rustfmt::skip]
fn test_compress() {
    let decompressed = include_bytes!("../test_data/decompressed");
    let mut compressed = vec![0u8; decompressed.len() + 8];

    let compressed_size = oodle_safe::compress(
        oodle_safe::Compressor::Kraken,
        decompressed,
        &mut compressed,
        oodle_safe::CompressionLevel::Normal,
        None,
        None,
        None,
    )
    .unwrap_or_else(|_| panic!("compression failed"));

    // Trim the output buffer to the actual size of the compressed data and convert
    // it to a Vec<u8>, else the test will end up double free-ing the buffer.
    let compressed = compressed[..compressed_size].to_vec();

    // Compare the compressed data to the expected output.
    let expected = include_bytes!("../test_data/compressed");
    let expected = &expected[4..]; // Skip the size header.
    assert_eq!(compressed, expected);
}

#[test]
fn test_compress_with_default_options() {
    let decompressed = include_bytes!("../test_data/decompressed");
    let mut compressed = vec![0u8; decompressed.len() + 8];

    let compressed_size = oodle_safe::compress(
        oodle_safe::Compressor::Kraken,
        decompressed,
        &mut compressed,
        oodle_safe::CompressionLevel::Normal,
        Some(oodle_safe::CompressOptions::default()),
        None,
        None,
    )
    .unwrap_or_else(|_| panic!("compression failed"));

    // Trim the output buffer to the actual size of the compressed data and convert
    // it to a Vec<u8>, else the test will end up double free-ing the buffer.
    let compressed = compressed[..compressed_size].to_vec();

    // Compare the compressed data to the expected output.
    let expected = include_bytes!("../test_data/compressed");
    let expected = &expected[4..]; // Skip the size header.
    assert_eq!(compressed, expected);
}

#[test]
#[rustfmt::skip]
fn test_decompress() {
    let compressed = include_bytes!("../test_data/compressed");
    let decompressed_size = u32::from_le_bytes(compressed[..4].try_into().unwrap()) as usize;
    let mut decompressed = vec![0u8; decompressed_size];

    let result = oodle_safe::decompress(
        &compressed[4..],
        &mut decompressed,
        None,
        None,
        None,
        None,
    )
    .unwrap_or_else(|_| panic!("decompression failed"));

    // Make sure the decompressed size matches the expected size.
    assert_eq!(decompressed_size, result);

    // Compare the decompressed data to the expected output.
    let expected = include_bytes!("../test_data/decompressed");
    assert_eq!(decompressed, expected);
}
