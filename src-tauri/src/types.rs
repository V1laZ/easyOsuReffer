use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
