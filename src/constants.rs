/// The number of raw bytes per seek chunk
pub const BLOCK_LEN: u32 = oodle_sys::OODLELZ_BLOCK_LEN;

/// Return value of [compress] and [decompress] on failure
pub const FAILED: u32 = oodle_sys::OODLELZ_FAILED;

/// Maximum value of max_local_dictionary_size in CompressOptions
pub const LOCALDICTIONARYSIZE_MAX: u32 = oodle_sys::OODLELZ_LOCALDICTIONARYSIZE_MAX;
