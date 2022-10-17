use std::fs;

use crate::message_types::{decks::*, event_courses::*, start_hook::*, rank_info::*};

#[derive(Clone)]
pub struct Log {
    pub raw_data: String,

    pub profile_data: Option<StartHook>,
    pub rank_info: Option<RankInfo>,
    pub event_courses: Option<EventCourses>,
    pub decks: Option<Decks>
}

impl Log {
    pub fn new(file_location: &str) -> Log {
        Log {
            raw_data: fs::read_to_string(file_location).unwrap(),
            profile_data: None,
            rank_info: None,
            event_courses: None,
            decks: None
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
            self.profile_data = Some(StartHook::new(internal_parse(data, 2)))
        }

        if data.contains("<== Rank_GetCombinedRankInfo") {
            self.rank_info = Some(RankInfo::new(internal_parse(data, 2)))
        }

        if data.contains("<== Event_GetCoursesV2") {
            self.event_courses = Some(EventCourses::new(internal_parse(data, 2)))
        }

        if data.contains("<== Deck_GetDeck") {
            self.decks = Some(Decks::new(internal_parse(data, 2)))
        }
    }
}

fn internal_parse(data: &str, line: usize) -> &str {
    data
    .trim()
    .lines()
    .nth(line)
    .unwrap()
}
