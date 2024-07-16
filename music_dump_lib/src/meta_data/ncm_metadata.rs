use std::path::PathBuf;

use id3::{frame, TagLike};

use crate::{guess_pict_type, NcmInfo};

use super::metadata_trait::MetaData;

pub struct NcmMetaData {
    tag: id3::Tag,
}

impl NcmMetaData {
    pub fn new(ncm_info: NcmInfo, image: Vec<u8>) -> Self {
        let mut tag = id3::Tag::new();
        let artist_string = ncm_info.artist
            .into_iter()
            .map(|(name, _)| name)
            .collect::<Vec<String>>()
            .join("/");
        tag.set_artist(artist_string);
        tag.set_album(ncm_info.album);
        
        // TODO: add From<ImageFormat> for PictureType? a feature request for id3 crate?
        tag.add_frame(id3::frame::Picture {
            mime_type: guess_pict_type(&image),
            picture_type: frame::PictureType::CoverFront,
            description: "".to_owned(),
            data: image,
        });
        Self { tag }
    }
}

impl MetaData for NcmMetaData {
    fn inject(&self, path_buf: PathBuf) {
        self.tag.write_to_path(path_buf, id3::Version::Id3v23).unwrap();
    }
}