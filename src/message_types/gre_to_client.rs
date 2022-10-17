use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GreToClient {
    #[serde(rename = "transactionId")]
    pub transaction_id: String,

    #[serde(rename = "timestamp")]
    pub timestamp: String,

    #[serde(rename = "greToClientEvent")]
    pub gre_to_client_event: GreToClientEvent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GreToClientEvent {
    #[serde(rename = "greToClientMessages")]
    pub gre_to_client_messages: Vec<GreToClientMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GreToClientMessage {
    #[serde(rename = "type")]
    pub gre_to_client_message_type: String,

    #[serde(rename = "systemSeatIds")]
    pub system_seat_ids: Vec<i64>,

    #[serde(rename = "msgId")]
    pub msg_id: i64,

    #[serde(rename = "connectResp")]
    pub connect_resp: Option<ConnectResp>,

    #[serde(rename = "dieRollResultsResp")]
    pub die_roll_results_resp: Option<DieRollResultsResp>,

    #[serde(rename = "gameStateId")]
    pub game_state_id: Option<i64>,

    #[serde(rename = "gameStateMessage")]
    pub game_state_message: Option<GameStateMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectResp {
    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "protoVer")]
    pub proto_ver: String,

    #[serde(rename = "settings")]
    pub settings: Settings,

    #[serde(rename = "deckMessage")]
    pub deck_message: DeckMessage,

    #[serde(rename = "grpVersion")]
    pub grp_version: Version,

    #[serde(rename = "greVersion")]
    pub gre_version: Version,

    #[serde(rename = "skins")]
    pub skins: Vec<Skin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeckMessage {
    #[serde(rename = "deckCards")]
    pub deck_cards: Vec<i64>,

    #[serde(rename = "sideboardCards")]
    pub sideboard_cards: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "majorVersion")]
    pub major_version: i64,

    #[serde(rename = "minorVersion")]
    pub minor_version: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "stops")]
    pub stops: Vec<Stop>,

    #[serde(rename = "autoPassOption")]
    pub auto_pass_option: String,

    #[serde(rename = "graveyardOrder")]
    pub graveyard_order: String,

    #[serde(rename = "manaSelectionType")]
    pub mana_selection_type: String,

    #[serde(rename = "defaultAutoPassOption")]
    pub default_auto_pass_option: String,

    #[serde(rename = "smartStopsSetting")]
    pub smart_stops_setting: String,

    #[serde(rename = "autoTapStopsSetting")]
    pub auto_tap_stops_setting: String,

    #[serde(rename = "autoOptionalPaymentCancellationSetting")]
    pub auto_optional_payment_cancellation_setting: String,

    #[serde(rename = "transientStops")]
    pub transient_stops: Vec<Stop>,

    #[serde(rename = "stackAutoPassOption")]
    pub stack_auto_pass_option: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stop {
    #[serde(rename = "stopType")]
    pub stop_type: String,

    #[serde(rename = "appliesTo")]
    pub applies_to: AppliesTo,

    #[serde(rename = "status")]
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skin {
    #[serde(rename = "catalogId")]
    pub catalog_id: i64,

    #[serde(rename = "skinCode")]
    pub skin_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DieRollResultsResp {
    #[serde(rename = "playerDieRolls")]
    pub player_die_rolls: Vec<PlayerDieRoll>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerDieRoll {
    #[serde(rename = "systemSeatId")]
    pub system_seat_id: i64,

    #[serde(rename = "rollValue")]
    pub roll_value: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStateMessage {
    #[serde(rename = "type")]
    pub game_state_message_type: String,

    #[serde(rename = "gameStateId")]
    pub game_state_id: i64,

    #[serde(rename = "gameInfo")]
    pub game_info: GameInfo,

    #[serde(rename = "teams")]
    pub teams: Vec<Team>,

    #[serde(rename = "players")]
    pub players: Vec<Player>,

    #[serde(rename = "turnInfo")]
    pub turn_info: TurnInfo,

    #[serde(rename = "zones")]
    pub zones: Vec<Zone>,

    #[serde(rename = "diffDeletedInstanceIds")]
    pub diff_deleted_instance_ids: Vec<i64>,

    #[serde(rename = "timers")]
    pub timers: Vec<Timer>,

    #[serde(rename = "update")]
    pub update: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameInfo {
    #[serde(rename = "matchID")]
    pub match_id: String,

    #[serde(rename = "gameNumber")]
    pub game_number: i64,

    #[serde(rename = "stage")]
    pub stage: String,

    #[serde(rename = "type")]
    pub game_info_type: String,

    #[serde(rename = "variant")]
    pub variant: String,

    #[serde(rename = "matchState")]
    pub match_state: String,

    #[serde(rename = "matchWinCondition")]
    pub match_win_condition: String,

    #[serde(rename = "maxTimeoutCount")]
    pub max_timeout_count: i64,

    #[serde(rename = "maxPipCount")]
    pub max_pip_count: i64,

    #[serde(rename = "timeoutDurationSec")]
    pub timeout_duration_sec: i64,

    #[serde(rename = "superFormat")]
    pub super_format: String,

    #[serde(rename = "mulliganType")]
    pub mulligan_type: String,

    #[serde(rename = "deckConstraintInfo")]
    pub deck_constraint_info: DeckConstraintInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeckConstraintInfo {
    #[serde(rename = "minDeckSize")]
    pub min_deck_size: i64,

    #[serde(rename = "maxDeckSize")]
    pub max_deck_size: i64,

    #[serde(rename = "maxSideboardSize")]
    pub max_sideboard_size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "lifeTotal")]
    pub life_total: i64,

    #[serde(rename = "systemSeatNumber")]
    pub system_seat_number: i64,

    #[serde(rename = "maxHandSize")]
    pub max_hand_size: i64,

    #[serde(rename = "teamId")]
    pub team_id: i64,

    #[serde(rename = "timerIds")]
    pub timer_ids: Vec<i64>,

    #[serde(rename = "controllerSeatId")]
    pub controller_seat_id: i64,

    #[serde(rename = "controllerType")]
    pub controller_type: String,

    #[serde(rename = "pendingMessageType")]
    pub pending_message_type: Option<String>,

    #[serde(rename = "startingLifeTotal")]
    pub starting_life_total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "playerIds")]
    pub player_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timer {
    #[serde(rename = "timerId")]
    pub timer_id: i64,

    #[serde(rename = "type")]
    pub timer_type: String,

    #[serde(rename = "durationSec")]
    pub duration_sec: i64,

    #[serde(rename = "running")]
    pub running: Option<bool>,

    #[serde(rename = "behavior")]
    pub behavior: Behavior,

    #[serde(rename = "warningThresholdSec")]
    pub warning_threshold_sec: Option<i64>,

    #[serde(rename = "elapsedMs")]
    pub elapsed_ms: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurnInfo {
    #[serde(rename = "decisionPlayer")]
    pub decision_player: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Zone {
    #[serde(rename = "zoneId")]
    pub zone_id: i64,

    #[serde(rename = "type")]
    pub zone_type: String,

    #[serde(rename = "visibility")]
    pub visibility: Visibility,

    #[serde(rename = "ownerSeatId")]
    pub owner_seat_id: Option<i64>,

    #[serde(rename = "viewers")]
    pub viewers: Option<Vec<i64>>,

    #[serde(rename = "objectInstanceIds")]
    pub object_instance_ids: Option<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AppliesTo {
    #[serde(rename = "SettingScope_Opponents")]
    SettingScopeOpponents,

    #[serde(rename = "SettingScope_Team")]
    SettingScopeTeam,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "SettingStatus_Clear")]
    SettingStatusClear,

    #[serde(rename = "SettingStatus_Set")]
    SettingStatusSet,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Behavior {
    #[serde(rename = "TimerBehavior_StartDelayedTimer")]
    TimerBehaviorStartDelayedTimer,

    #[serde(rename = "TimerBehavior_TakeControl")]
    TimerBehaviorTakeControl,

    #[serde(rename = "TimerBehavior_Timeout")]
    TimerBehaviorTimeout,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "Visibility_Hidden")]
    VisibilityHidden,

    #[serde(rename = "Visibility_Private")]
    VisibilityPrivate,

    #[serde(rename = "Visibility_Public")]
    VisibilityPublic,
}

impl Decks {
    pub fn new(input: &str) -> GreToClient {
        serde_json::from_str::<GreToClient>(input).unwrap()
    }
}