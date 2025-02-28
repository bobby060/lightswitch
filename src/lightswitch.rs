use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_ec2::client::Waiters;
use aws_sdk_ec2::{
    config::{Config, Region},
    Client,
};

use aws_sdk_ec2::types::Instance;
use aws_sdk_ec2::types::Tag;
use aws_sdk_ec2::Error;
use std::io::{Error as IoError, ErrorKind};
use std::time::Duration;
pub struct Ec2Controller {
    config: SdkConfig,
}

impl Ec2Controller {
    pub async fn new() -> Self {
        Ec2Controller {
            config: aws_config::from_env()
                .region(Region::new("us-east-2"))
                .load()
                .await,
        }
    }

    pub async fn list_instances(&self) -> Result<Vec<(Option<String>, String, String)>, Error> {
        let client = Client::new(&self.config);
        let response = client.describe_instances().send().await?;
        Ok(Vec::from_iter(
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
        ))
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

    pub async fn start_instance(&self, args: &[String]) -> Result<String, Error> {
        let instance_id = if args.len() == 2 && args[0] == "-n" {
            self.name_to_id(&args[1]).await?
        } else {
            args[0].clone()
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

    pub async fn stop_instance(&self, args: &[String]) -> Result<(), Error> {
        let instance_id = if args.len() == 2 && args[0] == "-n" {
            self.name_to_id(&args[1]).await?
        } else {
            args[0].to_string()
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
