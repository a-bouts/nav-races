use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::race;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Race {
    pub(crate) id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) race_id: Option<String>,
    #[serde(default)]
    pub(crate) archived: bool,
    pub(crate) name: String,
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub(crate) short_name: Option<String>,
    pub(crate) boat: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) end_time: Option<DateTime<Utc>>,
    pub(crate) start: LatLon,
    pub(crate) waypoints: Vec<Waypoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ice_limits: Option<Limits>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct LatLon {
    pub(crate) lat: f64,
    pub(crate) lon: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Limits {
    pub(crate) north: Vec<LatLon>,
    pub(crate) south: Vec<LatLon>,
    #[serde(rename = "maxLat")]
    pub(crate) max_lat: f64,
    #[serde(rename = "minLat")]
    pub(crate) min_lat: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Waypoint {
    pub(crate) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) radius: Option<u8>,
    pub(crate) latlons: Vec<LatLon>,
    #[serde(rename = "toAvoid", skip_serializing_if = "Option::is_none")]
    pub(crate) to_avoid: Option<Vec<Vec<Vec<f64>>>>
}

impl From<race::Race> for Race {
    fn from(race: race::Race) -> Self {
        Race {
            id: race.id.expect("Race id is not null"),
            race_id: race.race_id,
            archived: race.archived,
            name: race.name,
            short_name: race.short_name,
            boat: race.boat,
            start_time: race.start_time,
            end_time: race.end_time,
            start: race.start.into(),
            waypoints: race.waypoints.into_iter().map(|w| w.into()).collect(),
            ice_limits: race.ice_limits.map(|x| x.into())
        }
    }
}

impl Into<race::Race> for Race {
    fn into(self) -> race::Race {
        race::Race {
            id: Some(self.id),
            race_id: self.race_id,
            archived: self.archived,
            name: self.name,
            short_name: self.short_name,
            boat: self.boat,
            start_time: self.start_time,
            end_time: self.end_time,
            start: self.start.into(),
            waypoints: self.waypoints.into_iter().map(|w| w.into()).collect(),
            ice_limits: self.ice_limits.map(|x| x.into())
        }
    }
}

impl From<race::LatLon> for LatLon {
    fn from(latlon: race::LatLon) -> Self {
        LatLon {
            lat: latlon.lat,
            lon: latlon.lon
        }
    }
}


impl Into<race::LatLon> for LatLon {
    fn into(self) -> race::LatLon {
        race::LatLon {
            lat: self.lat,
            lon: self.lon
        }
    }
}

impl From<race::Limits> for Limits {
    fn from(limits: race::Limits) -> Self {
        Limits {
            north: limits.north.into_iter().map(|x| x.into()).collect(),
            south: limits.south.into_iter().map(|x| x.into()).collect(),
            max_lat: limits.max_lat,
            min_lat: limits.min_lat
        }
    }
}

impl Into<race::Limits> for Limits {
    fn into(self) -> race::Limits {
        race::Limits {
            north: self.north.into_iter().map(|x| x.into()).collect(),
            south: self.south.into_iter().map(|x| x.into()).collect(),
            max_lat: self.max_lat,
            min_lat: self.min_lat
        }
    }
}

impl From<race::Waypoint> for Waypoint {
    fn from(waypoint: race::Waypoint) -> Self {
        Waypoint {
            name: waypoint.name,
            radius: waypoint.radius,
            latlons: waypoint.latlons.into_iter().map(|l| l.into()).collect(),
            to_avoid: waypoint.to_avoid
        }
    }
}

impl Into<race::Waypoint> for Waypoint {
    fn into(self) -> race::Waypoint {
        race::Waypoint {
            name: self.name,
            radius: self.radius,
            latlons: self.latlons.into_iter().map(|l| l.into()).collect(),
            to_avoid: self.to_avoid
        }
    }
}
