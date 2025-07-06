use crate::banchobot_parser::BanchoBotParser;
use crate::types::*;
use futures::stream::StreamExt;
use irc::client::prelude::*;
use tauri::Emitter;

pub async fn handle_irc_connection(
    mut client: irc::client::Client,
    app_handle: tauri::AppHandle,
    state: IrcState,
    mut command_receiver: tokio::sync::mpsc::UnboundedReceiver<IrcCommand>,
) {
    println!("Starting IRC connection handler...");

    let mut stream = client.stream().unwrap();

    loop {
        tokio::select! {
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

            command = command_receiver.recv() => {
                match command {
                    Some(IrcCommand::SendMessage { room_id, message }) => {
                        if let Err(e) = client.send_privmsg(&room_id, &message) {
                            println!("Failed to send message: {}", e);
                        } else {
                            println!("Sent message to {}: {}", room_id, message);

                            // Create our own message and add it to the room
                            let current_username = {
                                let irc_state = state.lock().unwrap();
                                irc_state.current_username.clone().unwrap_or_default()
                            };

                            let our_message = IrcMessage {
                                room_id: room_id.clone(),
                                username: current_username,
                                message: message.clone(),
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                                is_private: false,
                            };

                            {
                                let mut irc_state = state.lock().unwrap();
                                if let Some(room) = irc_state.rooms.get_mut(&room_id) {
                                    room.add_message(our_message.clone());
                                }
                            }

                            if let Err(e) = app_handle.emit("irc-message", &our_message) {
                                println!("Failed to emit our message to frontend: {}", e);
                            }
                        }
                    }
                    Some(IrcCommand::SendPrivateMessage { username, message }) => {
                        if let Err(e) = client.send_privmsg(&username, &message) {
                            println!("Failed to send private message: {}", e);
                        } else {
                            println!("Sent private message to {}: {}", username, message);

                            // Create our own message and add it to the PM room
                            let current_username = {
                                let irc_state = state.lock().unwrap();
                                irc_state.current_username.clone().unwrap_or_default()
                            };

                            let our_message = IrcMessage {
                                room_id: username.clone(),
                                username: current_username,
                                message: message.clone(),
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                                is_private: true,
                            };

                            {
                                let mut irc_state = state.lock().unwrap();
                                if let Some(room) = irc_state.rooms.get_mut(&username) {
                                    room.add_message(our_message.clone());
                                }
                            }

                            if let Err(e) = app_handle.emit("irc-message", &our_message) {
                                println!("Failed to emit our PM to frontend: {}", e);
                            }
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
        irc_state.rooms.clear();
        irc_state.active_room = None;
        irc_state.config = None;
        irc_state.client = None;
        irc_state.message_sender = None;
        irc_state.current_username = None;
        irc_state.lobby_states.clear();
    }

    if let Err(e) = app_handle.emit("irc-disconnected", ()) {
        println!("Failed to emit disconnect event: {}", e);
    }
}

fn handle_incoming_message(
    msg: irc::proto::Message,
    app_handle: &tauri::AppHandle,
    state: &IrcState,
) {
    match msg.command {
        Command::PRIVMSG(room, text) => {
            if let Some(prefix) = msg.prefix {
                let nick = match prefix {
                    irc::proto::Prefix::Nickname(nick, _, _) => nick,
                    irc::proto::Prefix::ServerName(server) => server,
                };

                let is_private = !room.starts_with("#");

                let room_id = if is_private {
                    let current_username = {
                        let irc_state = state.lock().unwrap();
                        irc_state.current_username.clone().unwrap_or_default()
                    };

                    if nick == current_username {
                        // This is our outgoing message, use recipient as room ID
                        room.clone()
                    } else {
                        // This is incoming PM, use sender as room ID
                        nick.clone()
                    }
                } else {
                    // For channels, use the channel name
                    room.clone()
                };

                let irc_message = IrcMessage {
                    room_id: room_id.clone(),
                    username: nick.clone(),
                    message: text.clone(),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    is_private,
                };

                println!("[{}] <{}> {}", room_id, nick, text);

                {
                    let mut irc_state = state.lock().unwrap();

                    // Create room if it doesn't exist (for incoming PMs)
                    if is_private && !irc_state.rooms.contains_key(&room_id) {
                        let new_room = Room::new_private_message(room_id.clone());
                        irc_state.rooms.insert(room_id.clone(), new_room);
                    }

                    // Add message to appropriate room
                    if let Some(room_obj) = irc_state.rooms.get_mut(&room_id) {
                        room_obj.add_message(irc_message.clone());
                    }
                }

                if room_id.starts_with("#mp_") {
                    BanchoBotParser::parse_irc_message(&irc_message, state, app_handle);
                }

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

                {
                    let mut irc_state = state.lock().unwrap();
                    let current_username = irc_state.current_username.clone().unwrap_or_default();
                    if nick == current_username {
                        if !irc_state.rooms.contains_key(&channel) {
                            let new_room = Room::new_channel(channel.clone());
                            irc_state.rooms.insert(channel.clone(), new_room);
                        }
                        irc_state.active_room = Some(channel.clone());
                    }
                }

                println!("{} joined {}", nick, channel);

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

                {
                    let mut irc_state = state.lock().unwrap();
                    if nick == irc_state.current_username.clone().unwrap_or_default() {
                        irc_state.rooms.remove(&channel);
                        irc_state.active_room = None;
                    }
                }

                println!("{} left {}", nick, channel);

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

            match response {
                Response::RPL_WELCOME => {
                    println!("Successfully connected and welcomed to the server!");
                }
                Response::RPL_NAMREPLY => {
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
                            irc_state.rooms.remove(channel);
                        }

                        if let Err(e) = app_handle.emit(
                            "room-error",
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
                            irc_state.rooms.remove(channel);
                        }

                        if let Err(e) = app_handle.emit(
                            "room-error",
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
                            irc_state.rooms.remove(channel);
                        }

                        if let Err(e) = app_handle.emit(
                            "room-error",
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
                            irc_state.rooms.remove(channel);
                        }

                        if let Err(e) = app_handle.emit(
                            "room-error",
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

                        {
                            let mut irc_state = state.lock().unwrap();
                            irc_state.rooms.remove(channel);
                        }

                        if let Err(e) = app_handle.emit(
                            "room-error",
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
