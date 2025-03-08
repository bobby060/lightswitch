mod cli_parser;
mod lightswitch;

use cli_parser::{CliOption, CliOptions, CliParser, Command, CommandType};
use lightswitch::config::LightswitchConfig;
use lightswitch::Ec2Controller;
const HELP: &str = "lightswitch usage:

list: List all instances
start [-i <instance_id> | -n <name> | --name <name> | --instance <instance_id>]: Start an instance
stop [-i <instance_id> | -n <name> | --name <name> | --instance <instance_id>]: Stop an instance
configure: Set the aws region
help: Show this help message";

#[tokio::main]
async fn main() {
    let mut config: Option<LightswitchConfig> = LightswitchConfig::load().ok();

    println!("Lightswitch is a simple CLI tool to start and stop EC2 instances.\n\n");

    // If no config is found, configure the controller
    if config.is_none() {
        println!("No region set, setting now...");
        let controller = Ec2Controller::new("us-east-2").await;
        controller.configure().await.unwrap();
        config = LightswitchConfig::load().ok();
    }

    let region = config.unwrap().get_region();

    let parser = build_parser();

    let command = parser.parse(std::env::args().collect());

    match command.as_ref().unwrap().command {
        CommandType::List => {
            let controller = Ec2Controller::new(&region).await;
            controller.list_instances(false).await.unwrap();
        }
        CommandType::Start => {
            let controller = Ec2Controller::new(&region).await;
            let dns = controller
                .start_instance(&command.unwrap().options)
                .await
                .unwrap();
            println!("New dns: {:?}", dns);
        }
        CommandType::Stop => {
            let controller = Ec2Controller::new(&region).await;
            controller
                .stop_instance(&command.unwrap().options)
                .await
                .unwrap();
        }
        CommandType::Configure => {
            let controller = Ec2Controller::new(&region).await;
            controller.configure().await.unwrap();
        }
        _ => {
            println!("{}", HELP);
        }
    }
}

fn build_parser() -> CliParser {
    let mut options = CliOptions::new();
    options
        .add_option(CliOption::new(CommandType::Start, "-n", "--name"))
        .add_option(CliOption::new(CommandType::Start, "-i", "--instance"))
        .add_option(CliOption::new(CommandType::Stop, "-n", "--name"))
        .add_option(CliOption::new(CommandType::Stop, "-i", "--instance"));

    CliParser::new(options)
}
