use std::path::Path;

use srt::SrtFile;
use whisper::UnalignedWhisperXFile;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub mod convert;
pub mod srt;
pub mod whisper;

fn main() -> Result<()> {
    let json_path = "/home/dgmastertemple/Development/python/uvx_whisperx/turbo/2024-03-17 - The True Grace of God - Dr. Samuel Renihan (31824205120297).json";
    let srt_path = "/home/dgmastertemple/Development/python/uvx_whisperx/turbo/Rust 2024-03-17 - The True Grace of God - Dr. Samuel Renihan (31824205120297).srt";
    let file = UnalignedWhisperXFile::read_from(json_path)?;
    let srt = file.to_srt();
    srt.write_to(srt_path)?;

    Ok(())
}
