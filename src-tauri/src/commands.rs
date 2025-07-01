use crate::irc_handler::handle_irc_connection;
use crate::types::*;
use anyhow::Result;
use irc::client::prelude::*;
use std::sync::Arc;
use tauri::State;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn connect_to_bancho(
    config: ConnectionConfig,
    state: State<'_, IrcState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    println!("Attempting to connect to osu! Bancho...");

    // Check if already connected
    {
        let irc_state = state.lock().unwrap();
        if irc_state.connected {
            return Err("Already connected to IRC".to_string());
        }
    }

    let irc_config = Config {
        nickname: Some(config.username.clone()),
        server: Some("irc.ppy.sh".to_string()),
        port: Some(6667),
        username: Some(config.username.clone()),
        password: Some(config.password.clone()),
        use_tls: Some(false),
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

            // Create a command channel for sending messages
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<IrcCommand>();

            // Update the state
            {
                let mut irc_state = state.lock().unwrap();
                irc_state.connected = true;
                irc_state.config = Some(config.clone());
                irc_state.current_username = Some(config.username.clone());
                irc_state.message_sender = Some(tx);
            }

            // Clone what we need for the async task
            let state_clone = Arc::clone(&state.inner());
            let app_handle_clone = app_handle.clone();

            // Spawn the message handling task
            tokio::spawn(async move {
                handle_irc_connection(client, app_handle_clone, state_clone, rx).await;
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
pub async fn send_message_to_channel(
    channel: String,
    message: String,
    state: State<'_, IrcState>,
) -> Result<String, String> {
    let sender = {
        let irc_state = state.lock().unwrap();
        if !irc_state.connected {
            return Err("Not connected to IRC".to_string());
        }
        irc_state.message_sender.clone()
    };

    if let Some(sender) = sender {
        if message == "!mp settings" {
            clear_all_players(&channel, &state);
        }
        let command = IrcCommand::SendMessage { channel, message };
        if let Err(_) = sender.send(command) {
            return Err("Failed to queue message for sending".to_string());
        }
        Ok("Message queued for sending".to_string())
    } else {
        Err("Message sender not available".to_string())
    }
}

#[tauri::command]
pub async fn join_channel(channel: String, state: State<'_, IrcState>) -> Result<String, String> {
    let sender = {
        let irc_state = state.lock().unwrap();
        if !irc_state.connected {
            return Err("Not connected to IRC".to_string());
        }

        if irc_state.channels.contains(&channel) {
            return Err("Already in this channel".to_string());
        }

        irc_state.message_sender.clone()
    };

    if let Some(sender) = sender {
        let command = IrcCommand::JoinChannel {
            channel: channel.clone(),
        };
        if let Err(_) = sender.send(command) {
            return Err("Failed to queue join command".to_string());
        }

        // Update the channels list optimistically
        {
            let mut irc_state = state.lock().unwrap();
            if !irc_state.channels.contains(&channel) {
                irc_state.channels.push(channel.clone());
            }

            // If it's a lobby channel, create lobby state
            if channel.starts_with("#mp_") {
                if !irc_state.lobby_states.contains_key(&channel) {
                    irc_state
                        .lobby_states
                        .insert(channel.clone(), LobbyState::new(channel.clone()));
                }
            }
        }

        Ok(format!("Joining channel: {}", channel))
    } else {
        Err("Message sender not available".to_string())
    }
}

#[tauri::command]
pub async fn leave_channel(channel: String, state: State<'_, IrcState>) -> Result<String, String> {
    let sender = {
        let irc_state = state.lock().unwrap();
        if !irc_state.connected {
            return Err("Not connected to IRC".to_string());
        }
        irc_state.message_sender.clone()
    };

    if let Some(sender) = sender {
        let command = IrcCommand::LeaveChannel {
            channel: channel.clone(),
        };
        if let Err(_) = sender.send(command) {
            return Err("Failed to queue leave command".to_string());
        }

        remove_channel(&channel, &state);

        Ok(format!("Left channel: {}", channel))
    } else {
        Err("Message sender not available".to_string())
    }
}

#[tauri::command]
pub async fn disconnect_from_bancho(state: State<'_, IrcState>) -> Result<String, String> {
    let sender = {
        let irc_state = state.lock().unwrap();
        if !irc_state.connected {
            return Err("Not connected".to_string());
        }
        irc_state.message_sender.clone()
    };

    if let Some(sender) = sender {
        let command = IrcCommand::Disconnect;
        if let Err(_) = sender.send(command) {
            // If sending fails, force disconnect
            let mut irc_state = state.lock().unwrap();
            irc_state.connected = false;
            irc_state.channels.clear();
            irc_state.config = None;
            irc_state.client = None;
            irc_state.message_sender = None;
            irc_state.current_username = None;
            irc_state.lobby_states.clear();
        }
    }

    Ok("Disconnected from osu! Bancho".to_string())
}

#[tauri::command]
pub async fn get_connection_status(state: State<'_, IrcState>) -> Result<bool, String> {
    let irc_state = state.lock().unwrap();
    Ok(irc_state.connected)
}

#[tauri::command]
pub async fn get_joined_channels(state: State<'_, IrcState>) -> Result<Vec<String>, String> {
    let irc_state = state.lock().unwrap();
    Ok(irc_state.channels.clone())
}

// Lobby management commands
#[tauri::command]
pub async fn get_lobby_state(
    channel: String,
    state: State<'_, IrcState>,
) -> Result<Option<LobbyState>, String> {
    let irc_state = state.lock().unwrap();
    Ok(irc_state.lobby_states.get(&channel).cloned())
}

pub fn remove_channel(channel: &str, state: &IrcState) {
    let mut irc_state = state.lock().unwrap();
    irc_state.channels.retain(|c: &String| c != channel);

    // If it's a lobby channel, remove lobby state
    if channel.starts_with("#mp_") {
        irc_state.lobby_states.remove(channel);
    }
}

pub fn clear_all_players(channel: &str, state: &IrcState) {
    let mut irc_state = state.lock().unwrap();
    if let Some(lobby) = irc_state.lobby_states.get_mut(channel) {
        for slot in &mut lobby.slots {
            slot.player = None;
        }
    }
}
