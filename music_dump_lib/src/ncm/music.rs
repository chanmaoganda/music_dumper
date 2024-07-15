use std::fs;

pub struct NcmMusic {
    
}

impl NcmMusic {
    
}

impl From<fs::File> for NcmMusic {
    fn from(mut file: fs::File) -> Self {


        NcmMusic {}
    }
}