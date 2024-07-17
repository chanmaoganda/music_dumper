use crate::NcmMetaData;

pub struct NcmMusic {
    pub metadata: NcmMetaData,
    pub audio_data: Vec<u8>,
    pub music_type: String,
}



impl NcmMusic {
    pub fn new(metadata: NcmMetaData, music_type: String, audio_data: Vec<u8>) -> Self {
        Self {
            metadata,
            audio_data,
            music_type,
        }
    }
}