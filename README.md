# lightswitch
A command line utility for starting/stopping Ec2 instances





## Usage

Requires that AWS CLI is installed and configured.

See [here](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) for instructions.


```
lightswitch
    list: List all instance

    start [-i <instance_id> | -n <name>]: Start an instance
    stop [-i <instance_id> | -n <name>]: Stop an instance

    configure: set the aws region

```


Build yourself
```
cargo build --release
```

After that you can run via `.'lightswitch`

Run
```
cargo run -- --help


## Roadmap:
- [ ] Change location config file to be user-configurable
- [ ] Set default config location based on OS
- [ ] Allow access via IAM
- [ ] Add support for GCP, Azure

