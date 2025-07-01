use crate::types::*;
use regex::Regex;
use tauri::Emitter;

// BanchoBot parsing logic
pub struct BanchoBotParser;

impl BanchoBotParser {
    pub fn parse_irc_message(
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

        // Current beatmap
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

        // Active mods
        if let Ok(regex) = Regex::new(r"^Active mods: (.+)$") {
            if let Some(captures) = regex.captures(text) {
                let mods_str = captures.get(1).unwrap().as_str();
                let mut mods: Vec<String> = mods_str
                    .split(", ")
                    .map(|m| Self::normalize_mod_name(m))
                    .collect();

                let mut freemod = false;
                mods.retain(|m| {
                    if m == "Freemod" {
                        freemod = true;
                        false
                    } else {
                        true
                    }
                });

                Self::update_mods(channel, mods, freemod, state, app_handle);
                return true;
            }
        }

        // Beatmap changed
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

        // Mods changed
        if let Ok(regex) = Regex::new(r"^Enabled (.+), disabled FreeMod$") {
            if let Some(captures) = regex.captures(text) {
                let mods_str = captures.get(1).unwrap().as_str();
                let mods = mods_str
                    .split(", ")
                    .map(|m| Self::normalize_mod_name(m))
                    .collect();
                Self::update_mods(channel, mods, false, state, app_handle);
                return true;
            }
        }

        if text == "Disabled all mods, enabled FreeMod" {
            Self::update_mods(channel, Vec::new(), true, state, app_handle);
            return true;
        }

        if text == "Disabled all mods, disabled FreeMod" {
            Self::update_mods(channel, Vec::new(), false, state, app_handle);
            return true;
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

    fn normalize_mod_name(mod_name: &str) -> String {
        match mod_name {
            "Hidden" => "HD".to_string(),
            "HardRock" => "HR".to_string(),
            "DoubleTime" => "DT".to_string(),
            "Flashlight" => "FL".to_string(),
            "NoFail" => "NF".to_string(),
            "Easy" => "EZ".to_string(),
            "HalfTime" => "HT".to_string(),
            "SuddenDeath" => "SD".to_string(),
            "Perfect" => "PF".to_string(),
            "Relax" => "RX".to_string(),
            "Nightcore" => "NC".to_string(),
            "SpunOut" => "SO".to_string(),
            _ => mod_name.to_string(),
        }
    }

    fn update_mods(
        channel: &str,
        mods: Vec<String>,
        freemod: bool,
        state: &IrcState,
        app_handle: &tauri::AppHandle,
    ) {
        let mut irc_state = state.lock().unwrap();
        if let Some(lobby) = irc_state.lobby_states.get_mut(channel) {
            lobby.selected_mods = mods;
            lobby.freemod = freemod;
            let _ = app_handle.emit("lobby-updated", lobby.clone());
        }
    }
}
