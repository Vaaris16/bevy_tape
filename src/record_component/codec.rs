use core::fmt;

/// Video codecs affect file size, visual quality, and compatibility
/// across devices and platforms.
///
/// The [`Codec`] enum defines all video codecs supported by bevy_tape.

#[derive(Clone, Debug, Copy)]
pub enum Codec {
    /// **H.264 (AVC)** is the most widely supported video codec.
    ///
    /// It is natively supported by virtually all web browsers, operating systems,
    /// smartphones, and video editing platforms, making it the default and safest
    /// choice for maximum compatibility.
    H264,

    /// **H.265 (HEVC)** provides significantly better compression efficiency than H.264,
    /// typically achieving similar visual quality at up to ~50% lower bitrates.
    ///
    /// While it supports high-resolution content (4K and beyond), hardware and
    /// browser support is more limited compared to H.264.
    H265,

    /// **VP9** is an open and royalty-free codec developed by Google.
    ///
    /// It offers substantially better compression than H.264 (often 30–50% smaller files)
    /// and is widely supported for web streaming in modern browsers.
    Vp9,

    /// **AV1** is a next-generation, open, and royalty-free codec.
    ///
    /// It delivers superior compression efficiency compared to VP9 and HEVC
    /// and is designed for high-resolution content, including 4K and 8K.
    /// Encoding is computationally expensive, and hardware support is still emerging.
    Av1,

    /// **ProRes** is a high-quality intra-frame codec developed by Apple.
    ///
    /// It prioritizes visual fidelity, high bitrates, and fast decode performance,
    /// making it ideal for professional editing and post-production workflows.
    ProRes,

    /// **Raw video** stores uncompressed pixel data with no loss or processing.
    ///
    /// This produces extremely large files but preserves exact frame data,
    /// making it useful for debugging, testing, or custom processing pipelines.
    Raw,
}

impl fmt::Display for Codec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let codec = match self {
            Codec::H264 => "libx264",
            Codec::H265 => "libx265",
            Codec::Vp9 => "libvpx-vp9",
            Codec::Av1 => "libaom-av1",
            Codec::ProRes => "prores_ks",
            Codec::Raw => "rawvideo",
        };

        f.write_str(codec)
    }
}
