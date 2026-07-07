use crate::record_component::record::RecordScreen;
use bevy::prelude::*;
use std::process::{self, Child, Stdio};

/// Holds the running FFmpeg process used for screen recording.
#[derive(Resource)]
pub struct FFmpegChild {
    pub child: Child,
}

/// Spawns an FFmpeg process configured to receive raw RGBA frames via stdin
/// and encode them into a video file.
///
/// This function is invoked by [record](crate::record_screen::record::record) to encode streamed RGBA frame data.
pub fn spawn_ffmpeg(width: u32, height: u32, mut commands: Commands, record: &RecordScreen) {
    // Resolution string in WIDTHxHEIGHT format required by FFmpeg
    let res = format!("{}x{}", width, height);

    // Final output filename including extension
    let file_name = format!("{}{}", record.output_name, record.file_type);

    // Output pixel format (used by the encoder)
    let px_format = format!("{}", record.pixel_format);

    // Video codec identifier understood by FFmpeg
    let codec = format!("{}", record.codec);

    // Spawn FFmpeg process configured to read raw RGBA frames from stdin
    let child = process::Command::new("ffmpeg")
        .args([
            "-y", // Overwrite output file if it exists
            "-f",
            "rawvideo", // Input format: raw video frames
            "-pix_fmt",
            "rgba", // Input pixel format (from GPU)
            "-s",
            &res, // Frame resolution
            "-r",
            &record.fps.to_string(), // Input frame rate
            "-i",
            "-", // Read input from stdin
            "-c:v",
            &codec, // Video codec
            "-pix_fmt",
            &px_format, // Output pixel format
            &file_name, // Output file
        ])
        .stdin(Stdio::piped()) // Enable writing frame data to stdin
        .spawn()
        .unwrap_or_else(|err| {
            eprintln!("failed to spawn ffmpeg: {}", err);
            std::process::exit(1);
        });

    // Store FFmpeg process as a Bevy resource for later frame submission
    commands.insert_resource(FFmpegChild { child });
}
