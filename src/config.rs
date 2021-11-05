use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub(crate) races_dir: String,
    pub(crate) archived_dir: String
}
