mod lightswitch;

use lightswitch::Ec2Controller;

const HELP: &str = "Usage: lightswitch <command> [<instance_id>| -n <name>]

Commands:
    list: List all instance

    start <instance_id> | -n <name>: Start an instance
    stop <instance_id> | -n <name>: Stop an instance";

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();

    match command.as_str() {
        "list" => {
            let controller = Ec2Controller::new().await;
            let instances = controller.list_instances().await.unwrap();
            println!("Instances:");
            for instance in instances {
                println!("{}\t{}", instance.0.unwrap_or("".to_string()), instance.1);
            }
        }
        "start" => {
            let controller = Ec2Controller::new().await;
            let dns = controller.start_instance(&args[2..]).await.unwrap();
            println!("New dns: {:?}", dns);
        }
        "stop" => {
            let controller = Ec2Controller::new().await;
            controller.stop_instance(&args[2..]).await.unwrap();
        }
        _ => {
            println!("{}", HELP);
        }
    }
}
