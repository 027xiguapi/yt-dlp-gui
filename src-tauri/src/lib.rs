mod models;
mod config;
#[macro_use]
mod download;
mod extraction;
mod utils;
mod app;
mod logger;
mod database;
mod rss;

pub use models::*;
pub use config::*;
pub use download::*;
pub use extraction::*;
pub use utils::*;
pub use rss::*;

use tauri::{Emitter, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logger::init_logger();

    logger::log_to_file("Starting Tauri application");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_sql::Builder::new().add_migrations("sqlite:video-dlp.db", database::get_migrations()).build())
        .setup(|app| {
            app::setup_app(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            let _ = app.emit("single-instance", args.clone());

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
            get_video_info,
            parse_rss_feed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
