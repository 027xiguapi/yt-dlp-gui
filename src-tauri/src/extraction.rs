use crate::models::{ChannelExtractionResult, CapturedResource};
use crate::logger;
use tauri::Emitter;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::process::Command as TokioCommand;

#[tauri::command]
pub async fn extract_channel_urls(
    channel_url: String,
    window: tauri::Window,
    ytdlp_path: Option<String>,
    _app_handle: tauri::AppHandle,
) -> Result<ChannelExtractionResult, String> {
    logger::log_to_file(&format!("Starting channel extraction for URL: {}", channel_url));

    let ytdlp_path = std::path::PathBuf::from(
        ytdlp_path.unwrap_or_else(|| "./win/yt-dlp.exe".to_string())
    );

    let mut cmd = TokioCommand::new(&ytdlp_path);
    cmd.arg("--flat-playlist")
        .arg("--dump-json")
        .arg("--no-warnings")
        .arg(&channel_url)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    match cmd.spawn() {
        Ok(mut child) => {
            logger::log_to_file("yt-dlp process spawned for channel extraction");
            let mut urls = Vec::new();
            let mut channel_name = String::new();

            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                let mut line_count = 0;

                while let Ok(Some(line)) = lines.next_line().await {
                    if let Ok(entry) = serde_json::from_str::<serde_json::Value>(&line) {
                        line_count += 1;

                        if line_count == 1 {
                            if let Some(name) = entry.get("channel").and_then(|v| v.as_str()) {
                                channel_name = name.to_string();
                            } else if let Some(name) = entry.get("uploader").and_then(|v| v.as_str()) {
                                channel_name = name.to_string();
                            }
                        }

                        if let Some(url) = entry.get("url").and_then(|v| v.as_str()) {
                            urls.push(url.to_string());
                        } else if let Some(id) = entry.get("id").and_then(|v| v.as_str()) {
                            urls.push(format!("https://www.youtube.com/watch?v={}", id));
                        } else if let Some(url) = entry.get("webpage_url").and_then(|v| v.as_str()) {
                            urls.push(url.to_string());
                        }

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
pub async fn sniff_youtube_resources(
    video_url: String,
    window: tauri::Window,
    _app_handle: tauri::AppHandle,
) -> Result<(Vec<CapturedResource>, Vec<CapturedResource>), String> {
    logger::log_to_file(&format!("Starting resource sniffing for URL: {}", video_url));

    let ytdlp_path = std::path::PathBuf::from("./win/yt-dlp.exe");

    let mut cmd = TokioCommand::new(&ytdlp_path);
    cmd.arg("--dump-json")
        .arg("--no-warnings")
        .arg(&video_url)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    match cmd.spawn() {
        Ok(mut child) => {
            logger::log_to_file("yt-dlp process spawned for resource sniffing");
            let mut videos = Vec::new();
            let mut images = Vec::new();

            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();

                while let Ok(Some(line)) = lines.next_line().await {
                    if let Ok(info) = serde_json::from_str::<serde_json::Value>(&line) {
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
