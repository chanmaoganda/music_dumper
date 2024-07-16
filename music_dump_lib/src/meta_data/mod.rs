mod metadata_trait;
mod ncm_metadata;
mod qmc_metadata;

use image::{guess_format, ImageFormat};
pub use ncm_metadata::*;
pub use metadata_trait::MetaData;

pub fn guess_pict_type(data: &Vec<u8>) -> String {
    let format = guess_format(&data).unwrap_or(ImageFormat::Jpeg);
    match format {
        ImageFormat::Png => "png".to_owned(),
        ImageFormat::Jpeg => "jpeg".to_owned(),
        ImageFormat::Bmp => "bmp".to_owned(),
        ImageFormat::Ico => "ico".to_owned(),
        _ => "unknown".to_owned(),
    }
}