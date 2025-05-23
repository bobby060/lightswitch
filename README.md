# lightswitch
A command line utility for starting/stopping Ec2 instances.

AWS CLI doesn't offer convenient ways to start/stop instances, so I created lightswitch to make it easier.





## Usage

Requires that AWS CLI is installed and configured.

See [here](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) for instructions.


```
aws-lightswitch
    list: List all instance

    start [-i <instance_id> | -n <name>]: Start an instance
    stop [-i <instance_id> | -n <name>]: Stop an instance

    configure: set the aws region

```
If you run stop or start with no options, lightswitch will list all instances in current region and let you pick which to start


Install from cargo
```
cargo install aws-lightswitch
```


Build yourself
```
cargo build --release
```

After that you can run via `./aws-lightswitch
`




