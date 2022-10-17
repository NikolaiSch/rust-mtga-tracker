use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventCourses {
    #[serde(rename = "Courses")]
    pub courses: Vec<Course>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Course {
    #[serde(rename = "CourseId")]
    pub course_id: String,

    #[serde(rename = "InternalEventName")]
    pub internal_event_name: String,

    #[serde(rename = "CurrentModule")]
    pub current_module: i64,

    #[serde(rename = "ModulePayload")]
    pub module_payload: String,

    #[serde(rename = "CourseDeckSummary")]
    pub course_deck_summary: CourseDeckSummary,

    #[serde(rename = "CourseDeck")]
    pub course_deck: CourseDeck,

    #[serde(rename = "CardPool")]
    pub card_pool: Vec<Option<serde_json::Value>>,

    #[serde(rename = "CurrentLosses")]
    pub current_losses: Option<i64>,

    #[serde(rename = "CurrentWins")]
    pub current_wins: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CourseDeck {
    #[serde(rename = "MainDeck")]
    pub main_deck: Vec<CommandZone>,

    #[serde(rename = "ReducedSideboard")]
    pub reduced_sideboard: Vec<CommandZone>,

    #[serde(rename = "Sideboard")]
    pub sideboard: Vec<CommandZone>,

    #[serde(rename = "CommandZone")]
    pub command_zone: Vec<CommandZone>,

    #[serde(rename = "Companions")]
    pub companions: Vec<Option<serde_json::Value>>,

    #[serde(rename = "CardSkins")]
    pub card_skins: Vec<CardSkin>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CardSkin {
    #[serde(rename = "GrpId")]
    pub grp_id: i64,

    #[serde(rename = "CCV")]
    pub ccv: Ccv,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandZone {
    #[serde(rename = "cardId")]
    pub card_id: i64,

    #[serde(rename = "quantity")]
    pub quantity: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CourseDeckSummary {
    #[serde(rename = "DeckId")]
    pub deck_id: String,

    #[serde(rename = "Mana")]
    pub mana: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Attributes")]
    pub attributes: Vec<Attribute>,

    #[serde(rename = "DeckTileId")]
    pub deck_tile_id: i64,

    #[serde(rename = "DeckArtId")]
    pub deck_art_id: i64,

    #[serde(rename = "FormatLegalities")]
    pub format_legalities: FormatLegalities,

    #[serde(rename = "PreferredCosmetics")]
    pub preferred_cosmetics: PreferredCosmetics,

    #[serde(rename = "DeckValidationSummaries")]
    pub deck_validation_summaries: Vec<Option<serde_json::Value>>,

    #[serde(rename = "UnownedCards")]
    pub unowned_cards: FormatLegalities,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attribute {
    #[serde(rename = "name")]
    pub name: Name,

    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormatLegalities {
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreferredCosmetics {
    #[serde(rename = "Emotes")]
    pub emotes: Vec<Option<serde_json::Value>>,

    #[serde(rename = "Sleeve")]
    pub sleeve: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Ccv {
    #[serde(rename = "DA")]
    Da,

    #[serde(rename = "FF")]
    Ff,

    #[serde(rename = "SH")]
    Sh,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Name {
    #[serde(rename = "Format")]
    Format,

    #[serde(rename = "IsFavorite")]
    IsFavorite,

    #[serde(rename = "LastPlayed")]
    LastPlayed,

    #[serde(rename = "LastUpdated")]
    LastUpdated,

    #[serde(rename = "TileID")]
    TileId,

    #[serde(rename = "Version")]
    Version,
}


impl EventCourses {
    pub fn new(input: &str) -> EventCourses {
        serde_json::from_str::<EventCourses>(input).unwrap()
    }
}