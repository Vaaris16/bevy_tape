use core::fmt;
/// The [`PixelFormat`] enum enumerates all pixel formats supported by bevy_tape.
///
/// The pixel format of a video affects file size, color accuracy,
/// compatibility with editing software, and system performance during playback.

#[derive(Debug, Clone, Copy)]
pub enum PixelFormat {
    /// **RGBA (8-bit)** stores red, green, blue, and alpha channels
    /// with no chroma subsampling.
    ///
    /// This format preserves exact color values and transparency but
    /// produces large data sizes and is rarely supported by delivery codecs.
    /// It is commonly used for rendering pipelines and intermediate buffers.
    Rgba,

    /// **YUV 4:2:0 (8-bit)** is the most widely used pixel format.
    ///
    /// It reduces chroma resolution in both horizontal and vertical directions,
    /// significantly lowering file size with minimal perceived quality loss.
    /// This format is required by many hardware decoders and web platforms.
    Yuv420p,

    /// **YUV 4:2:2 (8-bit)** retains full vertical chroma resolution.
    ///
    /// It offers improved color detail compared to 4:2:0 and is commonly
    /// used in professional acquisition and broadcast workflows.
    Yuv422p,

    /// **YUV 4:4:4 (8-bit)** preserves full chroma resolution with no subsampling.
    ///
    /// This format provides maximum color fidelity and is often used for
    /// compositing, VFX, and high-end post-production pipelines.
    Yuv444p,

    /// **YUV 4:2:0 (10-bit)** increases color precision per channel.
    ///
    /// The higher bit depth reduces banding artifacts and improves HDR
    /// and color-graded content while maintaining efficient compression.
    Yuv420p10,

    /// **YUV 4:2:2 (10-bit)** combines higher chroma detail with increased precision.
    ///
    /// This format is widely used in professional recording formats and
    /// editing codecs where color accuracy is critical.
    Yuv422p10,

    /// **YUV 4:4:4 (10-bit)** offers the highest color fidelity and precision.
    ///
    /// With no chroma subsampling and increased bit depth, this format is
    /// ideal for mastering, archival, and high-end cinematic workflows.
    Yuv444p10,
}

impl fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let px_format = match self {
            PixelFormat::Rgba => "rgba",
            PixelFormat::Yuv420p => "yuv420p",
            PixelFormat::Yuv422p => "yuv422p",
            PixelFormat::Yuv444p => "yuv444p",
            PixelFormat::Yuv420p10 => "yuv420p10",
            PixelFormat::Yuv422p10 => "yuv422p10",
            PixelFormat::Yuv444p10 => "yuv444p10",
        };

        f.write_str(px_format)
    }
}
