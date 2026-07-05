use crate::FileType;
use bevy::prelude::*;
use std::process::{self, Stdio};

use crate::FFmpegChild;

pub fn spawn_ffmpeg(
    width: u32,
    height: u32,
    mut commands: Commands,
    fps: u32,
    output_name: &str,
    file_ext: FileType,
) {
    let res = format!("{}x{}", width, height);
    let file_name = format!("{}{}", output_name, file_ext);
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
            "libx264",
            "-pix_fmt",
            "yuv420p",
            &file_name,
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("errr");
    commands.insert_resource(FFmpegChild { child: child });
}
