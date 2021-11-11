use std::collections::HashMap;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, SubsecRound, Utc};

use serde::{Deserialize, Serialize};
use crate::race;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Leg {
    #[serde(rename = "_id")]
    id: RaceId,
    #[serde(rename = "lastUpdate", with = "ts_milliseconds")]
    last_update: DateTime<Utc>,
    name: String,
    #[serde(rename = "displayOrder")]
    display_order: u8,
    #[serde(rename = "priceLevel")]
    price_level: u8,
    #[serde(rename = "freeCredits")]
    free_credits: u16,
    #[serde(rename = "optionPrices")]
    option_prices: HashMap<String, u16>,
    #[serde(rename = "pilotBoatCredits")]
    pilot_boat_credits: Option<u16>,
    status: Status,
    #[serde(rename = "estimatedTime")]
    estimated_time: u8,
    #[serde(rename = "estimatedLength")]
    estimated_length: u16,
    schedule: Schedule,
    open: Date,
    close: Date,
    start: Start,
    end: End,
    course: Vec<LatLon>,
    checkpoints: Vec<Checkpoint>,
    ice_limits: Limits,
    #[serde(rename = "defaultMapPreset")]
    default_map_preset: String,
    #[serde(rename = "mapPresets")]
    map_presets: Vec<String>,
    race: Race,
    boat: Boat,
    #[serde(rename = "syncAWS")]
    sync_aws: String,
    #[serde(rename = "specialIcons")]
    special_icons: Option<SpecialIcons>,
    #[serde(rename = "vsrLevel")]
    vsr_level: u8,
    #[serde(rename = "hasCode")]
    has_code: Option<bool>,
    #[serde(rename = "sponsorLogo")]
    sponsor_logo: Option<String>,
    #[serde(rename = "sponsorURL")]
    sponsor_url: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct RaceId {
    race_id: u16,
    num: Option<u8>,
}

impl Into<String> for RaceId {
    fn into(self) -> String {
        format!("{}{}", self.race_id.to_string(), self.num.map_or(String::from(""), |num| format!(".{}", num.to_string())))
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Status {
    Opened,
    Started
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Schedule {
    Validated,
    DontShow
}

#[derive(Deserialize, Serialize, Debug)]
struct Date {
    #[serde(with = "ts_milliseconds")]
    date: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Start {
    lat: f64,
    lon: f64,
    name: String,
    #[serde(with = "ts_milliseconds")]
    date: DateTime<Utc>,
    heading: u16,
    #[serde(rename = "countryCode")]
    country_code: String,
    #[serde(rename = "countryFlag")]
    country_flag: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct End {
    lat: f64,
    lon: f64,
    name: String,
    #[serde(with = "ts_milliseconds")]
    date: DateTime<Utc>,
    radius: u8,
    #[serde(rename = "countryCode")]
    country_code: String,
    #[serde(rename = "countryFlag")]
    country_flag: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct LatLon {
    lat: f64,
    lon: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct Checkpoint {
    id: u8,
    group: u8,
    name: String,
    start: LatLon,
    end: LatLon,
    engine: bool,
    display: Display,
    side: Side,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Display {
    None,
    Buoy,
    Gate,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Side {
    Stbd,
    Port
}

#[derive(Deserialize, Serialize, Debug)]
struct Limits {
    north: Vec<LatLon>,
    south: Vec<LatLon>,
    #[serde(rename = "maxLat")]
    max_lat: f64,
    #[serde(rename = "minLat")]
    min_lat: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct Race {
    name: String,
    #[serde(rename = "type")]
    typ: String,
    #[serde(rename = "realRaceTag")]
    real_race_tag: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct Boat {
    name: String,
    label: String,
    polar_id: u8,
    #[serde(rename = "assetBundle")]
    asset_bundle: String,
    stats: HashMap<String, f64>,
    #[serde(rename = "type")]
    typ: String,
    #[serde(rename = "defaultSkin")]
    default_skin: String
}

#[derive(Deserialize, Serialize, Debug)]
struct SpecialIcons {
    #[serde(rename = "raceLogo")]
    race_logo: bool,
    #[serde(rename = "raceLogoLink")]
    race_logo_link: String,
}

impl Into<race::LatLon> for LatLon {
    fn into(self) -> race::LatLon {
        race::LatLon {
            lat: self.lat,
            lon: self.lon
        }
    }
}

impl Into<race::LatLon> for Start {
    fn into(self) -> race::LatLon {
        race::LatLon {
            lat: self.lat,
            lon: self.lon
        }
    }
}

impl Into<race::LatLon> for End {
    fn into(self) -> race::LatLon {
        race::LatLon {
            lat: self.lat,
            lon: self.lon
        }
    }
}

impl Leg {
    fn clean_name(&self) -> String {
        self.race.name
            .to_lowercase()
            .chars()
            .filter(|c| c.is_digit(10) || c.is_ascii_alphabetic() || c.clone() == ' ')
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("-")
    }
}

impl Into<race::Race> for Leg {

    fn into(self) -> race::Race {
        let mut race = race::Race {
            id: Some(self.clean_name()),
            race_id: Some(self.id.into()),
            archived: false,
            name: self.race.name.clone(),
            short_name: Some(self.race.name),
            boat: "".to_string(),
            start_time: Some(self.start.date.round_subsecs(0)),
            end_time: Some(self.end.date.round_subsecs(0)),
            start: self.start.into(),
            waypoints: self.checkpoints.iter()
                .filter(|c| c.display != Display::None)
                .enumerate()
                .map(|(index, checkpoint)| {

                    let latlons = match checkpoint.side {
                        Side::Stbd => vec![
                            checkpoint.end.clone().into(),
                            checkpoint.start.clone().into(),
                        ],
                        _ => vec![
                            checkpoint.start.clone().into(),
                            checkpoint.end.clone().into(),
                        ]
                    };

                    race::Waypoint {
                        name: (index + 1).to_string(),
                        radius: None,
                        latlons: latlons,
                        to_avoid: None
                    }
                })
                .collect(),
            ice_limits: Some(race::Limits {
                north: self.ice_limits.north.iter().map(|latlon| latlon.clone().into()).collect(),
                south: self.ice_limits.south.iter().map(|latlon| latlon.clone().into()).collect(),
                max_lat: self.ice_limits.max_lat,
                min_lat: self.ice_limits.min_lat
            })
        };

        race.waypoints.push(race::Waypoint {
            name: "end".to_string(),
            radius: Some(self.end.radius),
            latlons: vec![self.end.into()],
            to_avoid: None
        });

        race
    }
}