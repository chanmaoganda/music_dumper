use std::path::PathBuf;

use crate::NcmMetaData;

pub struct NcmMusic {
    pub metadata: NcmMetaData,
    pub path_buf: PathBuf,
    pub audio_data: Vec<u8>,
}

impl NcmMusic {
    pub fn new(metadata: NcmMetaData, path_buf: PathBuf, audio_data: Vec<u8>) -> Self {
        Self {
            metadata,
            path_buf,
            audio_data,
        }
    }
}