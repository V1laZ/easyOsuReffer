use anyhow::Result;
use futures::stream::StreamExt;
use irc::client::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{Emitter, State};

// IRC client state
#[derive(Debug)]
pub struct IrcClientState {
    pub connected: bool,
    pub channels: Vec<String>,
    pub config: Option<ConnectionConfig>,
}

impl Default for IrcClientState {
    fn default() -> Self {
        Self {
            connected: false,
            channels: Vec::new(),
            config: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IrcMessage {
    pub channel: String,
    pub username: String,
    pub message: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionConfig {
    pub username: String,
    pub password: String,
}

// Global state for the IRC client
pub type IrcState = Arc<Mutex<IrcClientState>>;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn connect_to_bancho(
    config: ConnectionConfig,
    state: State<'_, IrcState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    println!("Attempting to connect to osu! Bancho...");

    // Update the state with connection config
    {
        let mut irc_state = state.lock().unwrap();
        irc_state.config = Some(config.clone());
    }

    let irc_config = Config {
        nickname: Some(config.username.clone()),
        server: Some("irc.ppy.sh".to_string()),
        port: Some(6667),
        username: Some(config.username.clone()),
        password: Some(config.password.clone()),
        use_tls: Some(false),
        channels: vec!["#osu".to_string()],
        ..Config::default()
    };

    match irc::client::Client::from_config(irc_config).await {
        Ok(client) => {
            println!("IRC client created successfully");

            // Start the connection
            if let Err(e) = client.identify() {
                return Err(format!("Failed to identify: {}", e));
            }

            println!("Connected to osu! Bancho!");

            // Update the state
            {
                let mut irc_state = state.lock().unwrap();
                irc_state.connected = true;
                irc_state.channels = vec!["#osu".to_string()];
            }

            // Clone what we need for the async task
            let state_clone = Arc::clone(&state.inner());
            let app_handle_clone = app_handle.clone();

            // Spawn the message handling task
            tokio::spawn(async move {
                handle_irc_messages(client, app_handle_clone, state_clone).await;
            });

            Ok("Successfully connected to osu! Bancho".to_string())
        }
        Err(e) => {
            println!("Failed to create IRC client: {}", e);
            Err(format!("Failed to connect: {}", e))
        }
    }
}

#[tauri::command]
async fn send_message_to_channel(
    channel: String,
    message: String,
    state: State<'_, IrcState>,
) -> Result<String, String> {
    let config = {
        let irc_state = state.lock().unwrap();
        if !irc_state.connected {
            return Err("Not connected to IRC".to_string());
        }
        irc_state.config.clone()
    };

    if let Some(config) = config {
        // Create a new client for sending the message
        let irc_config = Config {
            nickname: Some(config.username.clone()),
            server: Some("irc.ppy.sh".to_string()),
            port: Some(6667),
            username: Some(config.username.clone()),
            password: Some(config.password.clone()),
            use_tls: Some(false),
            channels: vec![channel.clone()],
            ..Config::default()
        };

        match irc::client::Client::from_config(irc_config).await {
            Ok(client) => {
                if let Err(e) = client.identify() {
                    return Err(format!("Failed to identify: {}", e));
                }

                // Send the message
                if let Err(e) = client.send_privmsg(&channel, &message) {
                    return Err(format!("Failed to send message: {}", e));
                }

                Ok("Message sent".to_string())
            }
            Err(e) => Err(format!("Failed to create client for sending: {}", e)),
        }
    } else {
        Err("No connection configuration available".to_string())
    }
}

#[tauri::command]
async fn join_channel(channel: String, state: State<'_, IrcState>) -> Result<String, String> {
    let config = {
        let irc_state = state.lock().unwrap();
        if !irc_state.connected {
            return Err("Not connected to IRC".to_string());
        }
        irc_state.config.clone()
    };

    if let Some(config) = config {
        // Create a new client for joining the channel
        let irc_config = Config {
            nickname: Some(config.username.clone()),
            server: Some("irc.ppy.sh".to_string()),
            port: Some(6667),
            username: Some(config.username.clone()),
            password: Some(config.password.clone()),
            use_tls: Some(false),
            channels: vec![channel.clone()],
            ..Config::default()
        };

        match irc::client::Client::from_config(irc_config).await {
            Ok(client) => {
                if let Err(e) = client.identify() {
                    return Err(format!("Failed to identify: {}", e));
                }

                // Join the channel
                if let Err(e) = client.send_join(&channel) {
                    return Err(format!("Failed to join channel: {}", e));
                }

                // Update the channels list
                {
                    let mut irc_state = state.lock().unwrap();
                    if !irc_state.channels.contains(&channel) {
                        irc_state.channels.push(channel.clone());
                    }
                }

                Ok(format!("Joined channel: {}", channel))
            }
            Err(e) => Err(format!("Failed to create client for joining: {}", e)),
        }
    } else {
        Err("No connection configuration available".to_string())
    }
}

#[tauri::command]
async fn disconnect_from_bancho(state: State<'_, IrcState>) -> Result<String, String> {
    let mut irc_state = state.lock().unwrap();

    if irc_state.connected {
        irc_state.connected = false;
        irc_state.channels.clear();
        irc_state.config = None;

        Ok("Disconnected from osu! Bancho".to_string())
    } else {
        Err("Not connected".to_string())
    }
}

#[tauri::command]
async fn get_connection_status(state: State<'_, IrcState>) -> Result<bool, String> {
    let irc_state = state.lock().unwrap();
    Ok(irc_state.connected)
}

#[tauri::command]
async fn get_joined_channels(state: State<'_, IrcState>) -> Result<Vec<String>, String> {
    let irc_state = state.lock().unwrap();
    Ok(irc_state.channels.clone())
}

async fn handle_irc_messages(
    mut client: irc::client::Client,
    app_handle: tauri::AppHandle,
    state: IrcState,
) {
    println!("Starting IRC message handler...");

    let mut stream = client.stream().unwrap();

    while let Some(message) = stream.next().await {
        // Check if we should still be connected
        {
            let irc_state = state.lock().unwrap();
            if !irc_state.connected {
                println!("Connection marked as disconnected, stopping message handler");
                break;
            }
        }

        match message {
            Ok(msg) => {
                println!("Received IRC message: {:?}", msg);

                match msg.command {
                    Command::PRIVMSG(channel, text) => {
                        if let Some(prefix) = msg.prefix {
                            // Extract nickname from prefix
                            let nick = match prefix {
                                irc::proto::Prefix::Nickname(nick, _, _) => nick,
                                irc::proto::Prefix::ServerName(server) => server,
                            };

                            let irc_message = IrcMessage {
                                channel: channel.clone(),
                                username: nick.clone(),
                                message: text.clone(),
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            };

                            println!("[{}] <{}> {}", channel, nick, text);

                            // Emit the message to the frontend
                            if let Err(e) = app_handle.emit("irc-message", &irc_message) {
                                println!("Failed to emit message to frontend: {}", e);
                            }
                        }
                    }
                    Command::JOIN(channel, _, _) => {
                        if let Some(prefix) = msg.prefix {
                            let nick = match prefix {
                                irc::proto::Prefix::Nickname(nick, _, _) => nick,
                                irc::proto::Prefix::ServerName(server) => server,
                            };

                            println!("{} joined {}", nick, channel);

                            // Emit join event to frontend
                            if let Err(e) = app_handle.emit(
                                "user-joined",
                                serde_json::json!({
                                    "channel": channel,
                                    "username": nick
                                }),
                            ) {
                                println!("Failed to emit join event: {}", e);
                            }
                        }
                    }
                    Command::PART(channel, _) => {
                        if let Some(prefix) = msg.prefix {
                            let nick = match prefix {
                                irc::proto::Prefix::Nickname(nick, _, _) => nick,
                                irc::proto::Prefix::ServerName(server) => server,
                            };

                            println!("{} left {}", nick, channel);

                            // Emit part event to frontend
                            if let Err(e) = app_handle.emit(
                                "user-left",
                                serde_json::json!({
                                    "channel": channel,
                                    "username": nick
                                }),
                            ) {
                                println!("Failed to emit part event: {}", e);
                            }
                        }
                    }
                    Command::Response(response, args) => {
                        println!("Server response: {:?} - {:?}", response, args);

                        // Handle specific server responses
                        match response {
                            Response::RPL_WELCOME => {
                                println!("Successfully connected and welcomed to the server!");
                            }
                            Response::RPL_NAMREPLY => {
                                // Channel user list
                                if args.len() >= 4 {
                                    let channel = &args[2];
                                    let users = &args[3];
                                    println!("Users in {}: {}", channel, users);
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        // Handle other commands if needed
                        println!("Other IRC command: {:?}", msg.command);
                    }
                }
            }
            Err(e) => {
                println!("Error receiving IRC message: {}", e);
                break;
            }
        }
    }

    println!("IRC message handler ended");

    // Mark as disconnected
    {
        let mut irc_state = state.lock().unwrap();
        irc_state.connected = false;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(IrcState::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            connect_to_bancho,
            send_message_to_channel,
            join_channel,
            disconnect_from_bancho,
            get_connection_status,
            get_joined_channels
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
