# lightswitch
A command line utility for starting/stopping Ec2 instances

## Usage
```
lightswitch
    list: List all instance

    start [-i <instance_id> | -n <name>]: Start an instance
    stop [-i <instance_id> | -n <name>]: Stop an instance

    configure: set the aws region

```


Compile
```
cargo build --release
```

Run
```
cargo run -- --help
