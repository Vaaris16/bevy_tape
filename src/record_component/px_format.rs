use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PixelFormat {
    Rgba,
    Yuv420p,
    Yuv422p,
    Yuv444p,
    Yuv420p10,
    Yuv422p10,
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
