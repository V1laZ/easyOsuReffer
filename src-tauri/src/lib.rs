mod banchobot_parser;
mod commands;
mod irc_handler;
mod migrations;
mod osu_api;
mod types;

use base64::Engine;
use commands::*;
use tauri::{Emitter, Manager};
use tauri_plugin_deep_link::DeepLinkExt;

use crate::migrations::get_migrations;
use crate::types::IrcState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = get_migrations();

    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init());
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
            fetch_user_data,
            set_mappool,
            get_room_state,
            check_for_updates,
            install_update,
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
