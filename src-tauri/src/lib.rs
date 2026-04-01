use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{Emitter, Manager};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use uuid::Uuid;
use log::{info, error, debug};
use std::path::Path;

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
    ytdlp_path: Option<String>,
    cookies_from_browser: Option<String>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let task_id = task.id.clone();
    let task_id_clone = task_id.clone();
    let url = task.url.clone();
    let preset = task.preset.clone();
    let path = task.path.clone();

    info!("Starting download task - ID: {}, URL: {}, Preset: {}, Path: {}", task_id, url, preset, path);

    tokio::spawn(async move {
        let last_error_msg = std::sync::Arc::new(tokio::sync::Mutex::new(String::new()));
        let last_error_msg_clone = last_error_msg.clone();
        let ytdlp_path = std::path::PathBuf::from(
            ytdlp_path.unwrap_or_else(|| "./win/yt-dlp.exe".to_string())
        );
        info!("Executing yt-dlp from: {}", ytdlp_path.display());

        // Check if file exists
        if ytdlp_path.exists() {
            info!("yt-dlp.exe found at: {}", ytdlp_path.display());
        } else {
            error!("yt-dlp.exe NOT found at: {}", ytdlp_path.display());
            // Try absolute path
            if let Ok(current_dir) = std::env::current_dir() {
                error!("Current working directory: {}", current_dir.display());
                let abs_path = current_dir.join(&ytdlp_path);
                error!("Absolute path would be: {}", abs_path.display());
            }
        }

        let mut cmd = TokioCommand::new(&ytdlp_path);
        cmd.env("PYTHONIOENCODING", "utf-8")
            .arg("--newline")
            .arg("--no-simulate")
            .arg("--progress")
            .arg("--progress-template")
            .arg("%(progress.status)s__SEP__%(progress._total_bytes_estimate_str)s__SEP__%(progress._percent_str)s__SEP__%(progress._speed_str)s__SEP__%(progress._eta_str)s__SEP__%(info.title)s")
            .arg("-P")
            .arg(&path);

        // --cookies 和 --cookies-from-browser 互斥，优先使用 cookie 文件
        // 当 browser 为 custom 时，强制使用 cookie 文件
        let browser_val = cookies_from_browser.as_deref().unwrap_or_default();
        let is_custom_browser = browser_val.eq_ignore_ascii_case("custom");

        let browser_active = !is_custom_browser && !browser_val.is_empty();
        let cookie_file_active = is_custom_browser || (!browser_active && cookie_path.as_deref().map(|s| !s.is_empty()).unwrap_or(false));

        if browser_active {
            info!("Using cookies from browser: {}", browser_val);
            cmd.arg("--cookies-from-browser").arg(browser_val);
        } else if cookie_file_active {
            if let Some(cookie_file) = cookie_path.as_deref() {
                info!("Using cookie file: {}", cookie_file);
                cmd.arg("--cookies").arg(cookie_file);
            }
        }

        // 只有 YouTube 链接才附加 -f 格式参数
        let is_youtube = url.contains("youtube.com") || url.contains("youtu.be");
        if is_youtube {
            cmd.arg("-f").arg(&preset);
        }
        cmd.arg(&url)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        // Log the complete command
        let mut cmd_args = vec![
            "--newline".to_string(),
            "--no-simulate".to_string(),
            "--progress".to_string(),
            "--progress-template".to_string(),
            "%(progress.status)s__SEP__%(progress._total_bytes_estimate_str)s__SEP__%(progress._percent_str)s__SEP__%(progress._speed_str)s__SEP__%(progress._eta_str)s__SEP__%(info.title)s".to_string(),
            "-P".to_string(),
            path.clone(),
        ];

        // 日志同步：优先使用浏览器 cookie，否则使用 cookie 文件
        if browser_active {
            cmd_args.push("--cookies-from-browser".to_string());
            cmd_args.push(cookies_from_browser.as_deref().unwrap().to_string());
        } else if cookie_file_active {
            cmd_args.push("--cookies".to_string());
            cmd_args.push(cookie_path.as_deref().unwrap().to_string());
        }

        if is_youtube {
            cmd_args.push("-f".to_string());
            cmd_args.push(preset.clone());
        }
        cmd_args.push(url.clone());

        info!("Executing command: {} {}", ytdlp_path.display(), cmd_args.join(" "));

        match cmd.spawn() {
            Ok(mut child) => {
                info!("yt-dlp process spawned successfully for task: {}", task_id_clone);
                // if let Some(stdout) = child.stdout.take() {
                //     let reader = BufReader::new(stdout);
                //     let mut lines = reader.lines();

                    // while let Ok(Some(line)) = lines.next_line().await {
                    //     if line.contains("__SEP__") {
                    //         let parts: Vec<&str> = line.split("__SEP__").collect();
                    //         if parts.len() >= 6 {
                    //             debug!("Progress update - Size: {}, Progress: {}, Speed: {}, ETA: {}",
                    //                 parts[1].trim(), parts[2].trim(), parts[3].trim(), parts[4].trim());
                    //             let progress_data = serde_json::json!({
                    //                 "id": task_id_clone.clone(),
                    //                 "status": "Downloading",
                    //                 "size": parts[1].trim(),
                    //                 "progress": parts[2].trim().replace("%", ""),
                    //                 "speed": parts[3].trim(),
                    //                 "eta": parts[4].trim(),
                    //                 "title": parts[5].trim(),
                    //             });
                    //             let _ = window.emit("download_progress", progress_data);
                    //         }
                    //     } else if line.starts_with("[Merger]") || line.starts_with("[ExtractAudio]")
                    //     {
                    //         info!("Converting file for task: {}", task_id_clone);
                    //         let _ = window.emit(
                    //             "download_progress",
                    //             serde_json::json!({
                    //                 "id": task_id_clone.clone(),
                    //                 "status": "Converting",
                    //             }),
                    //         );
                    //     }
                    // }
                // }

                // 收集 stderr 错误信息
                let mut stderr_output = String::new();
                if let Some(stderr) = child.stderr.take() {
                    let task_id_err = task_id_clone.clone();
                    let err_msg_store = last_error_msg_clone.clone(); // 克隆引用
                    tokio::spawn(async move {
                        let reader = BufReader::new(stderr);
                        let mut lines = reader.lines();
                        while let Ok(Some(line)) = lines.next_line().await {
                            if !line.trim().is_empty() {
                                error!("[yt-dlp stderr][{}] {}", task_id_err, line);
                                // 保存最后一行有效错误信息
                                let mut storage = err_msg_store.lock().await;
                                *storage = line; 
                            }
                        }
                    });
                }

                let status = child.wait().await;
                match status {
                    Ok(exit_status) => {
                        if exit_status.success() {
                            info!("Download completed successfully for task: {}", task_id_clone);
                            let _ = window.emit(
                                "download_progress",
                                serde_json::json!({
                                    "id": task_id_clone.clone(),
                                    "status": "Finished",
                                    "progress": "100",
                                }),
                            );
                        } else {
                            // 获取捕获到的错误信息
                            let final_error = last_error_msg.lock().await.clone();
                            let error_to_show = if final_error.is_empty() {
                                "Process exited with error (check console)".to_string()
                            } else {
                                final_error
                            };

                            error!("Download failed: {}", error_to_show);
                            let _ = window.emit(
                                "download_progress",
                                serde_json::json!({
                                    "id": task_id_clone.clone(),
                                    "status": "ERROR",
                                    "error": error_to_show, // 这里的 error 字段现在包含了具体原因
                                }),
                            );
                        }
                    }
                    Err(e) => {
                        error!("Download process error for task {}: {}", task_id_clone, e);
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
    ytdlp_path: Option<String>,
    app_handle: tauri::AppHandle,
) -> Result<ChannelExtractionResult, String> {
    info!("Starting channel extraction for URL: {}", channel_url);

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
            info!("yt-dlp process spawned for channel extraction");
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
    app_handle: tauri::AppHandle,
) -> Result<(Vec<CapturedResource>, Vec<CapturedResource>), String> {
    info!("Starting resource sniffing for URL: {}", video_url);

    let ytdlp_path = std::path::PathBuf::from("./win/yt-dlp.exe");

    let mut cmd = TokioCommand::new(&ytdlp_path);
    cmd.arg("--dump-json")
        .arg("--no-warnings")
        .arg(&video_url)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    match cmd.spawn() {
        Ok(mut child) => {
            info!("yt-dlp process spawned for resource sniffing");
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

#[tauri::command]
async fn check_version(cmd: String, args: Vec<String>, ytdlp_path: Option<String>) -> Result<String, String> {
    let ytdlp_path_str = ytdlp_path.as_deref().unwrap_or_default();
    info!(
        "check_version called - cmd: {}, args: {:?}, ytdlp_path: {}",
        cmd, args, ytdlp_path_str
    );

    let mut command = TokioCommand::new(&cmd);
    command.args(&args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    info!("Executing: {} {}", cmd, args.join(" "));

    match command.output().await {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if output.status.success() {
                info!("Command succeeded: {}", stdout.trim());
                return Ok(stdout.trim().to_string());
            }
        }
        Err(e) => {
            info!("Command failed: {}", e);
        }
    }

    let exe_dir = Path::new(ytdlp_path_str)
        .parent()
        .ok_or_else(|| format!("Invalid ytdlp_path: {}", ytdlp_path_str))?;

    let exe_dir_str = exe_dir.to_string_lossy().to_string();
    info!("exe_dir: {}", exe_dir_str);

    let fallback_paths = match cmd.as_str() {
        "yt-dlp" => vec![ytdlp_path_str.to_string(), "yt-dlp".to_string()],
        "deno" => vec![
            exe_dir.join("deno.exe").to_string_lossy().to_string(),
            "deno".to_string(),
        ],
        "ffmpeg" => vec![
            exe_dir.join("ffmpeg.exe").to_string_lossy().to_string(),
            "ffmpeg".to_string(),
        ],
        "ffprobe" => vec![
            exe_dir.join("ffprobe.exe").to_string_lossy().to_string(),
            "ffprobe".to_string(),
        ],
        _ => return Err(format!("Failed to execute {}", cmd)),
    };

    info!("fallback_paths: {:?}", fallback_paths);

    for path in fallback_paths {
        let mut fallback_cmd = TokioCommand::new(&path);
        fallback_cmd.args(&args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        info!("Trying fallback: {} {}", path, args.join(" "));

        if let Ok(output) = fallback_cmd.output().await {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if output.status.success() {
                info!("Fallback succeeded: {}", stdout.trim());
                return Ok(stdout.trim().to_string());
            } else if !stderr.is_empty() {
                info!("Fallback stderr: {}", stderr.trim());
                return Ok(stderr.trim().to_string());
            }
        }
    }

    error!("Failed to execute {} with args {:?}", cmd, args);
    Err(format!("Failed to execute {}", cmd))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Starting Tauri application");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_default_config,
            load_config,
            save_config,
            start_download,
            generate_task_id,
            extract_channel_urls,
            sniff_youtube_resources,
            check_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
