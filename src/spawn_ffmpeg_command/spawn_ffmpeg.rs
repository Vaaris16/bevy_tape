use crate::{Codec, FileType, PixelFormat};
use bevy::prelude::*;
use std::process::{self, Child, Stdio};

#[derive(Resource)]
pub struct FFmpegChild {
    pub child: Child,
}

pub fn spawn_ffmpeg(
    width: u32,
    height: u32,
    mut commands: Commands,
    fps: u32,
    output_name: &str,
    file_ext: FileType,
    pixel_format: PixelFormat,
    codec_type: Codec,
) {
    let res = format!("{}x{}", width, height);
    let file_name = format!("{}{}", output_name, file_ext);
    let px_format = format!("{}", pixel_format);
    let codec = format!("{}", codec_type);
    let child = process::Command::new("ffmpeg")
        .args([
            "-y",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgba",
            "-s",
            &res,
            "-r",
            &fps.to_string(),
            "-i",
            "-",
            "-c:v",
            &codec,
            "-pix_fmt",
            &px_format,
            &file_name,
        ])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap_or_else(|err| {
            eprintln!("failed spawning ffmpeg: {}", err);
            std::process::exit(1);
        });
    commands.insert_resource(FFmpegChild { child: child });
}
