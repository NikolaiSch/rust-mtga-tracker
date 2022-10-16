#[deny(non_snake_case)]
use serde::Deserialize;
use serde_json::{from_str, from_value};

#[derive(Deserialize, Debug)]
pub struct StartHook {
    pub InventoryInfo: Inventory
}

impl StartHook {
    pub fn create_from_str(input: &str) -> Result<StartHook, ()> {
        let d: StartHook = from_value(from_str(input).unwrap()).unwrap();
        Ok(d)
    }
}

#[derive(Deserialize, Debug)]
pub struct Inventory {
    pub Gems: u32,
    pub Gold: u32,
    pub TotalVaultProgress: u32,
    pub WildCardCommons: u32,
    pub WildCardUnCommons: u32,
    pub WildCardRares: u32,
    pub WildCardMythics: u32
}
