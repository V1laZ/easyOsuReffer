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
