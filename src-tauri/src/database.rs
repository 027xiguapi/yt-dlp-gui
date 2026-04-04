use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_migrations() -> Vec<Migration> {
    vec![
        // 配置表
        Migration {
            version: 1,
            description: "Create config table",
            sql: "
                CREATE TABLE IF NOT EXISTS config (
                    id INTEGER PRIMARY KEY,
                    key TEXT UNIQUE NOT NULL,
                    value TEXT NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );
            ",
            kind: MigrationKind::Up,
        },
        // 下载记录表
        Migration {
            version: 2,
            description: "Create download_history table",
            sql: "
                CREATE TABLE IF NOT EXISTS download_history (
                    id TEXT PRIMARY KEY,
                    url TEXT NOT NULL,
                    title TEXT,
                    preset TEXT NOT NULL,
                    path TEXT NOT NULL,
                    status TEXT NOT NULL,
                    progress REAL DEFAULT 0,
                    speed TEXT,
                    eta TEXT,
                    size TEXT,
                    error TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    completed_at DATETIME
                );
            ",
            kind: MigrationKind::Up,
        },
        // 用户登录表
        Migration {
            version: 3,
            description: "Create user table",
            sql: "
                CREATE TABLE IF NOT EXISTS user (
                    id INTEGER PRIMARY KEY,
                    username TEXT UNIQUE NOT NULL,
                    password_hash TEXT NOT NULL,
                    email TEXT UNIQUE,
                    is_active INTEGER DEFAULT 1,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    last_login DATETIME
                );
            ",
            kind: MigrationKind::Up,
        },
        // 软件过期表
        Migration {
            version: 4,
            description: "Create software_license table",
            sql: "
                CREATE TABLE IF NOT EXISTS software_license (
                    id INTEGER PRIMARY KEY,
                    license_key TEXT UNIQUE NOT NULL,
                    user_id INTEGER,
                    start_date DATE NOT NULL,
                    end_date DATE NOT NULL,
                    is_active INTEGER DEFAULT 1,
                    status TEXT DEFAULT 'active',
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (user_id) REFERENCES user(id)
                );
            ",
            kind: MigrationKind::Up,
        },
        // RSS 订阅表
        Migration {
            version: 5,
            description: "Create rss_feeds table",
            sql: "
                CREATE TABLE IF NOT EXISTS rss_feeds (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    channel_id TEXT UNIQUE NOT NULL,
                    title TEXT NOT NULL,
                    url TEXT NOT NULL,
                    description TEXT,
                    thumbnail TEXT,
                    last_checked DATETIME,
                    auto_refresh INTEGER DEFAULT 1,
                    refresh_interval_minutes INTEGER DEFAULT 30,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );
            ",
            kind: MigrationKind::Up,
        },
        // RSS 视频项表
        Migration {
            version: 6,
            description: "Create rss_items table",
            sql: "
                CREATE TABLE IF NOT EXISTS rss_items (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    feed_id INTEGER NOT NULL,
                    video_id TEXT UNIQUE NOT NULL,
                    title TEXT NOT NULL,
                    description TEXT,
                    url TEXT NOT NULL,
                    thumbnail TEXT,
                    published_at DATETIME,
                    downloaded INTEGER DEFAULT 0,
                    download_task_id TEXT,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (feed_id) REFERENCES rss_feeds(id) ON DELETE CASCADE
                );
            ",
            kind: MigrationKind::Up,
        },
    ]
}
