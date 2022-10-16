use std::fs;
use crate::message_types::start_hook::StartHook;

pub struct Log {
    pub raw_data: String,

    pub profileData: Option<StartHook>
}

impl Log {
    pub fn new(file_location: &str) -> Log {
        Log {
            raw_data: fs::read_to_string(file_location).unwrap(),
            profileData: None
        }
    }

    pub fn init(&mut self) -> () {
        let log_presplit = &self.raw_data.replace("[UnityCrossThreadLogger]", "!");
        let log_vec = log_presplit.split("!");

        log_vec.for_each(|val| {
            self.parse_from_str(val)
        })
    }

    pub fn parse_from_str(&mut self, data: &str) -> () {
        if data.contains("<== StartHook") {
            let transformed_data = data
            .trim()
            .lines()
            .nth(2).unwrap();

            self.profileData = Some(StartHook::create_from_str(transformed_data).unwrap());
        }
    }
}
