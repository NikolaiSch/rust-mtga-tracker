use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RankInfo {
    #[serde(rename = "constructedSeasonOrdinal")]
    pub constructed_season_ordinal: i64,

    #[serde(rename = "constructedClass")]
    pub constructed_class: String,

    #[serde(rename = "constructedLevel")]
    pub constructed_level: i64,

    #[serde(rename = "constructedMatchesWon")]
    pub constructed_matches_won: i64,

    #[serde(rename = "constructedMatchesLost")]
    pub constructed_matches_lost: i64,

    #[serde(rename = "limitedSeasonOrdinal")]
    pub limited_season_ordinal: i64,

    #[serde(rename = "limitedClass")]
    pub limited_class: String,

    #[serde(rename = "limitedLevel")]
    pub limited_level: i64,

    #[serde(rename = "limitedStep")]
    pub limited_step: i64,

    #[serde(rename = "limitedMatchesWon")]
    pub limited_matches_won: i64,

    #[serde(rename = "limitedMatchesLost")]
    pub limited_matches_lost: i64,

    #[serde(rename = "constructedPercentile")]
    pub constructed_percentile: f64,
}

impl RankInfo {
    pub fn new(input: &str) -> RankInfo {
       serde_json::from_str::<RankInfo>(input).unwrap()
    }
}