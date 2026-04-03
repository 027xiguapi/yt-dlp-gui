use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
    pub id: String,
    pub url: String,
    pub preset: String,
    pub path: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub progress: f32,
    #[serde(default)]
    pub speed: String,
    #[serde(default)]
    pub eta: String,
    #[serde(default)]
    pub size: String,
    #[serde(default)]
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub presets: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub current_preset: usize,
    pub path: String,
    pub global_args: String,
    pub update_ytdlp: bool,
    pub cookie_path: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut presets = HashMap::new();
        presets.insert(
            "best".to_string(),
            "-f bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4]/bv*+ba/b".to_string(),
        );
        presets.insert(
            "mp4".to_string(),
            "-f bv*[vcodec^=avc]+ba[ext=m4a]/b".to_string(),
        );
        presets.insert(
            "mp3".to_string(),
            "--extract-audio --audio-format mp3 --audio-quality 0".to_string(),
        );

        Config {
            general: GeneralConfig {
                current_preset: 0,
                path: dirs::download_dir()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                global_args: "--cookies-from-browser firefox".to_string(),
                update_ytdlp: true,
                cookie_path: String::new(),
            },
            presets,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelExtractionResult {
    pub urls: Vec<String>,
    pub channel_name: String,
    pub total_videos: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedResource {
    pub url: String,
    pub resource_type: String,
    pub mime_type: String,
    pub size: u64,
}
