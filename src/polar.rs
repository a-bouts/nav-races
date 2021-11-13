use serde::{Deserialize, Serialize};

use log::{error, info, warn};
use reqwest::StatusCode;

use crate::config::ServiceConfig;

pub(crate) struct PolarService {
    polars: ServiceConfig,
}

impl PolarService {

    pub(crate) fn new(polars: ServiceConfig) -> Self {
        PolarService { polars }
    }

    pub(crate) async fn get_boat(&self, polar_id: u8) -> Option<String> {
        match reqwest::get(format!("{}?polar_id={}", self.polars.url, polar_id)).await {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    match response.json::<Polar>().await {
                        Ok(polar) => {
                            info!("Found Polar {} : '{}'", polar_id, polar.id.clone().unwrap_or_default());
                            polar.id
                        },
                        Err(e) => {
                            error!("Error deserializing polar {} : {}", polar_id, e);
                            None
                        }
                    }
                } else {
                    warn!("Polar '{}' not found : {}", polar_id, response.status());
                    None
                }
            },
            Err(e) => {
                error!("Error getting polar {} : {}", polar_id, e);
                None
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Polar {
    pub(crate) id: Option<String>,
}
