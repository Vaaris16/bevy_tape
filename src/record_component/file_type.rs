use core::fmt;

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
