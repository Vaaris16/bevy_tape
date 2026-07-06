use crate::record_component::{codec::Codec, file_type::FileType, px_format::PixelFormat};
use bevy::{prelude::*, render::extract_component::ExtractComponent};
/// [`RecordScreen`] is a component that the user attaches to a camera entity
/// to enable real-time screen recording.
///
/// This component defines the performance characteristics and output
/// configuration of the recording process.

#[derive(Component, Clone, Debug, ExtractComponent)]
pub struct RecordScreen {
    /// The output file name **without** the file extension.
    pub output_name: String,

    /// Frames per second of the recorded video.
    ///
    /// Higher values improve motion smoothness but increase encoding cost
    /// and file size.
    pub fps: u32,

    /// The container format of the output video (e.g. MP4, MOV).
    pub file_type: FileType,

    /// The pixel format used for encoding the video.
    ///
    /// This affects color fidelity, chroma subsampling, and codec compatibility.
    pub pixel_format: PixelFormat,

    /// The video codec used for compression and decompression.
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
