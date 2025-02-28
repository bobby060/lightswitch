use aws_sdk_ec2::Region;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct LightswitchConfig {
    profile: String,
    region: Region,
    keys: HashMap<String, String>,
}

impl LightswitchConfig {
    fn new() -> Self {
        LightswitchConfig {
            profile: "default".to_string(),
            region: "us-east-2".to_string(),
            keys: HashMap::new(),
        }
    }

    fn save(&self) -> Result<(), Error> {
        let config_file = std::fs::File::create("lightswitch.json").unwrap();
    }
}
