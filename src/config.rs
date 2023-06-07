use std::collections::HashMap;

use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug, Clone)]
pub struct DataProviders {
    #[serde(rename = "data-provider")]
    pub data_provider: HashMap<Url, DataProvider>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all="kebab-case")]
pub struct DataProvider {
    pub name: String,
    #[serde(default)]
    pub white_list: Vec<String>,
    #[serde(default)]
    pub black_list: Vec<String>,
}
