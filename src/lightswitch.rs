pub mod config;

use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_ec2::client::Waiters;
use aws_sdk_ec2::{
    config::{Config, Region},
    Client,
};

use aws_sdk_ec2::types::Instance;
use aws_sdk_ec2::types::Tag;
use aws_sdk_ec2::Error;
use std::collections::HashMap;
use std::io::{Error as IoError, ErrorKind};
use std::time::Duration;

use config::LightswitchConfig;

pub struct Ec2Controller {
    config: SdkConfig,
}

impl Ec2Controller {
    /// Configure the controller by setting the AWS region
    pub async fn configure(&self) -> Result<(), Error> {
        let client = Client::new(&self.config);
        let response = client.describe_regions().send().await?;
        let regions = response.regions();

        let mut index = 0;
        for region in regions {
            println!("{}: {}", index, region.region_name().unwrap());
            index += 1;
        }

        println!("Enter the number corresponding to the region you want to use:");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let index = input.trim().parse::<usize>().unwrap();
        let region = regions[index].region_name().unwrap().to_string();

        let config = LightswitchConfig::new(&region);
        config.save().unwrap();

        println!("Region set to: {}", region);

        Ok(())
    }

    pub async fn new(region: &str) -> Self {
        Ec2Controller {
            config: aws_config::from_env()
                .region(Region::new(region.to_string()))
                .load()
                .await,
        }
    }

    pub async fn list_instances(&self) -> Result<String, Error> {
        let client = Client::new(&self.config);
        let response = client.describe_instances().send().await?;
        let instances = Vec::from_iter(
            response
                .reservations()
                .into_iter()
                .flat_map(|r| r.instances().into_iter())
                .cloned()
                .map(|i| {
                    (
                        i.tags()
                            .iter()
                            .find(|t| t.key().unwrap() == "Name")
                            .map(|t| t.value().unwrap().to_string()),
                        i.instance_id().unwrap().to_string(),
                        i.state().unwrap().name().unwrap().to_string(),
                    )
                }),
        );

        // Build the output string
        let mut output = String::new();

        let max_name_len = instances
            .iter()
            .map(|i| i.0.as_ref().unwrap_or(&String::from("")).len())
            .max()
            .unwrap_or(4);

        let len_padded_string = |s: &str, len: usize| -> String {
            let padding = len - s.len();
            let mut output = String::new();
            output += &" ".repeat(padding / 2);
            output += s;
            output += &" ".repeat(padding / 2 + padding % 2);
            output
        };

        let len_aws_id = 20;
        let len_state = 10;

        output += "Current Instances:\n";
        output += &format!(
            "|{}|{}|{}|\n",
            len_padded_string("Name", max_name_len),
            len_padded_string("ID", len_aws_id),
            len_padded_string("State", len_state)
        );
        output += &"-".repeat(max_name_len + len_aws_id + len_state);
        output += "\n";

        for instance in instances {
            output += &format!(
                "|{}|{}|{}|\n",
                len_padded_string(&instance.0.unwrap_or("".to_string()), max_name_len),
                len_padded_string(&instance.1, len_aws_id),
                len_padded_string(&instance.2, len_state)
            );
        }

        Ok(output)
    }

    /// Convert a name to an instance ID if that name exists as a tag on an instance
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the instance to convert
    ///
    /// # Returns
    ///
    /// The instance ID of the instance with the given name
    async fn name_to_id(&self, name: &str) -> Result<String, Error> {
        let client = Client::new(&self.config);
        let response = client.describe_instances().send().await?;
        Ok(response
            .reservations()
            .into_iter()
            .flat_map(|r| r.instances())
            .find(|i| {
                i.tags()
                    .iter()
                    .find(|t| t.key().unwrap() == "Name")
                    .map(|t| t.value().unwrap().to_string())
                    == Some(name.to_string())
            })
            .unwrap()
            .instance_id()
            .unwrap()
            .to_string())
    }

    pub async fn start_instance(&self, options: &HashMap<String, String>) -> Result<String, Error> {
        let instance_id = if options.contains_key("-n") {
            self.name_to_id(&options["-n"]).await?
        } else {
            options["-i"].clone()
        };

        let client = Client::new(&self.config);
        client
            .start_instances()
            .instance_ids(&instance_id)
            .send()
            .await?;

        println!("Waiting for instance to start...");

        client
            .wait_until_instance_running()
            .instance_ids(&instance_id)
            .wait(Duration::from_secs(60))
            .await?;

        let output = client
            .describe_instances()
            .instance_ids(instance_id)
            .send()
            .await?;

        Ok(output
            .reservations()
            .into_iter()
            .flat_map(|r| r.instances())
            .next()
            .unwrap()
            .public_dns_name()
            .unwrap()
            .to_string())
    }

    pub async fn stop_instance(&self, options: &HashMap<String, String>) -> Result<(), Error> {
        let instance_id = if options.contains_key("-n") {
            self.name_to_id(&options["-n"]).await?
        } else {
            options["-i"].clone()
        };

        let client = Client::new(&self.config);
        client
            .stop_instances()
            .instance_ids(&instance_id)
            .send()
            .await?;

        println!("Instance stopping...");

        Ok(())
    }
}
