use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Config {
    pub steamgrid_api_key: Option<String>, 
}