use serde::{Deserialize, Serialize};

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

// User data structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
    pub id: u64,
    pub username: String,
    pub avatar_url: String,
    pub country: String,
    pub pp: f64,
    pub rank: Option<u64>,
    pub country_rank: Option<u64>,
    pub accuracy: f64,
}

// Comprehensive osu!api UserExtended response type
#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiUserResponse {
    // Core fields
    pub id: u64,
    pub username: String,
    pub avatar_url: String,
    pub country_code: String,
    pub default_group: String,
    pub is_active: bool,
    pub is_bot: bool,
    pub is_deleted: bool,
    pub is_online: bool,
    pub is_supporter: bool,
    pub last_visit: Option<String>,
    pub pm_friends_only: bool,
    pub profile_colour: Option<String>,

    // Extended fields
    pub cover_url: String,
    pub discord: Option<String>,
    pub has_supported: bool,
    pub interests: Option<String>,
    pub join_date: String,
    pub location: Option<String>,
    pub max_blocks: u32,
    pub max_friends: u32,
    pub occupation: Option<String>,
    pub playmode: String,
    pub playstyle: Option<Vec<String>>,
    pub post_count: u32,
    pub profile_hue: Option<u32>,
    pub profile_order: Vec<String>,
    pub title: Option<String>,
    pub title_url: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,

    // Nested objects
    pub country: OsuApiCountry,
    pub cover: OsuApiCover,
    pub kudosu: OsuApiKudosu,
    pub statistics: OsuApiUserStatistics,

    // Optional extended attributes
    pub account_history: Option<Vec<OsuApiAccountHistory>>,
    pub active_tournament_banner: Option<OsuApiProfileBanner>,
    pub active_tournament_banners: Option<Vec<OsuApiProfileBanner>>,
    pub badges: Option<Vec<OsuApiBadge>>,
    pub beatmap_playcounts_count: Option<u32>,
    pub favourite_beatmapset_count: Option<u32>,
    pub follower_count: Option<u32>,
    pub graveyard_beatmapset_count: Option<u32>,
    pub groups: Option<Vec<OsuApiUserGroup>>,
    pub guest_beatmapset_count: Option<u32>,
    pub is_restricted: Option<bool>,
    pub loved_beatmapset_count: Option<u32>,
    pub mapping_follower_count: Option<u32>,
    pub monthly_playcounts: Option<Vec<OsuApiMonthlyPlaycount>>,
    pub nominated_beatmapset_count: Option<u32>,
    pub page: Option<OsuApiPage>,
    pub pending_beatmapset_count: Option<u32>,
    pub previous_usernames: Option<Vec<String>>,
    pub rank_highest: Option<OsuApiRankHighest>,
    pub rank_history: Option<OsuApiRankHistory>,
    pub ranked_beatmapset_count: Option<u32>,
    pub replays_watched_counts: Option<Vec<OsuApiReplayWatchedCount>>,
    pub scores_best_count: Option<u32>,
    pub scores_first_count: Option<u32>,
    pub scores_recent_count: Option<u32>,
    pub session_verified: Option<bool>,
    pub statistics_rulesets: Option<OsuApiStatisticsRulesets>,
    pub support_level: Option<u32>,
    pub user_achievements: Option<Vec<OsuApiUserAchievement>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiCountry {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiCover {
    pub custom_url: Option<String>,
    pub url: String,
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiKudosu {
    pub total: u32,
    pub available: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiUserStatistics {
    pub count_100: Option<u64>,
    pub count_300: Option<u64>,
    pub count_50: Option<u64>,
    pub count_miss: Option<u64>,
    pub level: OsuApiLevel,
    pub global_rank: Option<u64>,
    pub pp: f64,
    pub ranked_score: u64,
    pub hit_accuracy: f64,
    pub play_count: u64,
    pub play_time: Option<u64>,
    pub total_score: u64,
    pub total_hits: u64,
    pub maximum_combo: u64,
    pub replays_watched_by_others: u64,
    pub is_ranked: bool,
    pub grade_counts: OsuApiGradeCounts,
    pub country_rank: Option<u64>,
    pub rank: Option<OsuApiRank>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiLevel {
    pub current: u32,
    pub progress: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiGradeCounts {
    pub ss: u32,
    pub ssh: u32,
    pub s: u32,
    pub sh: u32,
    pub a: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiRank {
    pub global: Option<u64>,
    pub country: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiAccountHistory {
    pub description: Option<String>,
    pub id: u64,
    pub length: u64,
    pub permanent: bool,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub history_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiProfileBanner {
    pub id: u64,
    pub tournament_id: u64,
    pub image: Option<String>,
    #[serde(rename = "image@2x")]
    pub image_2x: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiBadge {
    pub awarded_at: String,
    pub description: String,
    pub image_url: String,
    #[serde(rename = "image@2x_url")]
    pub image_2x_url: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiUserGroup {
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub short_name: String,
    pub description: Option<String>,
    pub colour: Option<String>,
    pub playmodes: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiMonthlyPlaycount {
    pub start_date: String,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiPage {
    pub html: String,
    pub raw: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiRankHighest {
    pub rank: u64,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiRankHistory {
    pub mode: String,
    pub data: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiReplayWatchedCount {
    pub start_date: String,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiStatisticsRulesets {
    pub osu: Option<OsuApiUserStatistics>,
    pub taiko: Option<OsuApiUserStatistics>,
    pub fruits: Option<OsuApiUserStatistics>,
    pub mania: Option<OsuApiUserStatistics>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsuApiUserAchievement {
    pub achieved_at: String,
    pub achievement_id: u64,
}
