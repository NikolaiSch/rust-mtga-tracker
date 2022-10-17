use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Decks {
    #[serde(rename = "MainDeck")]
    pub main_deck: Vec<MainDeck>,

    #[serde(rename = "ReducedSideboard")]
    pub reduced_sideboard: Vec<MainDeck>,

    #[serde(rename = "Sideboard")]
    pub sideboard: Vec<MainDeck>,

    #[serde(rename = "CommandZone")]
    pub command_zone: Vec<Option<MainDeck>>,

    #[serde(rename = "Companions")]
    pub companions: Vec<Option<MainDeck>>,

    #[serde(rename = "CardSkins")]
    pub card_skins: Vec<CardSkin>,

    #[serde(rename = "DoPreferReducedSideboard")]
    pub do_prefer_reduced_sideboard: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardSkin {
    #[serde(rename = "GrpId")]
    pub grp_id: i64,

    #[serde(rename = "CCV")]
    pub ccv: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainDeck {
    #[serde(rename = "cardId")]
    pub card_id: i64,

    #[serde(rename = "quantity")]
    pub quantity: i64,
}


impl Decks {
    pub fn new(input: &str) -> Decks {
        serde_json::from_str::<Decks>(input).unwrap()
    }
}