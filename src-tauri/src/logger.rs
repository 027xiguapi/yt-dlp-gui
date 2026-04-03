use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

static LOG_FILE: once_cell::sync::Lazy<Mutex<Option<std::fs::File>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(None));

pub fn init_logger() {
    let log_dir = get_log_dir();
    if !log_dir.exists() {
        let _ = std::fs::create_dir_all(&log_dir);
    }

    let _ = env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
            writeln!(buf, "[{}] [{}] {}", timestamp, record.level(), record.args())
        })
        .try_init();

    setup_file_logger();
}

fn get_log_dir() -> PathBuf {
    if let Ok(app_data) = std::env::var("APPDATA") {
        PathBuf::from(app_data).join("video-dlp-gui").join("logs")
    } else {
        PathBuf::from("./logs")
    }
}

fn get_log_file_path() -> PathBuf {
    let date = Local::now().format("%Y-%m-%d").to_string();
    get_log_dir().join(format!("{}.log", date))
}

fn setup_file_logger() {
    let file_path = get_log_file_path();
    match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
    {
        Ok(file) => {
            let mut log_file = LOG_FILE.lock().unwrap();
            *log_file = Some(file);
        }
        Err(e) => {
            eprintln!("Failed to open log file: {}", e);
        }
    }
}

pub fn log_to_file(message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_message = format!("[{}] {}\n", timestamp, message);

    if let Ok(mut log_file_guard) = LOG_FILE.lock() {
        if let Some(ref mut file) = *log_file_guard {
            let _ = file.write_all(log_message.as_bytes());
            let _ = file.flush();
        }
    }
}
