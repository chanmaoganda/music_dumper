#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    WebP,
    Bmp,
    Ico,
}

static MAGIC_BYTES: [(&[u8], ImageFormat); 7] = [
    (b"\x89PNG\r\n\x1a\n", ImageFormat::Png),
    (&[0xff, 0xd8, 0xff], ImageFormat::Jpeg),
    (b"GIF89a", ImageFormat::Gif),
    (b"GIF87a", ImageFormat::Gif),
    (b"RIFF", ImageFormat::WebP), // TODO: better magic byte detection, see https://github.com/image-rs/image/issues/660
    (b"BM", ImageFormat::Bmp),
    (&[0, 0, 1, 0], ImageFormat::Ico),
];

/// Guess image format from memory block
///
/// Makes an educated guess about the image format based on the Magic Bytes at the beginning.
/// TGA is not supported by this function.
/// This is not to be trusted on the validity of the whole memory block
/// This block is copied from `image` crate
pub fn guess_format(buffer: &[u8]) -> Option<ImageFormat> {
    for &(signature, format) in &MAGIC_BYTES {
        if buffer.starts_with(signature) {
            return Some(format);
        }
    }

    None
}
