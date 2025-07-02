use crate::irc_handler::handle_irc_connection;
use crate::types::*;
use anyhow::Result;
use irc::client::prelude::*;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn connect_to_bancho(
    config: ConnectionConfig,
    state: State<'_, IrcState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    println!("Attempting to connect to osu! Bancho...");

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

            if let Err(e) = client.identify() {
                return Err(format!("Failed to identify: {}", e));
            }

            println!("Connected to osu! Bancho!");

            let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<IrcCommand>();

            {
                let mut irc_state = state.lock().unwrap();
                irc_state.connected = true;
                irc_state.config = Some(config.clone());
                irc_state.current_username = Some(config.username.clone());
                irc_state.message_sender = Some(tx);
            }

            let state_clone = Arc::clone(&state.inner());
            let app_handle_clone = app_handle.clone();

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
pub async fn send_message_to_room(
    room_id: String,
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
        let room = {
            let irc_state = state.lock().unwrap();
            irc_state.rooms.get(&room_id).cloned()
        };

        if let Some(room) = room {
            let command = match room.room_type {
                RoomType::Channel | RoomType::MultiplayerLobby => {
                    if message == "!mp settings" {
                        clear_all_players(&room_id, &state);
                    }
                    IrcCommand::SendMessage { room_id, message }
                }
                RoomType::PrivateMessage => IrcCommand::SendPrivateMessage {
                    username: room_id,
                    message,
                },
            };

            if let Err(_) = sender.send(command) {
                return Err("Failed to queue message for sending".to_string());
            }
            Ok("Message queued for sending".to_string())
        } else {
            Err("Room not found".to_string())
        }
    } else {
        Err("Message sender not available".to_string())
    }
}

#[tauri::command]
pub async fn join_channel(room_id: String, state: State<'_, IrcState>) -> Result<String, String> {
    let sender = {
        let irc_state = state.lock().unwrap();
        if !irc_state.connected {
            return Err("Not connected to IRC".to_string());
        }

        if irc_state.rooms.contains_key(&room_id) {
            return Err("Already in this room".to_string());
        }

        irc_state.message_sender.clone()
    };

    if let Some(sender) = sender {
        let command = IrcCommand::JoinChannel {
            channel: room_id.clone(),
        };
        if let Err(_) = sender.send(command) {
            return Err("Failed to queue join command".to_string());
        }

        // Update the rooms list optimistically
        {
            let mut irc_state = state.lock().unwrap();
            let room = Room::new_channel(room_id.clone());
            irc_state.rooms.insert(room_id.clone(), room);

            // If it's a lobby room, create lobby state
            if room_id.starts_with("#mp_") {
                if !irc_state.lobby_states.contains_key(&room_id) {
                    irc_state
                        .lobby_states
                        .insert(room_id.clone(), LobbyState::new(room_id.clone()));
                }
            }
        }

        Ok(format!("Joining channel: {}", room_id))
    } else {
        Err("Message sender not available".to_string())
    }
}

#[tauri::command]
pub async fn leave_channel(room_id: String, state: State<'_, IrcState>) -> Result<String, String> {
    let sender = {
        let irc_state = state.lock().unwrap();
        if !irc_state.connected {
            return Err("Not connected to IRC".to_string());
        }
        irc_state.message_sender.clone()
    };

    if let Some(sender) = sender {
        let command = IrcCommand::LeaveChannel {
            channel: room_id.clone(),
        };
        if let Err(_) = sender.send(command) {
            return Err("Failed to queue leave command".to_string());
        }

        remove_room(&room_id, &state);

        Ok(format!("Left channel: {}", room_id))
    } else {
        Err("Message sender not available".to_string())
    }
}

#[tauri::command]
pub async fn close_private_message(
    username: String,
    state: State<'_, IrcState>,
) -> Result<String, String> {
    remove_room(&username, &state);

    Ok(format!("Closed private message with {}", username))
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
            irc_state.rooms.clear();
            irc_state.active_room = None;
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
pub async fn get_joined_rooms(state: State<'_, IrcState>) -> Result<Vec<Room>, String> {
    let irc_state = state.lock().unwrap();
    Ok(irc_state.rooms.values().cloned().collect())
}

#[tauri::command]
pub async fn get_room_messages(
    room_id: String,
    state: State<'_, IrcState>,
) -> Result<Vec<IrcMessage>, String> {
    let irc_state = state.lock().unwrap();
    if let Some(room) = irc_state.rooms.get(&room_id) {
        Ok(room.messages.clone())
    } else {
        Ok(Vec::new())
    }
}

#[tauri::command]
pub async fn set_active_room(room_id: String, state: State<'_, IrcState>) -> Result<(), String> {
    let mut irc_state = state.lock().unwrap();
    let prev_room_id = irc_state.active_room.clone();

    if let Some(prev_id) = prev_room_id {
        if let Some(prev_room) = irc_state.rooms.get_mut(&prev_id) {
            prev_room.is_active = false;
        }
    }

    if let Some(room) = irc_state.rooms.get_mut(&room_id) {
        room.is_active = true;
        room.mark_as_read();
        irc_state.active_room = Some(room_id);
        Ok(())
    } else {
        Err("Room not found".to_string())
    }
}

#[tauri::command]
pub async fn start_private_message(
    username: String,
    state: State<'_, IrcState>,
) -> Result<String, String> {
    let mut irc_state = state.lock().unwrap();

    if irc_state.rooms.contains_key(&username) {
        return Ok(format!(
            "Private message room with {} already exists",
            username
        ));
    }

    let room = Room::new_private_message(username.clone());
    irc_state.rooms.insert(username.clone(), room);

    Ok(format!("Started private message with {}", username))
}

#[tauri::command]
pub async fn get_lobby_state(
    room_id: String,
    state: State<'_, IrcState>,
) -> Result<Option<LobbyState>, String> {
    let irc_state = state.lock().unwrap();
    Ok(irc_state.lobby_states.get(&room_id).cloned())
}

pub fn remove_room(room_id: &str, state: &IrcState) {
    let mut irc_state = state.lock().unwrap();
    irc_state.rooms.remove(room_id);

    if room_id.starts_with("#mp_") {
        irc_state.lobby_states.remove(room_id);
    }

    if irc_state.active_room.as_ref() == Some(&room_id.to_string()) {
        irc_state.active_room = None;
    }
}

pub fn clear_all_players(room_id: &str, state: &IrcState) {
    let mut irc_state = state.lock().unwrap();
    if let Some(lobby) = irc_state.lobby_states.get_mut(room_id) {
        for slot in &mut lobby.slots {
            slot.player = None;
        }
    }
}
