use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Error;

#[derive(Serialize, Deserialize)]
pub struct LightswitchConfig {
    profile: String,
    region: String,
    // keys: HashMap<String, String>,
}

impl LightswitchConfig {
    pub fn new(region: &str) -> Self {
        LightswitchConfig {
            profile: "default".to_string(),
            region: region.to_string(),
        }
    }

    pub fn load() -> Result<Self, Error> {
        let config_file = std::fs::File::open(dirs::home_dir().unwrap().join(".lightswitch.json"))?;
        let config: LightswitchConfig = serde_json::from_reader(config_file)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Error> {
        let config_file =
            std::fs::File::create(dirs::home_dir().unwrap().join(".lightswitch.json"))?;
        serde_json::to_writer(config_file, self)?;
        Ok(())
    }

    pub fn get_region(&self) -> String {
        self.region.clone()
    }
}
