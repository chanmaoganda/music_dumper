use std::path::PathBuf;

use crate::{FlacMetadata, Mp3MetaData, NcmInfo};

pub trait MetaData {
    fn inject(&mut self, path_buf: &PathBuf);
}

pub fn ncm_metadata_builder(music_type: &str, ncm_info: NcmInfo, image: Vec<u8>) -> Box<dyn MetaData> {
    if music_type == "mp3" {
        Box::new(Mp3MetaData::new(ncm_info, image))
    } else if music_type == "flac" {
        Box::new(FlacMetadata::new(ncm_info, image))
    } else {
        panic!("Unsupported music type: {}", music_type);
    }
}