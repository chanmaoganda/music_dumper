use crate::MetaData;

pub struct NcmMusic {
    pub metadata: Box<dyn MetaData>,
    pub audio_data: Vec<u8>,
    pub music_type: String,
}



impl NcmMusic {
    pub fn new(metadata: Box<dyn MetaData>, music_type: String, audio_data: Vec<u8>) -> Self {
        Self {
            metadata,
            audio_data,
            music_type,
        }
    }
}