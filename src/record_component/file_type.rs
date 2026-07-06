use core::fmt;
/// The [`FileType`] enum enumerates all container formats supported by bevy_tape.
///
/// File types affect compatibility with devices and browsers, file size,
/// and streaming smoothness.

#[derive(Clone, Debug, Copy)]
pub enum FileType {
    /// **MP4 (MPEG-4 Part 14)** is the most widely supported container format.
    ///
    /// It is commonly used for web delivery and consumer playback and
    /// typically stores H.264, H.265, or AV1 video with AAC or Opus audio.
    /// MP4 does **not** support codecs such as ProRes.
    MP4,

    /// **MOV (QuickTime File Format)** is a flexible container developed by Apple.
    ///
    /// It is widely used in professional video workflows and supports a broad
    /// range of codecs, including ProRes, H.264, and H.265, along with rich metadata.
    MOV,

    /// **MKV (Matroska)** is an open and extensible container format.
    ///
    /// It supports virtually any video or audio codec, advanced subtitle formats,
    /// and multiple audio/video tracks, making it ideal for archival and distribution.
    MKV,

    /// **WebM** is an open, royalty-free container optimized for web streaming.
    ///
    /// It is primarily designed for VP9 and AV1 video with Opus or Vorbis audio
    /// and is natively supported by modern web browsers.
    WEBM,

    /// **AVI (Audio Video Interleave)** is a legacy container developed by Microsoft.
    ///
    /// While still supported by many players, it has limited metadata support
    /// and poor compatibility with modern codecs, making it unsuitable for
    /// contemporary workflows.
    AVI,

    /// **MPEG (Program Stream)** is an older container format used in early
    /// digital video and DVD media.
    ///
    /// It is primarily associated with MPEG-1 and MPEG-2 video and offers
    /// limited support for modern codecs and features.
    MPEG,

    /// **FLV (Flash Video)** is a legacy container format designed for Adobe Flash.
    ///
    /// It was historically used for web streaming but is now obsolete due to
    /// the deprecation of Flash and lack of modern codec support.
    FLV,

    /// **MTS (AVCHD Transport Stream)** is a container format used by consumer
    /// and professional camcorders.
    ///
    /// It typically stores H.264 video with AC-3 or PCM audio and is optimized
    /// for recording rather than distribution or streaming.
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
