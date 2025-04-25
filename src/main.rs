use clap::{Arg, ArgAction, Command};
use std::io::{self, Read, Write};
use std::path::Path;
use whisper::UnalignedWhisperXFile;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod convert;
mod srt;
mod whisper;

// Special thanks to Claude 3.7 for writing the CLAP interface
fn main() -> Result<()> {
    let matches = Command::new("json_to_srt")
        .version("1.0")
        .author("Blake Scampone")
        .about("Converts WhisperX JSON to SRT format")
        .arg(
            Arg::new("json_path")
                .short('j')
                .long("json_path")
                .help("Path to the input JSON file")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("srt_path")
                .short('s')
                .long("srt_path")
                .help("Path to the output SRT file")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("stdio")
                .short('p')
                .long("pipe")
                .help("Read from stdin and write to stdout (useful for Vim filter)")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Case 1: Stdin/Stdout mode (for Vim filter functionality)
    // Either explicit --pipe flag or no file arguments provided
    if matches.get_flag("stdio")
        || (!matches.contains_id("json_path") && !matches.contains_id("srt_path"))
    {
        let mut json_str = String::new();
        io::stdin().read_to_string(&mut json_str)?;

        // Process the input
        let file = UnalignedWhisperXFile::read_from_str(&json_str)?;
        let srt = file.to_srt();
        let output = srt.to_string();

        // Write to stdout
        io::stdout().write_all(output.as_bytes())?;

        return Ok(());
    }

    // Case 2: File mode
    // Check if we have both json_path and srt_path
    let json_path = matches
        .get_one::<String>("json_path")
        .ok_or("JSON input path is required when not using pipe mode")?;
    let srt_path = matches
        .get_one::<String>("srt_path")
        .ok_or("SRT output path is required when not using pipe mode")?;

    // Process files
    let file = UnalignedWhisperXFile::read_from(Path::new(json_path))?;
    let srt = file.to_srt();
    srt.write_to(Path::new(srt_path))?;

    Ok(())
}
