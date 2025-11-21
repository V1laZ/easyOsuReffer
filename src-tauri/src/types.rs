use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Room types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RoomType {
    Channel,
    PrivateMessage,
    MultiplayerLobby,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: String,
    pub display_name: String,
    pub room_type: RoomType,
    pub messages: Vec<IrcMessage>,
    pub unread_count: u32,
    pub is_active: bool,
}

impl Room {
    pub fn new_channel(channel_name: String) -> Self {
        let is_multiplayer = channel_name.starts_with("#mp_");
        let room_type = if is_multiplayer {
            RoomType::MultiplayerLobby
        } else {
            RoomType::Channel
        };

        Self {
            display_name: channel_name.clone(),
            id: channel_name,
            room_type,
            messages: Vec::new(),
            unread_count: 0,
            is_active: false,
        }
    }

    pub fn new_private_message(username: String) -> Self {
        Self {
            id: username.clone(),
            display_name: username,
            room_type: RoomType::PrivateMessage,
            messages: Vec::new(),
            unread_count: 0,
            is_active: false,
        }
    }

    pub fn add_message(&mut self, message: IrcMessage) {
        self.messages.push(message);
        if !self.is_active {
            self.unread_count += 1;
        }
    }

    pub fn mark_as_read(&mut self) {
        self.unread_count = 0;
    }
}

// Lobby state structures
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub username: String,
    pub team: Option<String>, // "red" or "blue"
    pub is_ready: bool,
    pub is_playing: bool,
    pub is_host: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerSlot {
    pub id: u8,
    pub player: Option<Player>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurrentMap {
    pub beatmap_id: u64,
    pub title: String,
    pub difficulty: String,
    pub artist: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LobbySettings {
    pub room_name: String,
    pub team_mode: String,     // "HeadToHead", "TagCoop", "TeamVs", "TagTeamVs"
    pub win_condition: String, // "Score", "Accuracy", "Combo", "ScoreV2"
    pub size: u8,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LobbyState {
    pub channel: String,
    pub settings: Option<LobbySettings>,
    pub current_map: Option<CurrentMap>,
    pub slots: Vec<PlayerSlot>,
    pub match_status: String, // "idle", "ready", "starting", "active"
    pub host: Option<String>,
    pub freemod: bool,
    pub selected_mods: Vec<String>,
    pub current_mappool_id: Option<u64>,
}

impl LobbyState {
    pub fn new(channel: String) -> Self {
        let slots = (1..=16).map(|id| PlayerSlot { id, player: None }).collect();

        Self {
            channel,
            settings: None,
            current_map: None,
            current_mappool_id: None,
            slots,
            match_status: "idle".to_string(),
            host: None,
            freemod: false,
            selected_mods: Vec::new(),
        }
    }
}

// IRC client state
#[derive(Debug)]
pub struct IrcClientState {
    pub connected: bool,
    pub rooms: HashMap<String, Room>,
    pub active_room_id: Option<String>,
    pub config: Option<ConnectionConfig>,
    pub client: Option<Arc<Mutex<irc::client::Client>>>,
    pub message_sender: Option<tokio::sync::mpsc::UnboundedSender<IrcCommand>>,
    pub lobby_states: HashMap<String, LobbyState>,
    pub current_username: Option<String>,
}

#[derive(Debug, Clone)]
pub enum IrcCommand {
    SendMessage { room_id: String, message: String },
    JoinChannel { channel: String },
    LeaveChannel { channel: String },
    SendPrivateMessage { username: String, message: String },
    Disconnect,
}

impl Default for IrcClientState {
    fn default() -> Self {
        Self {
            connected: false,
            rooms: HashMap::new(),
            active_room_id: None,
            config: None,
            client: None,
            message_sender: None,
            lobby_states: HashMap::new(),
            current_username: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IrcMessage {
    pub room_id: String,
    pub username: String,
    pub message: String,
    pub timestamp: u64,
    pub is_private: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionConfig {
    pub username: String,
    pub password: String,
}

pub type IrcState = Arc<Mutex<IrcClientState>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthCallbackData {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i32,
}

// Beatmap data structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BeatmapData {
    pub id: u64,
    pub beatmapset_id: u64,
    pub artist: String,
    pub title: String,
    pub difficulty: String,
    pub mapper: String,
    pub mode: u8,
    pub total_length: u32,
    pub bpm: f64,
    pub difficulty_rating: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiBeatmapResponse {
    pub id: u64,
    pub beatmapset_id: u64,
    pub version: String,
    pub mode_int: u8,
    pub total_length: u32,
    pub bpm: f64,
    pub difficulty_rating: f64,
    pub beatmapset: OsuApiBeatmapset,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiBeatmapset {
    pub artist: String,
    pub title: String,
    pub creator: String,
}
