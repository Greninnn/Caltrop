use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct Config {
    steamgrid_api_key: Option<&str>, 
}