use crate::{guess_pict_type, MetaData, NcmInfo};

pub struct FlacMetadata {
    tag: metaflac::Tag,
}

impl FlacMetadata {
    pub fn new(ncm_info: NcmInfo, image: Vec<u8>) -> Self {
        let mut tag = metaflac::Tag::new();
        let artist_vec = ncm_info.artist
            .into_iter()
            .map(|(name, _)| name)
            .collect::<Vec<String>>();
        let mc = tag.vorbis_comments_mut();
        mc.set_artist(artist_vec);
        mc.set_album(vec![ncm_info.album]);
        mc.set_title(vec![ncm_info.name]);
        // TODO: add From<ImageFormat> for PictureType? a feature request for id3 crate?
        tag.add_picture(
            guess_pict_type(&image),
            metaflac::block::PictureType::CoverFront,
            image);
        Self { tag }
    }
}

impl MetaData for FlacMetadata {
    fn inject(&mut self, path_buf: &std::path::PathBuf) {
        self.tag.write_to_path(path_buf).unwrap();
    }
}