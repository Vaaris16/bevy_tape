use core::fmt;
use std::process::Child;

use bevy::{
    prelude::*,
    render::{
        Render, RenderApp,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
    },
};
mod record_screen;
mod spawn_ffmpeg_command;

use record_screen::record::record;

pub struct TapePlugin;

impl Plugin for TapePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<RecordScreen>::default());
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(Render, record);
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum FileType {
    MP4,
    MOV,
    MKV,
    WEBM,
    AVI,
    MPEG,
    FLV,
    MTS,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ext = match self {
            FileType::MP4 => ".mp4",
            FileType::MOV => ".mov",
            FileType::MKV => ".mkv",
            FileType::WEBM => ".webm",
            FileType::AVI => ".avi",
            FileType::MPEG => ".mpeg",
            FileType::FLV => ".flv",
            FileType::MTS => ".mts",
        };

        f.write_str(ext)
    }
}

#[derive(Component, Clone, Debug, ExtractComponent)]
pub struct RecordScreen {
    pub output_name: String,
    pub fps: u32,
    pub file_type: FileType,
}

#[derive(Resource)]
pub struct FFmpegChild {
    pub child: Child,
}

impl Default for RecordScreen {
    fn default() -> Self {
        Self {
            output_name: String::from("output"),
            fps: 60,
            file_type: FileType::MP4,
        }
    }
}
