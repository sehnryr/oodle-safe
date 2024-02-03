use crate as oodle_safe;

#[test]
#[rustfmt::skip]
fn test_constants() {
    assert_eq!(oodle_safe::LOCALDICTIONARYSIZE_MAX, oodle_sys::OODLELZ_LOCALDICTIONARYSIZE_MAX);
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
    );

    // Trim the output buffer to the actual size of the compressed data and convert
    // it to a Vec<u8>, else the test will end up double free-ing the buffer.
    let compressed = compressed[..compressed_size as usize].to_vec();

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
    );

    // Make sure the decompressed size matches the expected size.
    assert_eq!(decompressed_size, result as usize);

    // Compare the decompressed data to the expected output.
    let expected = include_bytes!("../test_data/decompressed");
    assert_eq!(decompressed, expected);
}
