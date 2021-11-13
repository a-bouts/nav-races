use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    pub(crate) races_dir: String,
    pub(crate) archived_dir: String,
    pub(crate) polars: ServiceConfig,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct ServiceConfig {
    pub(crate) url: String
}