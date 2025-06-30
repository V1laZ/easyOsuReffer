use anyhow::Result;
use futures::stream::StreamExt;
use irc::client::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{Emitter, State};
use tauri_plugin_sql::{Migration, MigrationKind};

// IRC client state
#[derive(Debug)]
pub struct IrcClientState {
    pub connected: bool,
    pub channels: Vec<String>,
    pub config: Option<ConnectionConfig>,
    pub client: Option<Arc<Mutex<irc::client::Client>>>,
    pub message_sender: Option<tokio::sync::mpsc::UnboundedSender<IrcCommand>>,
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

        // Remove the channel from the channels list
        {
            let mut irc_state = state.lock().unwrap();
            irc_state.channels.retain(|c| c != &channel);
        }

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
