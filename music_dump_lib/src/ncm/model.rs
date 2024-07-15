use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NcmInfo {
    #[serde(rename = "musicName")]
    pub name: String,
    #[serde(rename = "musicId")]
    pub id: u32,
    pub album: String,
    pub artist: Vec<(String, u32)>,
    pub bitrate: u32,
    pub duration: u32,
    pub format: String,
    #[serde(rename = "mvId")]
    pub mv_id: Option<u32>,
    pub alias: Option<Vec<String>>,
}