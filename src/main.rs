mod cli_parser;
mod lightswitch;

use cli_parser::{CliOption, CliOptions, CliParser, Command, CommandType};
use lightswitch::config::LightswitchConfig;
use lightswitch::Ec2Controller;
const HELP: &str = "Usage: lightswitch <command> [<instance_id>| -n <name>]

Commands:
    list: List all instance

    start <instance_id> | -n <name>: Start an instance
    stop <instance_id> | -n <name>: Stop an instance";

#[tokio::main]
async fn main() {
    let mut config: Option<LightswitchConfig> = LightswitchConfig::load().ok();

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
            println!("{}", controller.list_instances().await.unwrap());
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
        .add_option(CliOption::new(CommandType::List, "-n", "--name"))
        .add_option(CliOption::new(CommandType::Start, "-n", "--name"))
        .add_option(CliOption::new(CommandType::Start, "-i", "--index"))
        .add_option(CliOption::new(CommandType::Stop, "-n", "--name"))
        .add_option(CliOption::new(CommandType::Stop, "-i", "--index"));

    CliParser::new(options)
}
