use anyhow::Result;
use futures::stream::StreamExt;
use irc::client::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{Emitter, State};
use tauri_plugin_sql::{Migration, MigrationKind};

// Lobby state structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub username: String,
    pub team: Option<String>, // "red" or "blue"
    #[serde(rename = "isReady")]
    pub is_ready: bool,
    #[serde(rename = "isPlaying")]
    pub is_playing: bool,
    #[serde(rename = "isHost")]
    pub is_host: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerSlot {
    pub id: u8,
    pub player: Option<Player>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentMap {
    #[serde(rename = "beatmapId")]
    pub beatmap_id: u64,
    pub title: String,
    pub difficulty: String,
    pub artist: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LobbySettings {
    #[serde(rename = "roomName")]
    pub room_name: String,
    #[serde(rename = "teamMode")]
    pub team_mode: String, // "HeadToHead", "TagCoop", "TeamVs", "TagTeamVs"
    #[serde(rename = "winCondition")]
    pub win_condition: String, // "Score", "Accuracy", "Combo", "ScoreV2"
    pub size: u8,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LobbyState {
    pub channel: String,
    pub settings: Option<LobbySettings>,
    #[serde(rename = "currentMap")]
    pub current_map: Option<CurrentMap>,
    pub slots: Vec<PlayerSlot>,
    #[serde(rename = "matchStatus")]
    pub match_status: String, // "idle", "ready", "starting", "active"
    pub host: Option<String>,
}

impl LobbyState {
    pub fn new(channel: String) -> Self {
        let slots = (1..=16).map(|id| PlayerSlot { id, player: None }).collect();

        Self {
            channel,
            settings: None,
            current_map: None,
            slots,
            match_status: "idle".to_string(),
            host: None,
        }
    }
}

// IRC client state
#[derive(Debug)]
pub struct IrcClientState {
    pub connected: bool,
    pub channels: Vec<String>,
    pub config: Option<ConnectionConfig>,
    pub client: Option<Arc<Mutex<irc::client::Client>>>,
    pub message_sender: Option<tokio::sync::mpsc::UnboundedSender<IrcCommand>>,
    pub lobby_states: HashMap<String, LobbyState>,
    pub current_username: Option<String>,
}

#[derive(Debug, Clone)]
pub enum IrcCommand {
    SendMessage { channel: String, message: String },
    JoinChannel { channel: String },
    LeaveChannel { channel: String },
    Disconnect,
}

impl Default for IrcClientState {
    fn default() -> Self {
        Self {
            connected: false,
            channels: Vec::new(),
            config: None,
            client: None,
            message_sender: None,
            lobby_states: HashMap::new(),
            current_username: None,
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
async fn send_message_to_channel(
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
async fn join_channel(channel: String, state: State<'_, IrcState>) -> Result<String, String> {
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
async fn leave_channel(channel: String, state: State<'_, IrcState>) -> Result<String, String> {
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
async fn disconnect_from_bancho(state: State<'_, IrcState>) -> Result<String, String> {
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
async fn get_connection_status(state: State<'_, IrcState>) -> Result<bool, String> {
    let irc_state = state.lock().unwrap();
    Ok(irc_state.connected)
}

#[tauri::command]
async fn get_joined_channels(state: State<'_, IrcState>) -> Result<Vec<String>, String> {
    let irc_state = state.lock().unwrap();
    Ok(irc_state.channels.clone())
}

// Lobby management commands
#[tauri::command]
async fn get_lobby_state(
    channel: String,
    state: State<'_, IrcState>,
) -> Result<Option<LobbyState>, String> {
    let irc_state = state.lock().unwrap();
    Ok(irc_state.lobby_states.get(&channel).cloned())
}

// BanchoBot parsing logic
struct BanchoBotParser;

impl BanchoBotParser {
    fn parse_irc_message(
        message: &IrcMessage,
        state: &IrcState,
        app_handle: &tauri::AppHandle,
    ) -> bool {
        if message.username != "BanchoBot" {
            // Handle user leaving lobby
            if let Ok(regex) = Regex::new(r"^(.+) left (#mp_\d+)$") {
                if let Some(captures) = regex.captures(&message.message) {
                    let username = captures.get(1).unwrap().as_str();
                    let channel = captures.get(2).unwrap().as_str();
                    Self::remove_player_by_username(username, channel, state, app_handle);
                    return true;
                }
            }
            return false;
        }

        Self::parse_banchobot_message(message, state, app_handle)
    }

    fn parse_banchobot_message(
        message: &IrcMessage,
        state: &IrcState,
        app_handle: &tauri::AppHandle,
    ) -> bool {
        if message.username != "BanchoBot" {
            return false;
        }

        let text = &message.message;
        let channel = &message.channel;

        if !channel.starts_with("#mp_") {
            return false;
        }

        // Ensure lobby state exists
        {
            let mut irc_state = state.lock().unwrap();
            if !irc_state.lobby_states.contains_key(channel) {
                irc_state
                    .lobby_states
                    .insert(channel.clone(), LobbyState::new(channel.clone()));
            }
        }

        // Room name pattern
        if let Ok(regex) = Regex::new(r"^Room name: (.+), History: https://osu\.ppy\.sh/mp/(\d+)$")
        {
            if let Some(captures) = regex.captures(text) {
                let room_name = captures.get(1).unwrap().as_str();
                Self::update_lobby_settings(
                    channel,
                    |settings| {
                        settings.room_name = room_name.to_string();
                    },
                    state,
                    app_handle,
                );
                return true;
            }
        }

        // Team mode and win condition
        if let Ok(regex) = Regex::new(r"^Team mode: (.+), Win condition: (.+)$") {
            if let Some(captures) = regex.captures(text) {
                let team_mode_str = captures.get(1).unwrap().as_str();
                let win_condition_str = captures.get(2).unwrap().as_str();

                let team_mode = match team_mode_str {
                    "Head To Head" => "HeadToHead",
                    "Tag Coop" => "TagCoop",
                    "Team Vs" => "TeamVs",
                    "Tag Team Vs" => "TagTeamVs",
                    _ => "HeadToHead",
                };

                let win_condition = match win_condition_str {
                    "Score" => "Score",
                    "Accuracy" => "Accuracy",
                    "Combo" => "Combo",
                    "Score V2" => "ScoreV2",
                    _ => "Score",
                };

                Self::update_lobby_settings(
                    channel,
                    |settings| {
                        settings.team_mode = team_mode.to_string();
                        settings.win_condition = win_condition.to_string();
                    },
                    state,
                    app_handle,
                );
                return true;
            }
        }

        // Slot info
        if let Ok(regex) = Regex::new(r"^Slot (\d+)\s+(.+)$") {
            if let Some(captures) = regex.captures(text) {
                if let Ok(slot_id) = captures.get(1).unwrap().as_str().parse::<u8>() {
                    let slot_info = captures.get(2).unwrap().as_str();
                    Self::parse_slot_info(slot_info, slot_id, channel, state, app_handle);
                    return true;
                }
            }
        }

        if let Ok(regex) = Regex::new(r"^Beatmap: https://osu\.ppy\.sh/b/(\d+) (.+) \[(.+)\]$") {
            if let Some(captures) = regex.captures(text) {
                if let Ok(beatmap_id) = captures.get(1).unwrap().as_str().parse::<u64>() {
                    let full_title = captures.get(2).unwrap().as_str();
                    let difficulty = captures.get(3).unwrap().as_str();

                    if let Ok(title_regex) = Regex::new(r"^(.+) - (.+)$") {
                        if let Some(title_captures) = title_regex.captures(full_title) {
                            let artist = title_captures.get(1).unwrap().as_str();
                            let title = title_captures.get(2).unwrap().as_str();

                            Self::update_current_map(
                                channel,
                                CurrentMap {
                                    beatmap_id,
                                    artist: artist.to_string(),
                                    title: title.to_string(),
                                    difficulty: difficulty.to_string(),
                                },
                                state,
                                app_handle,
                            );
                        }
                    }
                    return true;
                }
            }
        }

        if let Ok(regex) = Regex::new(
            r"^Beatmap changed to: (.+) - (.+) \[(.+)\] \(https://osu\.ppy\.sh/b/(\d+)\)$",
        ) {
            if let Some(captures) = regex.captures(text) {
                let artist = captures.get(1).unwrap().as_str();
                let title = captures.get(2).unwrap().as_str();
                let difficulty = captures.get(3).unwrap().as_str();
                if let Ok(beatmap_id) = captures.get(4).unwrap().as_str().parse::<u64>() {
                    Self::update_current_map(
                        channel,
                        CurrentMap {
                            beatmap_id,
                            artist: artist.to_string(),
                            title: title.to_string(),
                            difficulty: difficulty.to_string(),
                        },
                        state,
                        app_handle,
                    );
                    return true;
                }
            }
        }

        // Player joined
        if let Ok(regex) = Regex::new(r"^(.+) joined in slot (\d+)( for team (red|blue))?\.?$") {
            if let Some(captures) = regex.captures(text) {
                let username = captures.get(1).unwrap().as_str();
                if let Ok(slot_id) = captures.get(2).unwrap().as_str().parse::<u8>() {
                    let team = captures.get(4).map(|m| m.as_str().to_string());

                    Self::add_player(
                        channel,
                        slot_id,
                        Player {
                            username: username.to_string(),
                            team,
                            is_ready: false,
                            is_playing: false,
                            is_host: false,
                        },
                        state,
                        app_handle,
                    );
                    return true;
                }
            }
        }

        // Player left
        if let Ok(regex) = Regex::new(r"^(.+) left the game\.?$") {
            if let Some(captures) = regex.captures(text) {
                let username = captures.get(1).unwrap().as_str();
                Self::remove_player_by_username(username, channel, state, app_handle);
                return true;
            }
        }

        // Match status changes
        if text == "All players are ready" {
            Self::update_match_status(channel, "ready", state, app_handle);
            return true;
        }

        if text == "The match has started!" {
            Self::update_match_status(channel, "active", state, app_handle);
            return true;
        }

        if text == "Cleared match host" {
            Self::clear_host(channel, state, app_handle);
            return true;
        }

        // Host changed
        if let Ok(regex) = Regex::new(r"^Changed match host to (.+)$") {
            if let Some(captures) = regex.captures(text) {
                let new_host = captures.get(1).unwrap().as_str();
                Self::update_host(channel, new_host, state, app_handle);
                return true;
            }
        }

        // Player moved to different slot
        if let Ok(regex) = Regex::new(r"^(.+) moved to slot (\d+)$") {
            if let Some(captures) = regex.captures(text) {
                let username = captures.get(1).unwrap().as_str();
                if let Ok(new_slot_id) = captures.get(2).unwrap().as_str().parse::<u8>() {
                    let team = captures.get(4).map(|m| m.as_str().to_string());
                    Self::move_player_to_slot(
                        channel,
                        username,
                        new_slot_id,
                        team,
                        state,
                        app_handle,
                    );
                    return true;
                }
            }
        }

        // Match aborted
        if text == "The match was aborted" || text.contains("aborted the match") {
            Self::update_match_status(channel, "idle", state, app_handle);
            return true;
        }

        // Match finished
        if text.contains("finished playing") || text.contains("The match has finished!") {
            Self::update_match_status(channel, "idle", state, app_handle);
            return true;
        }

        // Beatmap changed
        if let Ok(regex) =
            Regex::new(r"^Beatmap changed to: https://osu\.ppy\.sh/b/(\d+) (.+) \[(.+)\]$")
        {
            if let Some(captures) = regex.captures(text) {
                if let Ok(beatmap_id) = captures.get(1).unwrap().as_str().parse::<u64>() {
                    let full_title = captures.get(2).unwrap().as_str();
                    let difficulty = captures.get(3).unwrap().as_str();

                    if let Ok(title_regex) = Regex::new(r"^(.+) - (.+)$") {
                        if let Some(title_captures) = title_regex.captures(full_title) {
                            let artist = title_captures.get(1).unwrap().as_str();
                            let title = title_captures.get(2).unwrap().as_str();

                            Self::update_current_map(
                                channel,
                                CurrentMap {
                                    beatmap_id,
                                    artist: artist.to_string(),
                                    title: title.to_string(),
                                    difficulty: difficulty.to_string(),
                                },
                                state,
                                app_handle,
                            );
                            return true;
                        }
                    }
                }
            }
        }

        // Match settings changed
        if let Ok(regex) = Regex::new(r#"^Room name updated to "(.+)"$"#) {
            if let Some(captures) = regex.captures(text) {
                let room_name = captures.get(1).unwrap().as_str();
                Self::update_lobby_settings(
                    channel,
                    |settings| {
                        settings.room_name = room_name.to_string();
                    },
                    state,
                    app_handle,
                );
                return true;
            }
        }

        false
    }

    fn parse_slot_info(
        slot_text: &str,
        slot_id: u8,
        channel: &str,
        state: &IrcState,
        app_handle: &tauri::AppHandle,
    ) {
        let is_ready = !slot_text.contains("Not Ready") && !slot_text.contains("No Map");

        // Extract username
        if let Ok(regex) = Regex::new(r"https?://osu\.ppy\.sh/u/\d+\s+([^\s\[]+)") {
            if let Some(captures) = regex.captures(slot_text) {
                let username = captures.get(1).unwrap().as_str().trim();
                if !username.is_empty() {
                    let is_host = slot_text.contains("[Host");

                    let team = if slot_text.contains("Team Blue") {
                        Some("blue".to_string())
                    } else if slot_text.contains("Team Red") {
                        Some("red".to_string())
                    } else {
                        None
                    };

                    Self::add_player(
                        channel,
                        slot_id,
                        Player {
                            username: username.to_string(),
                            team,
                            is_ready,
                            is_playing: false,
                            is_host,
                        },
                        state,
                        app_handle,
                    );
                }
            }
        }
    }

    fn update_lobby_settings<F>(
        channel: &str,
        updater: F,
        state: &IrcState,
        app_handle: &tauri::AppHandle,
    ) where
        F: FnOnce(&mut LobbySettings),
    {
        let mut irc_state = state.lock().unwrap();
        if let Some(lobby) = irc_state.lobby_states.get_mut(channel) {
            if lobby.settings.is_none() {
                lobby.settings = Some(LobbySettings {
                    room_name: String::new(),
                    team_mode: "HeadToHead".to_string(),
                    win_condition: "Score".to_string(),
                    size: 16,
                    password: None,
                });
            }

            if let Some(ref mut settings) = lobby.settings {
                updater(settings);
            }

            // Emit update to frontend
            let _ = app_handle.emit("lobby-updated", lobby.clone());
        }
    }

    fn update_current_map(
        channel: &str,
        map: CurrentMap,
        state: &IrcState,
        app_handle: &tauri::AppHandle,
    ) {
        let mut irc_state = state.lock().unwrap();
        if let Some(lobby) = irc_state.lobby_states.get_mut(channel) {
            lobby.current_map = Some(map);
            let _ = app_handle.emit("lobby-updated", lobby.clone());
        }
    }

    fn add_player(
        channel: &str,
        slot_id: u8,
        player: Player,
        state: &IrcState,
        app_handle: &tauri::AppHandle,
    ) {
        let mut irc_state = state.lock().unwrap();
        if let Some(lobby) = irc_state.lobby_states.get_mut(channel) {
            if let Some(slot) = lobby.slots.iter_mut().find(|s| s.id == slot_id) {
                slot.player = Some(player);
                let _ = app_handle.emit("lobby-updated", lobby.clone());
            }
        }
    }

    fn remove_player_by_username(
        username: &str,
        channel: &str,
        state: &IrcState,
        app_handle: &tauri::AppHandle,
    ) {
        let mut irc_state = state.lock().unwrap();
        if let Some(lobby) = irc_state.lobby_states.get_mut(channel) {
            for slot in &mut lobby.slots {
                if let Some(ref player) = slot.player {
                    if player.username == username {
                        slot.player = None;
                        break;
                    }
                }
            }
            let _ = app_handle.emit("lobby-updated", lobby.clone());
        }
    }

    fn update_match_status(
        channel: &str,
        status: &str,
        state: &IrcState,
        app_handle: &tauri::AppHandle,
    ) {
        let mut irc_state = state.lock().unwrap();
        if let Some(lobby) = irc_state.lobby_states.get_mut(channel) {
            lobby.match_status = status.to_string();

            if status == "ready" {
                for slot in &mut lobby.slots {
                    if let Some(ref mut player) = slot.player {
                        player.is_ready = true;
                    }
                }
            }

            let _ = app_handle.emit("lobby-updated", lobby.clone());
        }
    }

    fn clear_host(channel: &str, state: &IrcState, app_handle: &tauri::AppHandle) {
        let mut irc_state = state.lock().unwrap();
        if let Some(lobby) = irc_state.lobby_states.get_mut(channel) {
            lobby.host = None;

            for slot in &mut lobby.slots {
                if let Some(ref mut player) = slot.player {
                    player.is_host = false;
                }
            }

            let _ = app_handle.emit("lobby-updated", lobby.clone());
        }
    }

    fn update_host(
        channel: &str,
        host_username: &str,
        state: &IrcState,
        app_handle: &tauri::AppHandle,
    ) {
        let mut irc_state = state.lock().unwrap();
        if let Some(lobby) = irc_state.lobby_states.get_mut(channel) {
            lobby.host = Some(host_username.to_string());

            for slot in &mut lobby.slots {
                if let Some(ref mut player) = slot.player {
                    player.is_host = player.username == host_username;
                }
            }

            let _ = app_handle.emit("lobby-updated", lobby.clone());
        }
    }

    fn move_player_to_slot(
        channel: &str,
        username: &str,
        new_slot_id: u8,
        team: Option<String>,
        state: &IrcState,
        app_handle: &tauri::AppHandle,
    ) {
        let mut irc_state = state.lock().unwrap();
        if let Some(lobby) = irc_state.lobby_states.get_mut(channel) {
            // Find the player in their current slot and remove them
            let mut player_data = None;
            for slot in &mut lobby.slots {
                if let Some(ref player) = slot.player {
                    if player.username == username {
                        player_data = slot.player.take();
                        break;
                    }
                }
            }

            // Move the player to their new slot
            if let Some(mut player) = player_data {
                if let Some(team) = team {
                    player.team = Some(team);
                }

                if let Some(slot) = lobby.slots.iter_mut().find(|s| s.id == new_slot_id) {
                    slot.player = Some(player);
                }
            }

            let _ = app_handle.emit("lobby-updated", lobby.clone());
        }
    }
}

fn remove_channel(channel: &str, state: &IrcState) {
    let mut irc_state = state.lock().unwrap();
    irc_state.channels.retain(|c: &String| c != channel);

    // If it's a lobby channel, remove lobby state
    if channel.starts_with("#mp_") {
        irc_state.lobby_states.remove(channel);
    }
}

fn clear_all_players(channel: &str, state: &IrcState) {
    let mut irc_state = state.lock().unwrap();
    if let Some(lobby) = irc_state.lobby_states.get_mut(channel) {
        for slot in &mut lobby.slots {
            slot.player = None;
        }
    }
}

async fn handle_irc_connection(
    mut client: irc::client::Client,
    app_handle: tauri::AppHandle,
    state: IrcState,
    mut command_receiver: tokio::sync::mpsc::UnboundedReceiver<IrcCommand>,
) {
    println!("Starting IRC connection handler...");

    let mut stream = client.stream().unwrap();

    loop {
        tokio::select! {
            // Handle incoming IRC messages
            message = stream.next() => {
                match message {
                    Some(Ok(msg)) => {
                        handle_incoming_message(msg, &app_handle, &state);
                    }
                    Some(Err(e)) => {
                        println!("Error receiving IRC message: {}", e);
                        break;
                    }
                    None => {
                        println!("IRC stream ended");
                        break;
                    }
                }
            }

            // Handle outgoing commands
            command = command_receiver.recv() => {
                match command {
                    Some(IrcCommand::SendMessage { channel, message }) => {
                        if let Err(e) = client.send_privmsg(&channel, &message) {
                            println!("Failed to send message: {}", e);
                        } else {
                            println!("Sent message to {}: {}", channel, message);
                        }
                    }
                    Some(IrcCommand::JoinChannel { channel }) => {
                        if let Err(e) = client.send_join(&channel) {
                            println!("Failed to join channel {}: {}", channel, e);
                        } else {
                            println!("Joined channel: {}", channel);
                        }
                    }
                    Some(IrcCommand::LeaveChannel { channel }) => {
                        if let Err(e) = client.send_part(&channel) {
                            println!("Failed to leave channel {}: {}", channel, e);
                        } else {
                            println!("Left channel: {}", channel);
                        }
                    }
                    Some(IrcCommand::Disconnect) => {
                        println!("Disconnect command received");
                        let _ = client.send_quit("Goodbye!");
                        break;
                    }
                    None => {
                        println!("Command channel closed");
                        break;
                    }
                }
            }
        }
    }

    println!("IRC connection handler ended");

    // Mark as disconnected and clean up state
    {
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

fn handle_incoming_message(
    msg: irc::proto::Message,
    app_handle: &tauri::AppHandle,
    state: &IrcState,
) {
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

                // Parse BanchoBot messages for lobby updates
                if channel.starts_with("#mp_") {
                    BanchoBotParser::parse_irc_message(&irc_message, state, app_handle);
                }

                // Emit the message to the frontend
                if let Err(e) = app_handle.emit("irc-message", &irc_message) {
                    println!("Failed to emit message to frontend: {}", e);
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
                Response::ERR_NOSUCHCHANNEL => {
                    if args.len() >= 2 {
                        let channel = &args[1];
                        println!("Channel {} does not exist", channel);

                        {
                            let mut irc_state = state.lock().unwrap();
                            irc_state.channels.retain(|c| c != channel);
                        }

                        if let Err(e) = app_handle.emit(
                            "channel-error",
                            serde_json::json!({
                                "channel": channel,
                                "error": "Channel does not exist"
                            }),
                        ) {
                            println!("Failed to emit channel error: {}", e);
                        }
                    }
                }
                Response::ERR_INVITEONLYCHAN => {
                    if args.len() >= 2 {
                        let channel = &args[1];
                        println!("Channel {} is invite only", channel);

                        {
                            let mut irc_state = state.lock().unwrap();
                            irc_state.channels.retain(|c| c != channel);
                        }

                        if let Err(e) = app_handle.emit(
                            "channel-error",
                            serde_json::json!({
                                "channel": channel,
                                "error": "Channel is invite only"
                            }),
                        ) {
                            println!("Failed to emit channel error: {}", e);
                        }
                    }
                }
                Response::ERR_BANNEDFROMCHAN => {
                    if args.len() >= 2 {
                        let channel = &args[1];
                        println!("Banned from channel {}", channel);

                        {
                            let mut irc_state = state.lock().unwrap();
                            irc_state.channels.retain(|c| c != channel);
                        }

                        if let Err(e) = app_handle.emit(
                            "channel-error",
                            serde_json::json!({
                                "channel": channel,
                                "error": "You are banned from this channel"
                            }),
                        ) {
                            println!("Failed to emit channel error: {}", e);
                        }
                    }
                }
                Response::ERR_CHANNELISFULL => {
                    if args.len() >= 2 {
                        let channel = &args[1];
                        println!("Channel {} is full", channel);

                        {
                            let mut irc_state = state.lock().unwrap();
                            irc_state.channels.retain(|c| c != channel);
                        }

                        if let Err(e) = app_handle.emit(
                            "channel-error",
                            serde_json::json!({
                                "channel": channel,
                                "error": "Channel is full"
                            }),
                        ) {
                            println!("Failed to emit channel error: {}", e);
                        }
                    }
                }
                Response::ERR_BADCHANNELKEY => {
                    if args.len() >= 2 {
                        let channel = &args[1];
                        println!("Wrong key for channel {}", channel);

                        // Remove the channel from our state since join failed
                        {
                            let mut irc_state = state.lock().unwrap();
                            irc_state.channels.retain(|c| c != channel);
                        }

                        if let Err(e) = app_handle.emit(
                            "channel-error",
                            serde_json::json!({
                                "channel": channel,
                                "error": "Wrong channel password"
                            }),
                        ) {
                            println!("Failed to emit channel error: {}", e);
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {
            // Handle other commands if needed
            // println!("Other IRC command: {:?}", msg.command);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
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
    }];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:osu_reffer_database.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .manage(IrcState::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            connect_to_bancho,
            send_message_to_channel,
            join_channel,
            leave_channel,
            disconnect_from_bancho,
            get_connection_status,
            get_joined_channels,
            get_lobby_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
