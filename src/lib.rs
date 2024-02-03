//! Minimal safe wrapper around oodle-sys.
//!
//! This crate provides a minimal translation of the
//! [oodle-sys](https://crates.io/crates/oodle-sys) methods to Rust.
//!
//! Check Oodle's [website](http://www.radgametools.com/oodle.htm) for more
//! information.

use oodle_sys;

/// Bool enum for the LZ decoder to check the CRC of the compressed data.
///
/// To use CheckCRC::Yes, the compressed data must have been compressed with
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
pub fn decompress(
    compressed: &[u8],
    decompressed: &mut [u8],
    dictionary_base: Option<&mut [u8]>,
    check_crc: Option<CheckCRC>,
    verbosity: Option<Verbosity>,
    thread_phase: Option<DecodeThreadPhase>,
) -> isize {
    let (dictionary_base, dictionary_base_len) = match dictionary_base {
        Some(x) => (x.as_mut_ptr(), x.len() as isize),
        None => (std::ptr::null_mut(), 0),
    };

    return unsafe {
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
        )
    };
}
