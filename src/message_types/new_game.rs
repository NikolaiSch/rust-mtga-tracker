use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewGame {
    #[serde(rename = "transactionId")]
    pub transaction_id: String,

    #[serde(rename = "requestId")]
    pub request_id: i64,

    #[serde(rename = "timestamp")]
    pub timestamp: String,

    #[serde(rename = "matchGameRoomStateChangedEvent")]
    pub match_game_room_state_changed_event: MatchGameRoomStateChangedEvent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchGameRoomStateChangedEvent {
    #[serde(rename = "gameRoomInfo")]
    pub game_room_info: GameRoomInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameRoomInfo {
    #[serde(rename = "gameRoomConfig")]
    pub game_room_config: GameRoomConfig,

    #[serde(rename = "stateType")]
    pub state_type: String,

    #[serde(rename = "players")]
    pub players: Vec<Player>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameRoomConfig {
    #[serde(rename = "eventId")]
    pub event_id: String,

    #[serde(rename = "reservedPlayers")]
    pub reserved_players: Vec<ReservedPlayer>,

    #[serde(rename = "matchId")]
    pub match_id: String,

    #[serde(rename = "greConfig")]
    pub gre_config: GreConfig,

    #[serde(rename = "greHostLoggerLevel")]
    pub gre_host_logger_level: String,

    #[serde(rename = "joinRoomTimeoutSecs")]
    pub join_room_timeout_secs: i64,

    #[serde(rename = "playerDisconnectTimeoutSecs")]
    pub player_disconnect_timeout_secs: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GreConfig {
    #[serde(rename = "gameStateRedactorConfiguration")]
    pub game_state_redactor_configuration: GameStateRedactorConfiguration,

    #[serde(rename = "clipsConfiguration")]
    pub clips_configuration: Configuration,

    #[serde(rename = "checkpointConfiguration")]
    pub checkpoint_configuration: Configuration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStateRedactorConfiguration {
    #[serde(rename = "enableRedaction")]
    pub enable_redaction: bool,

    #[serde(rename = "enableForceDiff")]
    pub enable_force_diff: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservedPlayer {
    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "playerName")]
    pub player_name: String,

    #[serde(rename = "systemSeatId")]
    pub system_seat_id: i64,

    #[serde(rename = "teamId")]
    pub team_id: i64,

    #[serde(rename = "connectionInfo")]
    pub connection_info: ConnectionInfo,

    #[serde(rename = "courseId")]
    pub course_id: String,

    #[serde(rename = "sessionId")]
    pub session_id: String,

    #[serde(rename = "platformId")]
    pub platform_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionInfo {
    #[serde(rename = "connectionState")]
    pub connection_state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "userId")]
    pub user_id: String,

    #[serde(rename = "systemSeatId")]
    pub system_seat_id: i64,
}

impl NewGame {
    pub fn new(input: &str) -> NewGame {
        serde_json::from_str::<NewGame>(input).unwrap()
    }
}
