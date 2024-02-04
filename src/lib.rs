//! Minimal safe wrapper around oodle-sys.
//!
//! This crate provides a minimal translation of the
//! [oodle-sys](https://crates.io/crates/oodle-sys) methods to Rust.
//!
//! Check Oodle's [website](http://www.radgametools.com/oodle.htm) for more
//! information.

#[cfg(test)]
mod tests;

use oodle_sys;

include!("constants.rs");

/// Set of compression algorithms.
///
/// Each compressor has its own trade-offs between compression ratio and speed.
pub enum Compressor {
    /// No compression, just a copy
    None,

    /// Fast decompression, high compression ratio
    Kraken,

    /// Slighly slower decompression but higher compression ratio than Kraken
    Leviathan,

    /// Between Kraken and Selkie in speed with decent compression ratio
    Mermaid,

    /// "Super fast" relative to Mermaid. Used for maximum decompression speed
    Selkie,

    /// Automatically selects between Kraken, Leviathan, Mermaid, and Selkie
    Hydra,
}

impl Into<oodle_sys::OodleLZ_Compressor> for Compressor {
    fn into(self) -> oodle_sys::OodleLZ_Compressor {
        match self {
            Compressor::None => oodle_sys::OodleLZ_Compressor_OodleLZ_Compressor_None,
            Compressor::Kraken => oodle_sys::OodleLZ_Compressor_OodleLZ_Compressor_Kraken,
            Compressor::Leviathan => oodle_sys::OodleLZ_Compressor_OodleLZ_Compressor_Leviathan,
            Compressor::Mermaid => oodle_sys::OodleLZ_Compressor_OodleLZ_Compressor_Mermaid,
            Compressor::Selkie => oodle_sys::OodleLZ_Compressor_OodleLZ_Compressor_Selkie,
            Compressor::Hydra => oodle_sys::OodleLZ_Compressor_OodleLZ_Compressor_Hydra,
        }
    }
}

/// Set of compression levels.
///
/// A compressed data stream can be decompressed with any level, but the
/// compression level used to compress the data must be known.
///
/// The compression level controls the amount of work done by the compressor to
/// find the best compressed bitstream. It does not directly impact
/// decompression speed, it trades off encode speed for compression bitstream
/// quality.
pub enum CompressionLevel {
    /// Don't compress, just copy the data
    None,

    /// Lowest compression ratio, super fast
    SuperFast,

    /// Fastest with still decent compression ratio
    VeryFast,

    /// Good for daily use
    Fast,

    /// Standard medium speed
    Normal,

    /// Optimal parse level 1 (fastest)
    Optimal1,

    /// Optimal parse level 2 (recommended baseline)
    Optimal2,

    /// Optimal parse level 3 (slower)
    Optimal3,

    /// Optimal parse level 4 (very slow)
    Optimal4,

    /// Optimal parse level 5 (don't care about speed, just want best ratio)
    Optimal5,

    /// Faster than SuperFast, but lower compression ratio
    HyperFast1,

    /// Faster than HyperFast1, but lower compression ratio
    HyperFast2,

    /// Faster than HyperFast2, but lower compression ratio
    HyperFast3,

    /// Faster than HyperFast3, but lower compression ratio
    HyperFast4,

    /// Alias optimal standard level
    Optimal,

    /// Alias hyperfast base level
    HyperFast,

    /// Alias for the maximum compression level
    Max,

    /// Alias for the minimum compression level
    Min,
}

impl Into<oodle_sys::OodleLZ_CompressionLevel> for CompressionLevel {
    #[rustfmt::skip]
    fn into(self) -> oodle_sys::OodleLZ_CompressionLevel {
        match self {
            CompressionLevel::None => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_None,
            CompressionLevel::SuperFast => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_SuperFast,
            CompressionLevel::VeryFast => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_VeryFast,
            CompressionLevel::Fast => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Fast,
            CompressionLevel::Normal => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Normal,
            CompressionLevel::Optimal1 => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Optimal1,
            CompressionLevel::Optimal2 => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Optimal2,
            CompressionLevel::Optimal3 => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Optimal3,
            CompressionLevel::Optimal4 => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Optimal4,
            CompressionLevel::Optimal5 => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Optimal5,
            CompressionLevel::HyperFast1 => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_HyperFast1,
            CompressionLevel::HyperFast2 => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_HyperFast2,
            CompressionLevel::HyperFast3 => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_HyperFast3,
            CompressionLevel::HyperFast4 => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_HyperFast4,
            CompressionLevel::Optimal => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Optimal,
            CompressionLevel::HyperFast => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_HyperFast,
            CompressionLevel::Max => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Max,
            CompressionLevel::Min => oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Min,
        }
    }
}

/// Decoder profile to target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    /// Main profile, full feature set
    Main,

    /// Kraken only, limited feature set
    Reduced,
}

impl Into<oodle_sys::OodleLZ_Profile> for Profile {
    fn into(self) -> oodle_sys::OodleLZ_Profile {
        match self {
            Profile::Main => oodle_sys::OodleLZ_Profile_OodleLZ_Profile_Main,
            Profile::Reduced => oodle_sys::OodleLZ_Profile_OodleLZ_Profile_Reduced,
        }
    }
}

impl From<oodle_sys::OodleLZ_Profile> for Profile {
    fn from(profile: oodle_sys::OodleLZ_Profile) -> Self {
        match profile {
            oodle_sys::OodleLZ_Profile_OodleLZ_Profile_Main => Profile::Main,
            oodle_sys::OodleLZ_Profile_OodleLZ_Profile_Reduced => Profile::Reduced,
            _ => panic!("Invalid profile"),
        }
    }
}

/// Controls the amount of internal threading used by the compressor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Jobify {
    /// Use compressor default for level of internal job usage
    Default,

    /// Do not use jobs at all
    Disable,

    /// Try to balance parallelism with increased memory use
    Normal,

    /// Maximize parallelism at the cost of increased memory use
    Aggressive,
}

impl Into<oodle_sys::OodleLZ_Jobify> for Jobify {
    fn into(self) -> oodle_sys::OodleLZ_Jobify {
        match self {
            Jobify::Default => oodle_sys::OodleLZ_Jobify_OodleLZ_Jobify_Default,
            Jobify::Disable => oodle_sys::OodleLZ_Jobify_OodleLZ_Jobify_Disable,
            Jobify::Normal => oodle_sys::OodleLZ_Jobify_OodleLZ_Jobify_Normal,
            Jobify::Aggressive => oodle_sys::OodleLZ_Jobify_OodleLZ_Jobify_Aggressive,
        }
    }
}

impl From<oodle_sys::OodleLZ_Jobify> for Jobify {
    fn from(jobify: oodle_sys::OodleLZ_Jobify) -> Self {
        match jobify {
            oodle_sys::OodleLZ_Jobify_OodleLZ_Jobify_Default => Jobify::Default,
            oodle_sys::OodleLZ_Jobify_OodleLZ_Jobify_Disable => Jobify::Disable,
            oodle_sys::OodleLZ_Jobify_OodleLZ_Jobify_Normal => Jobify::Normal,
            oodle_sys::OodleLZ_Jobify_OodleLZ_Jobify_Aggressive => Jobify::Aggressive,
            _ => panic!("Invalid jobify"),
        }
    }
}

/// Options to use for compression.
///
/// Typically, you would use the default options and only change the fields you
/// need to modify.
///
/// To ensure that the options are valid, call [CompressOptions::validate]
/// after modifying the fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompressOptions {
    /// Was previously named `verbosity`, set to 0
    unused: u32,

    /// Cannot be used to reduce a compressor's default MML, but can be higher.
    /// On some types of data, a large MML (6 or 8) is a space-speed win.
    min_match_len: i32,

    /// Whether chunks should be independent, for seeking and parallelism
    seek_chunk_reset: bool,

    /// Length of independent seek chunks if [seek_chunk_reset] is true.
    /// This must be a power of 2 and >= [BLOCK_LEN]
    seek_chunk_len: u32,

    /// Decoder profile to target (set to 0)
    profile: Profile,

    /// Sets a maximum offset for matches, if lower than the maximum the format supports.
    /// <= 0 means infinite (use whole buffer).
    /// Often power of 2 but doesn't have to be.
    dictionary_size: i32,

    /// Number of bytes; It must gain at least this many bytes of compressed
    /// size to accept a speed-decreasing decision
    space_speed_tradeoff_bytes: i32,

    /// Was previously named `max_huffmans_per_chunk`, set to 0
    unused2: i32,

    /// Whether the encoder should send CRCs for each compressed quantum for
    /// integrity checking. This is necessary for using `CheckCRC::Yes` in
    /// decompression.
    send_quantum_crcs: bool,

    /// Size of local dictionary before needing a long range matcher.
    /// This does not set a window size for the decoder;
    /// it's useful to limit memory use and time taken in the encoder.
    /// This must be a power of 2 and < [LOCALDICTIONARYSIZE_MAX].
    max_local_dictionary_size: u32,

    /// Whether the encoder should should find matches beyond [max_local_dictionary_size]
    /// when using a long range matcher.
    make_long_range_matcher: bool,

    /// Default is 0. If non-zero, this sets the size of the match finder structure
    /// (often a hash table).
    match_table_size_log2: i32,

    /// Controls internal job usage for the compressor.
    jobify: Jobify,

    /// User pointer passed through to RunJob and WaitJob callbacks.
    jobify_user_ptr: *mut std::ffi::c_void,

    /// Far match must be at least this long.
    far_match_min_len: i32,

    /// If not 0, the log2 of the offset that must meet [far_match_min_len].
    far_match_offset_log2: i32,

    /// Reserved for future use, set to 0
    reserved: [u32; 4],
}

impl CompressOptions {
    pub fn validate(&mut self) {
        let options: *mut oodle_sys::OodleLZ_CompressOptions = &mut (*self).into();
        unsafe { oodle_sys::OodleLZ_CompressOptions_Validate(options) };
        *self = CompressOptions::from(unsafe { *options });
    }
}

impl Into<oodle_sys::OodleLZ_CompressOptions> for CompressOptions {
    fn into(self) -> oodle_sys::OodleLZ_CompressOptions {
        oodle_sys::OodleLZ_CompressOptions {
            unused_was_verbosity: self.unused,
            minMatchLen: self.min_match_len,
            seekChunkReset: if self.seek_chunk_reset { 1 } else { 0 },
            seekChunkLen: self.seek_chunk_len as i32,
            profile: self.profile.into(),
            dictionarySize: self.dictionary_size,
            spaceSpeedTradeoffBytes: self.space_speed_tradeoff_bytes,
            unused_was_maxHuffmansPerChunk: self.unused2,
            sendQuantumCRCs: if self.send_quantum_crcs { 1 } else { 0 },
            maxLocalDictionarySize: self.max_local_dictionary_size as i32,
            makeLongRangeMatcher: if self.make_long_range_matcher { 1 } else { 0 },
            matchTableSizeLog2: self.match_table_size_log2,
            jobify: self.jobify.into(),
            jobifyUserPtr: self.jobify_user_ptr,
            farMatchMinLen: self.far_match_min_len,
            farMatchOffsetLog2: self.far_match_offset_log2,
            reserved: self.reserved,
        }
    }
}

impl From<oodle_sys::OodleLZ_CompressOptions> for CompressOptions {
    fn from(options: oodle_sys::OodleLZ_CompressOptions) -> Self {
        Self {
            unused: options.unused_was_verbosity,
            min_match_len: options.minMatchLen,
            seek_chunk_reset: options.seekChunkReset != 0,
            seek_chunk_len: options.seekChunkLen as u32,
            profile: options.profile.into(),
            dictionary_size: options.dictionarySize,
            space_speed_tradeoff_bytes: options.spaceSpeedTradeoffBytes,
            unused2: options.unused_was_maxHuffmansPerChunk,
            send_quantum_crcs: options.sendQuantumCRCs != 0,
            max_local_dictionary_size: options.maxLocalDictionarySize as u32,
            make_long_range_matcher: options.makeLongRangeMatcher != 0,
            match_table_size_log2: options.matchTableSizeLog2,
            jobify: options.jobify.into(),
            jobify_user_ptr: options.jobifyUserPtr,
            far_match_min_len: options.farMatchMinLen,
            far_match_offset_log2: options.farMatchOffsetLog2,
            reserved: options.reserved,
        }
    }
}

impl Default for CompressOptions {
    fn default() -> Self {
        let options = unsafe {
            *oodle_sys::OodleLZ_CompressOptions_GetDefault(
                Compressor::None.into(),
                CompressionLevel::None.into(),
            )
        };

        options.into()
    }
}

/// Compress some data from memory to memory synchronously.
///
/// # Arguments
///
/// * `compressor` - The compressor to use.
/// * `decompressed` - The buffer containing the data to compress.
/// * `compressed` - The buffer to write the compressed data to.
/// * `level` - The compression level to use.
/// * `options` - Additional options to use for compression.
/// * `dictionary_base` - Preconditioned dictionary to use for compression.
/// * `scratch_memory` - Scratch memory to use for compression.
///
/// # Returns
///
/// The size of the compressed data.
pub fn compress(
    compressor: Compressor,
    decompressed: &[u8],
    compressed: &mut [u8],
    level: CompressionLevel,
    options: Option<CompressOptions>,
    dictionary_base: Option<&[u8]>,
    scratch_memory: Option<&mut [u8]>,
) -> Result<usize, u32> {
    let options = match options {
        Some(x) => &x.into(),
        None => std::ptr::null() as *const _,
    };

    let dictionary_base = match dictionary_base {
        Some(x) => x.as_ptr(),
        None => std::ptr::null(),
    };

    let (scratch_memory, scratch_memory_len) = match scratch_memory {
        Some(x) => (x.as_mut_ptr(), x.len() as isize),
        None => (std::ptr::null_mut(), 0),
    };

    let result = unsafe {
        oodle_sys::OodleLZ_Compress(
            compressor.into(),
            decompressed.as_ptr() as *const _,
            decompressed.len() as isize,
            compressed.as_mut_ptr() as *mut _,
            level.into(),
            options,
            dictionary_base as *const _,
            std::ptr::null(), // TODO: add long_range_matcher
            scratch_memory as *mut _,
            scratch_memory_len,
        ) as usize
    };

    if result == FAILED as usize {
        Err(FAILED)
    } else {
        Ok(result)
    }
}

/// Bool enum for the LZ decoder to check the CRC of the compressed data.
///
/// To use [CheckCRC::Yes], the compressed data must have been compressed with
/// the CRC option enabled.
pub enum CheckCRC {
    No,
    Yes,
}

impl Default for CheckCRC {
    fn default() -> Self {
        CheckCRC::No
    }
}

impl Into<oodle_sys::OodleLZ_CheckCRC> for CheckCRC {
    fn into(self) -> oodle_sys::OodleLZ_CheckCRC {
        match self {
            CheckCRC::No => oodle_sys::OodleLZ_CheckCRC_OodleLZ_CheckCRC_No,
            CheckCRC::Yes => oodle_sys::OodleLZ_CheckCRC_OodleLZ_CheckCRC_Yes,
        }
    }
}

/// Verbosity level for LZ decompression.
pub enum Verbosity {
    /// Will not log anything, even when the decoder sees corrupted data.
    None,
    Minimal,
    Some,
    Lots,
}

impl Default for Verbosity {
    fn default() -> Self {
        Verbosity::None
    }
}

impl Into<oodle_sys::OodleLZ_Verbosity> for Verbosity {
    fn into(self) -> oodle_sys::OodleLZ_Verbosity {
        match self {
            Verbosity::None => oodle_sys::OodleLZ_Verbosity_OodleLZ_Verbosity_None,
            Verbosity::Minimal => oodle_sys::OodleLZ_Verbosity_OodleLZ_Verbosity_Minimal,
            Verbosity::Some => oodle_sys::OodleLZ_Verbosity_OodleLZ_Verbosity_Some,
            Verbosity::Lots => oodle_sys::OodleLZ_Verbosity_OodleLZ_Verbosity_Lots,
        }
    }
}

/// Thread phase for threaded decompression.
///
/// Note that threaded decompression is only available for the Kraken compressor.
pub enum DecodeThreadPhase {
    One,
    Two,
    All,
    Unthreaded,
}

impl Default for DecodeThreadPhase {
    fn default() -> Self {
        DecodeThreadPhase::Unthreaded
    }
}

impl Into<oodle_sys::OodleLZ_Decode_ThreadPhase> for DecodeThreadPhase {
    #[rustfmt::skip]
    fn into(self) -> oodle_sys::OodleLZ_Decode_ThreadPhase {
        match self {
            DecodeThreadPhase::One => oodle_sys::OodleLZ_Decode_ThreadPhase_OodleLZ_Decode_ThreadPhase1,
            DecodeThreadPhase::Two => oodle_sys::OodleLZ_Decode_ThreadPhase_OodleLZ_Decode_ThreadPhase2,
            DecodeThreadPhase::All => oodle_sys::OodleLZ_Decode_ThreadPhase_OodleLZ_Decode_ThreadPhaseAll,
            DecodeThreadPhase::Unthreaded => oodle_sys::OodleLZ_Decode_ThreadPhase_OodleLZ_Decode_Unthreaded,
        }
    }
}

/// Decompress some data from memory to memory synchronously.
///
/// # Arguments
///
/// * `compressed` - The buffer containing the compressed data.
/// * `decompressed` - The buffer to write the decompressed data to.
/// * `dictionary_base` - Preconditioned dictionary to use for decompression.
/// The dictionary must be the same as the one used for compression.
/// * `check_crc` - Whether to check the validity of the compressed data.
/// * `verbosity` - The verbosity of the decompression.
/// * `thread_phase` - The thread phase for threaded decompression.
///
/// # Returns
///
/// The size of the decompressed data.
pub fn decompress(
    compressed: &[u8],
    decompressed: &mut [u8],
    dictionary_base: Option<&mut [u8]>,
    check_crc: Option<CheckCRC>,
    verbosity: Option<Verbosity>,
    thread_phase: Option<DecodeThreadPhase>,
) -> Result<usize, u32> {
    let (dictionary_base, dictionary_base_len) = match dictionary_base {
        Some(x) => (x.as_mut_ptr(), x.len() as isize),
        None => (std::ptr::null_mut(), 0),
    };

    let result = unsafe {
        oodle_sys::OodleLZ_Decompress(
            compressed.as_ptr() as *const _,
            compressed.len() as isize,
            decompressed.as_mut_ptr() as *mut _,
            decompressed.len() as isize,
            oodle_sys::OodleLZ_FuzzSafe_OodleLZ_FuzzSafe_Yes,
            check_crc.unwrap_or_default().into(),
            verbosity.unwrap_or_default().into(),
            dictionary_base as *mut _,
            dictionary_base_len,
            None, // TODO: add callback
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            thread_phase.unwrap_or_default().into(),
        ) as usize
    };

    if result == FAILED as usize {
        Err(FAILED)
    } else {
        Ok(result)
    }
}
