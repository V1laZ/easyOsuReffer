mod banchobot_parser;
mod commands;
mod irc_handler;
mod types;

use base64::Engine;
use commands::*;
use tauri::{Emitter, Manager};
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_sql::{Migration, MigrationKind};

use crate::types::IrcState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "
            CREATE TABLE IF NOT EXISTS user_credentials (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS mappools (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS beatmap_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                mappool_id INTEGER NOT NULL,
                beatmap_id INTEGER NOT NULL,
                artist TEXT NOT NULL,
                title TEXT NOT NULL,
                difficulty TEXT NOT NULL,
                mapper TEXT NOT NULL,
                mod_combination TEXT,
                category TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (mappool_id) REFERENCES mappools (id) ON DELETE CASCADE
            );
        ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "add_oauth_token_table",
            sql: "
            CREATE TABLE IF NOT EXISTS oauth_tokens (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                access_token TEXT NOT NULL,
                refresh_token TEXT NOT NULL,
                expires_in INTEGER NOT NULL,
                expires_at TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
        ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "add_user_id_foreign_key_to_oauth_tokens",
            sql: "
            ALTER TABLE oauth_tokens ADD COLUMN user_id INTEGER REFERENCES user_credentials(id) ON DELETE CASCADE;
        ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 4,
            description: "make_user_id_unique_in_oauth_tokens",
            sql: "
            CREATE UNIQUE INDEX IF NOT EXISTS idx_oauth_tokens_user_id ON oauth_tokens(user_id);
        ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 5,
            description: "recreate_oauth_tokens_with_username",
            sql: "
            DROP TABLE IF EXISTS oauth_tokens;

            CREATE TABLE oauth_tokens (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                irc_username TEXT NOT NULL UNIQUE,
                access_token TEXT NOT NULL,
                refresh_token TEXT NOT NULL,
                expires_in INTEGER NOT NULL,
                expires_at TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
        ",
            kind: MigrationKind::Up,
        },
    ];

    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }

    builder
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:osu_reffer_database.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .manage(IrcState::default())
        .invoke_handler(tauri::generate_handler![
            connect_to_bancho,
            reconnect_to_bancho,
            disconnect_from_bancho,
            send_message_to_room,
            join_channel,
            leave_channel,
            close_private_message,
            get_connection_status,
            get_rooms_list,
            set_active_room,
            start_private_message,
            fetch_beatmap_data,
            set_mappool,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            app.deep_link().on_open_url(move |event| {
                if let Some(url) = event.urls().first() {
                    println!("Received deep link: {}", url);
                    let query = url
                        .query_pairs()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<std::collections::HashMap<_, _>>();

                    if let Some(base64_data) = query.get("data") {
                        if let Ok(decoded_bytes) =
                            base64::engine::general_purpose::STANDARD.decode(base64_data)
                        {
                            if let Ok(decoded_string) = String::from_utf8(decoded_bytes) {
                                if let Ok(token_data) =
                                    serde_json::from_str::<serde_json::Value>(&decoded_string)
                                {
                                    app_handle
                                        .emit("oauth-token-callback", token_data)
                                        .expect("Failed to emit oauth-token-callback event");
                                }
                            }
                        }
                    }
                }
            });

            #[cfg(desktop)]
            {
                app.deep_link().register("osureffer")?;
            }

            #[cfg(any(target_os = "linux", all(debug_assertions, windows)))]
            {
                app.deep_link().register_all()?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
