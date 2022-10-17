#[deny(non_snake_case)]
use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct StartHook {
    pub InventoryInfo: Inventory
}

impl StartHook {
    pub fn new(input: &str) -> StartHook {
        from_str(input).unwrap()
    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Inventory {
    pub Gems: u32,
    pub Gold: u32,
    pub TotalVaultProgress: u32,
    pub WildCardCommons: u32,
    pub WildCardUnCommons: u32,
    pub WildCardRares: u32,
    pub WildCardMythics: u32
}
