use crate::logger;
use std::path::Path;
use tokio::process::Command as TokioCommand;

#[tauri::command]
pub async fn check_version(cmd: String, args: Vec<String>, ytdlp_path: Option<String>) -> Result<String, String> {
    let ytdlp_path_str = ytdlp_path.as_deref().unwrap_or_default();
    logger::log_to_file(&format!(
        "check_version called - cmd: {}, args: {:?}, ytdlp_path: {}",
        cmd, args, ytdlp_path_str
    ));

    let mut command = TokioCommand::new(&cmd);
    command.args(&args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    logger::log_to_file(&format!("Executing: {} {}", cmd, args.join(" ")));

    match command.output().await {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let _stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if output.status.success() {
                logger::log_to_file(&format!("Command succeeded: {}", stdout.trim()));
                return Ok(stdout.trim().to_string());
            }
        }
        Err(e) => {
            logger::log_to_file(&format!("Command failed: {}", e));
        }
    }

    let exe_dir = Path::new(ytdlp_path_str)
        .parent()
        .ok_or_else(|| format!("Invalid ytdlp_path: {}", ytdlp_path_str))?;

    let exe_dir_str = exe_dir.to_string_lossy().to_string();
    logger::log_to_file(&format!("exe_dir: {}", exe_dir_str));

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

    logger::log_to_file(&format!("fallback_paths: {:?}", fallback_paths));

    for path in fallback_paths {
        let mut fallback_cmd = TokioCommand::new(&path);
        fallback_cmd.args(&args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        logger::log_to_file(&format!("Trying fallback: {} {}", path, args.join(" ")));

        if let Ok(output) = fallback_cmd.output().await {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if output.status.success() {
                logger::log_to_file(&format!("Fallback succeeded: {}", stdout.trim()));
                return Ok(stdout.trim().to_string());
            } else if !stderr.is_empty() {
                logger::log_to_file(&format!("Fallback stderr: {}", stderr.trim()));
                return Ok(stderr.trim().to_string());
            }
        }
    }

    logger::log_to_file(&format!("Failed to execute {} with args {:?}", cmd, args));
    Err(format!("Failed to execute {}", cmd))
}

#[tauri::command]
pub fn open_download_folder(path: String) -> Result<(), String> {
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

    logger::log_to_file(&format!("Opening folder: {}", path));
    Ok(())
}
