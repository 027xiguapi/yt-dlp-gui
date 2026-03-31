use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::Emitter;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
    pub id: String,
    pub url: String,
    pub preset: String,
    pub path: String,
    pub status: String,
    pub progress: f32,
    pub speed: String,
    pub eta: String,
    pub size: String,
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

#[tauri::command]
fn get_default_config() -> Config {
    Config::default()
}

#[tauri::command]
fn load_config(path: String) -> Result<Config, String> {
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read config: {}", e))?;
    toml::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))
}

#[tauri::command]
fn save_config(path: String, config: Config) -> Result<(), String> {
    let content = toml::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    std::fs::write(&path, content)
        .map_err(|e| format!("Failed to write config: {}", e))
}

#[tauri::command]
async fn start_download(
    task: DownloadTask,
    window: tauri::Window,
    cookie_path: Option<String>,
) -> Result<String, String> {
    let task_id = task.id.clone();
    let task_id_clone = task_id.clone();
    let url = task.url.clone();
    let preset = task.preset.clone();
    let path = task.path.clone();

    tokio::spawn(async move {
        let mut cmd = TokioCommand::new("yt-dlp");
        cmd.arg("--newline")
            .arg("--no-simulate")
            .arg("--progress")
            .arg("--progress-template")
            .arg("%(progress.status)s__SEP__%(progress._total_bytes_estimate_str)s__SEP__%(progress._percent_str)s__SEP__%(progress._speed_str)s__SEP__%(progress._eta_str)s__SEP__%(info.title)s")
            .arg("-P")
            .arg(&path);

        // Add cookie file if provided
        if let Some(cookie_file) = cookie_path {
            if !cookie_file.is_empty() {
                cmd.arg("--cookies").arg(&cookie_file);
            }
        }

        cmd.arg(&preset)
            .arg("--")
            .arg(&url)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        match cmd.spawn() {
            Ok(mut child) => {
                if let Some(stdout) = child.stdout.take() {
                    let reader = BufReader::new(stdout);
                    let mut lines = reader.lines();

                    while let Ok(Some(line)) = lines.next_line().await {
                        if line.contains("__SEP__") {
                            let parts: Vec<&str> = line.split("__SEP__").collect();
                            if parts.len() >= 6 {
                                let progress_data = serde_json::json!({
                                    "id": task_id_clone.clone(),
                                    "status": "Downloading",
                                    "size": parts[1].trim(),
                                    "progress": parts[2].trim().replace("%", ""),
                                    "speed": parts[3].trim(),
                                    "eta": parts[4].trim(),
                                    "title": parts[5].trim(),
                                });
                                let _ = window.emit("download_progress", progress_data);
                            }
                        } else if line.starts_with("[Merger]") || line.starts_with("[ExtractAudio]")
                        {
                            let _ = window.emit(
                                "download_progress",
                                serde_json::json!({
                                    "id": task_id_clone.clone(),
                                    "status": "Converting",
                                }),
                            );
                        }
                    }
                }

                let status = child.wait().await;
                match status {
                    Ok(exit_status) => {
                        if exit_status.success() {
                            let _ = window.emit(
                                "download_progress",
                                serde_json::json!({
                                    "id": task_id_clone.clone(),
                                    "status": "Finished",
                                    "progress": "100",
                                }),
                            );
                        } else {
                            let _ = window.emit(
                                "download_progress",
                                serde_json::json!({
                                    "id": task_id_clone.clone(),
                                    "status": "ERROR",
                                }),
                            );
                        }
                    }
                    Err(e) => {
                        let _ = window.emit(
                            "download_progress",
                            serde_json::json!({
                                "id": task_id_clone.clone(),
                                "status": "ERROR",
                                "error": e.to_string(),
                            }),
                        );
                    }
                }
            }
            Err(e) => {
                let _ = window.emit(
                    "download_progress",
                    serde_json::json!({
                        "id": task_id_clone.clone(),
                        "status": "ERROR",
                        "error": e.to_string(),
                    }),
                );
            }
        }
    });

    Ok(task_id)
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

#[tauri::command]
fn generate_task_id() -> String {
    Uuid::new_v4().to_string()
}

#[tauri::command]
async fn extract_channel_urls(
    channel_url: String,
    window: tauri::Window,
) -> Result<ChannelExtractionResult, String> {
    let mut cmd = TokioCommand::new("yt-dlp");
    cmd.arg("--flat-playlist")
        .arg("--dump-json")
        .arg("--no-warnings")
        .arg(&channel_url)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    match cmd.spawn() {
        Ok(mut child) => {
            let mut urls = Vec::new();
            let mut channel_name = String::new();

            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                let mut line_count = 0;

                while let Ok(Some(line)) = lines.next_line().await {
                    if let Ok(entry) = serde_json::from_str::<serde_json::Value>(&line) {
                        line_count += 1;

                        // Extract channel name from first entry
                        if line_count == 1 {
                            if let Some(name) = entry.get("channel").and_then(|v| v.as_str()) {
                                channel_name = name.to_string();
                            } else if let Some(name) = entry.get("uploader").and_then(|v| v.as_str()) {
                                channel_name = name.to_string();
                            }
                        }

                        // Extract video URL
                        if let Some(url) = entry.get("url").and_then(|v| v.as_str()) {
                            urls.push(url.to_string());
                        } else if let Some(id) = entry.get("id").and_then(|v| v.as_str()) {
                            urls.push(format!("https://www.youtube.com/watch?v={}", id));
                        } else if let Some(url) = entry.get("webpage_url").and_then(|v| v.as_str()) {
                            urls.push(url.to_string());
                        }

                        // Emit progress
                        let progress = (line_count as f32 / 100.0).min(1.0);
                        let _ = window.emit(
                            "extraction_progress",
                            serde_json::json!({
                                "progress": (progress * 100.0) as u32,
                                "count": line_count,
                            }),
                        );
                    }
                }
            }

            let _ = child.wait().await;

            if urls.is_empty() {
                return Err("No videos found in channel".to_string());
            }

            Ok(ChannelExtractionResult {
                total_videos: urls.len(),
                urls,
                channel_name,
            })
        }
        Err(e) => Err(format!("Failed to execute yt-dlp: {}", e)),
    }
}

#[tauri::command]
async fn sniff_youtube_resources(
    video_url: String,
    window: tauri::Window,
) -> Result<(Vec<CapturedResource>, Vec<CapturedResource>), String> {
    let mut cmd = TokioCommand::new("yt-dlp");
    cmd.arg("--dump-json")
        .arg("--no-warnings")
        .arg(&video_url)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    match cmd.spawn() {
        Ok(mut child) => {
            let mut videos = Vec::new();
            let mut images = Vec::new();

            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();

                while let Ok(Some(line)) = lines.next_line().await {
                    if let Ok(info) = serde_json::from_str::<serde_json::Value>(&line) {
                        // Extract video formats
                        if let Some(formats) = info.get("formats").and_then(|v| v.as_array()) {
                            for format in formats {
                                if let Some(url) = format.get("url").and_then(|v| v.as_str()) {
                                    let mime_type = format
                                        .get("mime_type")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("video/unknown")
                                        .to_string();

                                    let filesize = format
                                        .get("filesize")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0);

                                    if mime_type.starts_with("video/") {
                                        videos.push(CapturedResource {
                                            url: url.to_string(),
                                            resource_type: "video".to_string(),
                                            mime_type,
                                            size: filesize,
                                        });
                                    }
                                }
                            }
                        }

                        // Extract thumbnail images
                        if let Some(thumbnails) = info.get("thumbnails").and_then(|v| v.as_array()) {
                            for thumb in thumbnails {
                                if let Some(url) = thumb.get("url").and_then(|v| v.as_str()) {
                                    let filesize = thumb
                                        .get("width")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0)
                                        * thumb
                                            .get("height")
                                            .and_then(|v| v.as_u64())
                                            .unwrap_or(0);

                                    images.push(CapturedResource {
                                        url: url.to_string(),
                                        resource_type: "image".to_string(),
                                        mime_type: "image/jpeg".to_string(),
                                        size: filesize,
                                    });
                                }
                            }
                        }

                        let _ = window.emit(
                            "sniff_progress",
                            serde_json::json!({
                                "videos": videos.len(),
                                "images": images.len(),
                            }),
                        );
                    }
                }
            }

            let _ = child.wait().await;

            if videos.is_empty() && images.is_empty() {
                return Err("No resources found".to_string());
            }

            Ok((videos, images))
        }
        Err(e) => Err(format!("Failed to execute yt-dlp: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_default_config,
            load_config,
            save_config,
            start_download,
            generate_task_id,
            extract_channel_urls,
            sniff_youtube_resources,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
