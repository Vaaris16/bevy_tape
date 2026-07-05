use crate::record_component::{codec::Codec, file_type::FileType, px_format::PixelFormat};
use bevy::{prelude::*, render::extract_component::ExtractComponent};

#[derive(Component, Clone, Debug, ExtractComponent)]
pub struct RecordScreen {
    pub output_name: String,
    pub fps: u32,
    pub file_type: FileType,
    pub pixel_format: PixelFormat,
    pub codec: Codec,
}

impl Default for RecordScreen {
    fn default() -> Self {
        Self {
            output_name: String::from("output"),
            fps: 60,
            file_type: FileType::MP4,
            pixel_format: PixelFormat::Yuv420p,
            codec: Codec::H264,
        }
    }
}
