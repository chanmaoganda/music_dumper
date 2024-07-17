use std::{fs::File, io::Write, path::PathBuf};

use crate::{NcmDecoder, NcmMusic};

use rayon::prelude::*;

pub struct NcmDumper {
    music_list: Vec<PathBuf>,
    output_directory: PathBuf
}

impl NcmDumper {
    pub fn new(music_list: Vec<PathBuf>, output_directory: PathBuf) -> Self {
        Self { music_list, output_directory }
    }

    pub fn dump_all(self) -> anyhow::Result<()> {
        self.music_list.into_par_iter()
            .for_each(|path_buf| {
                Self::dump(&path_buf, &self.output_directory).unwrap();
            });
        Ok(())
    }

    fn dump(path_buf: &PathBuf, output_directory: &PathBuf) -> anyhow::Result<()> {
        let mut decoder = NcmDecoder::new(path_buf);
        let NcmMusic { 
            mut metadata,
            music_type,
            audio_data 
        } = decoder.decode()?;
        let file_name = path_buf.file_name().unwrap();
        let path = output_directory.join(file_name).with_extension(music_type);
        let mut file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?;
        file.write_all(&audio_data)?;
        metadata.inject(&path);
        Ok(())
    }
}