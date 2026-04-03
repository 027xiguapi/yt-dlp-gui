use crate::models::DownloadTask;
use crate::models::VideoInfo;
use crate::models::ThumbnailInfo;
use crate::logger;
use tauri::Emitter;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::process::Command as TokioCommand;

#[tauri::command]
pub async fn start_download(
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

    logger::log_to_file(&format!(
        "Starting download task - ID: {}, URL: {}, Preset: {}, Path: {}",
        task_id, url, preset, path
    ));

    tokio::spawn(async move {
        let last_error_msg = std::sync::Arc::new(tokio::sync::Mutex::new(String::new()));
        let last_error_msg_clone = last_error_msg.clone();

        let ytdlp_path = std::path::PathBuf::from(
            ytdlp_path.unwrap_or_else(|| "./win/yt-dlp.exe".to_string()),
        );

        if !ytdlp_path.exists() {
            logger::log_to_file(&format!("yt-dlp NOT found: {}", ytdlp_path.display()));
            return;
        }

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

        let browser_val = cookies_from_browser.as_deref().unwrap_or("chrome");
        let is_custom_browser = browser_val.eq_ignore_ascii_case("custom");

        if !is_custom_browser {
            cmd.arg("--cookies-from-browser").arg(browser_val);
        } else if let Some(cookie_file) = cookie_path.as_deref() {
            if !cookie_file.is_empty() {
                cmd.arg("--cookies").arg(cookie_file);
            }
        }

        let is_youtube = url.contains("youtube.com") || url.contains("youtu.be");
        if url.contains("bilibili.com") {
            cmd.arg("-f").arg("bv*+ba/b");
        } else if is_youtube {
            cmd.arg("-f").arg(&preset);
        }

        cmd.arg(&url)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        logger::log_to_file(&format!("Executing: {:?}", cmd));

        match cmd.spawn() {
            Ok(mut child) => {
                let stdout = child.stdout.take().expect("stdout missing");
                let stderr = child.stderr.take().expect("stderr missing");

                let window_stdout = window.clone();
                let task_id_stdout = task_id_clone.clone();
                let task_id_stderr = task_id_clone.clone();

                let stdout_task = tokio::spawn(async move {
                    let reader = BufReader::new(stdout);
                    let mut lines = reader.lines();

                    while let Ok(Some(line)) = lines.next_line().await {
                        if line.contains("__SEP__") {
                            let parts: Vec<&str> = line.split("__SEP__").collect();
                            logger::log_to_file(&format!("[Download Progress] Raw line parts: {:?}", parts));

                            if parts.len() >= 4 {
                                let status = parts[0].trim();
                                let progress_str = parts[1].trim().replace("%", "").trim().to_string();
                                let speed = parts[2].trim().to_string();
                                let eta = parts[3].trim().to_string();

                                let progress_value: f32 = progress_str.parse().unwrap_or(0.0);

                                logger::log_to_file(&format!(
                                    "[Download Progress] ID: {}, Status: {}, Progress: {}%, Speed: {}, ETA: {}",
                                    task_id_stdout, status, progress_value, speed, eta
                                ));

                                let _ = window_stdout.emit(
                                    "download_progress",
                                    serde_json::json!({
                                        "id": task_id_stdout,
                                        "status": "Downloading",
                                        "progress": progress_value,
                                        "size": status,
                                        "speed": speed,
                                        "eta": eta,
                                    }),
                                );
                            }
                        } else if line.starts_with("[Merger]")
                            || line.starts_with("[ExtractAudio]")
                        {
                            logger::log_to_file(&format!("[Download Progress] ID: {} - Converting", task_id_stdout));
                            let _ = window_stdout.emit(
                                "download_progress",
                                serde_json::json!({
                                    "id": task_id_stdout,
                                    "status": "Converting",
                                }),
                            );
                        } else if !line.trim().is_empty() {
                            logger::log_to_file(&format!("[yt-dlp stdout][{}] {}", task_id_stdout, line));
                        }
                    }
                });

                let stderr_task = tokio::spawn(async move {
                    let reader = BufReader::new(stderr);
                    let mut lines = reader.lines();

                    while let Ok(Some(line)) = lines.next_line().await {
                        if !line.trim().is_empty() {
                            logger::log_to_file(&format!("[yt-dlp stderr][{}] {}", task_id_stderr, line));

                            let mut storage = last_error_msg_clone.lock().await;
                            *storage = line;
                        }
                    }
                });

                let _ = tokio::join!(stdout_task, stderr_task);

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

#[tauri::command]
pub fn generate_task_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[tauri::command]
pub async fn get_video_info(
    url: String,
    ytdlp_path: Option<String>,
) -> Result<VideoInfo, String> {
    let ytdlp_path = std::path::PathBuf::from(
        ytdlp_path.unwrap_or_else(|| "./win/yt-dlp.exe".to_string()),
    );

    if !ytdlp_path.exists() {
        return Err(format!("yt-dlp not found: {}", ytdlp_path.display()));
    }

    let output = TokioCommand::new(&ytdlp_path)
        .env("PYTHONIOENCODING", "utf-8")
        .env("PYTHONUTF8", "1")
        .env("PYTHONLEGACYWINDOWSSTDIO", "1")
        .env("LC_ALL", "C.UTF-8")
        .arg("--no-warnings")
        .arg("--no-color")
        .arg("--encoding")
        .arg("utf-8")
        .arg("-j")
        .arg("--skip-download")
        .arg(&url)
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let raw: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse yt-dlp output: {}", e))?;

    let title = raw["title"].as_str().unwrap_or("").to_string();
    let thumbnail = raw["thumbnail"].as_str().unwrap_or("").to_string();

    let thumbnails: Vec<ThumbnailInfo> = raw["thumbnails"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|t| {
                    Some(ThumbnailInfo {
                        url: t["url"].as_str()?.to_string(),
                        width: t["width"].as_u64(),
                        height: t["height"].as_u64(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(VideoInfo {
        title,
        thumbnail,
        thumbnails,
    })
}
