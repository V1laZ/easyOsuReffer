use crate::irc_handler::handle_irc_connection;
use crate::types::*;
use anyhow::Result;
use irc::client::prelude::*;
use std::sync::Arc;
use tauri::{Emitter, State};

fn emit_rooms_list_updated(app_handle: &tauri::AppHandle, state: &IrcState) {
    let rooms_response = {
        let irc_state = state.lock().unwrap();
        RoomsListResponse {
            rooms: irc_state.rooms.values().map(RoomListItem::from).collect(),
            active_room_id: irc_state.active_room_id.clone(),
        }
    };

    let _ = app_handle.emit("rooms-list-updated", rooms_response);
}


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

            // Rejoin all previously joined channels and multiplayer lobbies
            {
                let irc_state = state.lock().unwrap();
                let message_sender = irc_state.message_sender.clone();
                let rooms_to_rejoin: Vec<String> = irc_state
                    .rooms
                    .iter()
                    .filter_map(|(room_id, room)| match room.room_type {
                        RoomType::Channel | RoomType::MultiplayerLobby => Some(room_id.clone()),
                        _ => None,
                    })
                    .collect();
                if let Some(sender) = message_sender {
                    for room_id in rooms_to_rejoin {
                        let _ = sender.send(IrcCommand::JoinChannel { channel: room_id });
                    }
                }
            }
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
pub async fn join_channel(room_id: String, state: State<'_, IrcState>, app_handle: tauri::AppHandle) -> Result<String, String> {
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

            // Set all existing rooms to inactive
            for room in irc_state.rooms.values_mut() {
                room.is_active = false;
            }

            // Add new room as active
            let room = Room::new_channel(room_id.clone(), true);
            irc_state.rooms.insert(room_id.clone(), room);
            irc_state.active_room_id = Some(room_id.clone());
        }

        emit_rooms_list_updated(&app_handle, &state);

        Ok(format!("Joining channel: {}", room_id))
    } else {
        Err("Message sender not available".to_string())
    }
}

#[tauri::command]
pub async fn leave_channel(
    room_id: String,
    state: State<'_, IrcState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
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

        remove_room(&room_id, &state, &app_handle);

        emit_rooms_list_updated(&app_handle, &state);

        Ok(format!("Left channel: {}", room_id))
    } else {
        Err("Message sender not available".to_string())
    }
}

#[tauri::command]
pub async fn close_private_message(
    username: String,
    state: State<'_, IrcState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    remove_room(&username, &state, &app_handle);

    emit_rooms_list_updated(&app_handle, &state);

    Ok(format!("Closed private message with {}", username))
}

#[tauri::command]
pub async fn reconnect_to_bancho(
    state: State<'_, IrcState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let config = {
        let irc_state = state.lock().unwrap();
        if irc_state.connected {
            return Ok("Already connected".to_string());
        }
        irc_state.config.clone()
    };

    if let Some(config) = config {
        connect_to_bancho(config, state, app_handle).await
    } else {
        Err("No previous config found".to_string())
    }
}

#[tauri::command]
pub async fn disconnect_from_bancho(
    state: State<'_, IrcState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
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
            irc_state.active_room_id = None;
            irc_state.config = None;
            irc_state.client = None;
            irc_state.message_sender = None;
            irc_state.current_username = None;

            let _ = app_handle.emit("active-room-changed", serde_json::json!({
                "room": Option::<Room>::None
            }));
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
pub async fn get_rooms_list(state: State<'_, IrcState>) -> Result<RoomsListResponse, String> {
    let irc_state = state.lock().unwrap();
    Ok(RoomsListResponse {
        rooms: irc_state.rooms.values().map(RoomListItem::from).collect(),
        active_room_id: irc_state.active_room_id.clone(),
    })
}

#[tauri::command]
pub async fn set_active_room(
    room_id: String,
    state: State<'_, IrcState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    {
        let mut irc_state = state.lock().unwrap();

        for room in irc_state.rooms.values_mut() {
            room.is_active = false;
        }

        if irc_state.rooms.contains_key(&room_id) {
            irc_state.active_room_id = Some(room_id.clone());

            if let Some(room) = irc_state.rooms.get_mut(&room_id) {
                room.is_active = true;
                room.mark_as_read();

                let _ = app_handle.emit("active-room-changed", serde_json::json!({
                    "room": Some(room.clone())
                }));
            }
        } else {
            return Err("Room not found".to_string());
        }
    }

    // Emit rooms list update
    emit_rooms_list_updated(&app_handle, &state);

    Ok(())
}

#[tauri::command]
pub async fn start_private_message(
    username: String,
    state: State<'_, IrcState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    {
        let mut irc_state = state.lock().unwrap();

        if irc_state.rooms.contains_key(&username) {
            return Ok(format!(
                "Private message room with {} already exists",
                username
            ));
        }

        // Set all existing rooms to inactive
        for room in irc_state.rooms.values_mut() {
            room.is_active = false;
        }

        let room = Room::new_private_message(username.clone());
        irc_state.rooms.insert(username.clone(), room);
        irc_state.active_room_id = Some(username.clone());
    }

    emit_rooms_list_updated(&app_handle, &state);

    Ok(format!("Started private message with {}", username))
}

#[tauri::command]
pub async fn set_mappool(
    room_id: String,
    mappool_id: Option<u64>,
    state: State<'_, IrcState>,
) -> Result<Option<u64>, String> {
    let mut irc_state = state.lock().unwrap();
    if let Some(room) = irc_state.rooms.get_mut(&room_id) {
        if let Some(lobby_state) = &mut room.lobby_state {
            lobby_state.current_mappool_id = mappool_id;
            return Ok(mappool_id);
        }
    }
    Err("Lobby state not found".to_string())
}

#[tauri::command]
pub async fn fetch_beatmap_data(
    beatmap_id: String,
    access_token: String,
) -> Result<BeatmapData, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!(
            "https://osu.ppy.sh/api/v2/beatmaps/{}",
            beatmap_id
        ))
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch beatmap data: {}", e))?;

    if !response.status().is_success() {
        if response.status().as_u16() == 404 {
            return Err("Beatmap not found".to_string());
        }
        return Err(format!(
            "Failed to fetch beatmap data: {}",
            response.status()
        ));
    }

    let api_response: OsuApiBeatmapResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse API response: {}", e))?;

    Ok(BeatmapData {
        id: api_response.id,
        beatmapset_id: api_response.beatmapset_id,
        artist: api_response.beatmapset.artist,
        title: api_response.beatmapset.title,
        difficulty: api_response.version,
        mapper: api_response.beatmapset.creator,
        mode: api_response.mode_int,
        total_length: api_response.total_length,
        bpm: api_response.bpm,
        difficulty_rating: api_response.difficulty_rating,
    })
}

pub fn remove_room(room_id: &str, state: &IrcState, app_handle: &tauri::AppHandle) {
    let mut irc_state = state.lock().unwrap();
    irc_state.rooms.remove(room_id);

    // Clear active_room_id if the removed room was active
    let was_active = irc_state.active_room_id.as_deref() == Some(room_id);
    if was_active {
        irc_state.active_room_id = None;

        let _ = app_handle.emit("active-room-changed", serde_json::json!({
            "room": Option::<Room>::None
        }));
    }
}

pub fn clear_all_players(room_id: &str, state: &IrcState) {
    let mut irc_state = state.lock().unwrap();
    if let Some(room) = irc_state.rooms.get_mut(room_id) {
        if let Some(lobby) = &mut room.lobby_state {
            for slot in &mut lobby.slots {
                slot.player = None;
            }
        }
    }
}
