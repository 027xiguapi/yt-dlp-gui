use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton},
    Emitter, Manager,
};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use uuid::Uuid;
use log::{info, error};
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
    _app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let task_id = task.id.clone();
    let task_id_clone = task_id.clone();
    let url = task.url.clone();
    let preset = task.preset.clone();
    let path = task.path.clone();

    info!(
        "Starting download task - ID: {}, URL: {}, Preset: {}, Path: {}",
        task_id, url, preset, path
    );

    tokio::spawn(async move {
        let last_error_msg = std::sync::Arc::new(tokio::sync::Mutex::new(String::new()));
        let last_error_msg_clone = last_error_msg.clone();

        let ytdlp_path = std::path::PathBuf::from(
            ytdlp_path.unwrap_or_else(|| "./win/yt-dlp.exe".to_string()),
        );

        if !ytdlp_path.exists() {
            error!("yt-dlp NOT found: {}", ytdlp_path.display());
            return;
        }

        // ========================
        // 构建命令
        // ========================
        let mut cmd = TokioCommand::new(&ytdlp_path);

        cmd.env("PYTHONIOENCODING", "utf-8")
            .env("PYTHONUTF8", "1")
            .env("PYTHONLEGACYWINDOWSSTDIO", "1")
            .env("LC_ALL", "C.UTF-8");

        cmd.arg("--newline")
            .arg("--progress")
            .arg("--no-warnings")
            .arg("--no-color")
            .arg("--encoding")
            .arg("utf-8")
            .arg("--progress-template")
            .arg("%(progress.status)s__SEP__%(progress._percent_str)s__SEP__%(progress._speed_str)s__SEP__%(progress._eta_str)s")
            .arg("-P")
            .arg(&path);

        // ========================
        // Cookie 逻辑
        // ========================
        let browser_val = cookies_from_browser.as_deref().unwrap_or("chrome");
        let is_custom_browser = browser_val.eq_ignore_ascii_case("custom");

        if !is_custom_browser {
            cmd.arg("--cookies-from-browser").arg(browser_val);
        } else if let Some(cookie_file) = cookie_path.as_deref() {
            if !cookie_file.is_empty() {
                cmd.arg("--cookies").arg(cookie_file);
            }
        }

        // ========================
        // 平台适配（重点）
        // ========================
        let is_youtube = url.contains("youtube.com") || url.contains("youtu.be");
        if url.contains("bilibili.com") {
            cmd.arg("-f").arg("bv*+ba/b");
        } else if is_youtube {
            cmd.arg("-f").arg(&preset);
        }

        cmd.arg(&url)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        info!("Executing: {:?}", cmd);

        // ========================
        // 启动进程
        // ========================
        match cmd.spawn() {
            Ok(mut child) => {
                let stdout = child.stdout.take().expect("stdout missing");
                let stderr = child.stderr.take().expect("stderr missing");

                let window_stdout = window.clone();
                let task_id_stdout = task_id_clone.clone();
                let task_id_stderr = task_id_clone.clone();

                // ========================
                // 读取 stdout（进度）
                // ========================
                let stdout_task = tokio::spawn(async move {
                    let reader = BufReader::new(stdout);
                    let mut lines = reader.lines();

                    while let Ok(Some(line)) = lines.next_line().await {
                        if line.contains("__SEP__") {
                            let parts: Vec<&str> = line.split("__SEP__").collect();

                            if parts.len() >= 6 {
                                let _ = window_stdout.emit(
                                    "download_progress",
                                    serde_json::json!({
                                        "id": task_id_stdout,
                                        "status": "Downloading",
                                        "size": parts[1].trim(),
                                        "progress": parts[2].trim().replace("%", ""),
                                        "speed": parts[3].trim(),
                                        "eta": parts[4].trim(),
                                        "title": parts[5].trim(),
                                    }),
                                );
                            }
                        } else if line.starts_with("[Merger]")
                            || line.starts_with("[ExtractAudio]")
                        {
                            let _ = window_stdout.emit(
                                "download_progress",
                                serde_json::json!({
                                    "id": task_id_stdout,
                                    "status": "Converting",
                                }),
                            );
                        }
                    }
                });

                // ========================
                // 读取 stderr（错误）
                // ========================
                let stderr_task = tokio::spawn(async move {
                    let reader = BufReader::new(stderr);
                    let mut lines = reader.lines();

                    while let Ok(Some(line)) = lines.next_line().await {
                        if !line.trim().is_empty() {
                            error!("[yt-dlp stderr][{}] {}", task_id_stderr, line);

                            let mut storage = last_error_msg_clone.lock().await;
                            *storage = line;
                        }
                    }
                });

                // ❗关键：等待两个流结束（防止 Windows pipe 崩）
                let _ = tokio::join!(stdout_task, stderr_task);

                // ========================
                // 等待进程结束
                // ========================
                match child.wait().await {
                    Ok(status) => {
                        if status.success() {
                            let _ = window.emit(
                                "download_progress",
                                serde_json::json!({
                                    "id": task_id_clone,
                                    "status": "Finished",
                                    "progress": "100",
                                }),
                            );
                        } else {
                            let err = last_error_msg.lock().await.clone();

                            let _ = window.emit(
                                "download_progress",
                                serde_json::json!({
                                    "id": task_id_clone,
                                    "status": "ERROR",
                                    "error": if err.is_empty() {
                                        "Download failed".to_string()
                                    } else {
                                        err
                                    },
                                }),
                            );
                        }
                    }
                    Err(e) => {
                        let _ = window.emit(
                            "download_progress",
                            serde_json::json!({
                                "id": task_id_clone,
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
                        "id": task_id_clone,
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
    _app_handle: tauri::AppHandle,
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
    _app_handle: tauri::AppHandle,
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
            let _stderr = String::from_utf8_lossy(&output.stderr).to_string();

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

#[tauri::command]
fn open_download_folder(path: String) -> Result<(), String> {
    use std::process::Command;

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    info!("Opening folder: {}", path);
    Ok(())
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
        .setup(|app| {
            let show = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "隐藏窗口", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &hide, &quit])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app_handle: &tauri::AppHandle, event| match event.id.as_ref() {
                    "quit" => app_handle.exit(0),
                    "show" => {
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    match event {
                        TrayIconEvent::Click { button: MouseButton::Left, .. } => {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show(); 
                                let _ = window.unminimize(); 
                                let _ = window.set_focus(); 
                            }
                        }
                        TrayIconEvent::Click { button: MouseButton::Right, .. } => {
                            // Right click: Show context menu (handled automatically)
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            // Handle window close event - hide window and keep running in background
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        info!("Close requested - hiding window");
                        api.prevent_close();
                        let _ = window_clone.hide();
                    }
                });
            }

            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            let _ = app.emit("single-instance", args.clone());

            // Show window when another instance is launched
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .invoke_handler(tauri::generate_handler![
            get_default_config,
            load_config,
            save_config,
            start_download,
            generate_task_id,
            extract_channel_urls,
            sniff_youtube_resources,
            check_version,
            open_download_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
