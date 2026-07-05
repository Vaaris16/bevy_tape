use core::fmt;

#[derive(Clone, Debug, Copy)]
pub enum Codec {
    H264,
    H265,
    Vp9,
    Av1,
    ProRes,
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
