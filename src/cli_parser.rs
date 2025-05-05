use std::collections::{HashMap, HashSet};

pub struct CliParser {
    valid_options: CliOptions,
}

impl CliParser {
    pub fn new(valid_options: CliOptions) -> Self {
        Self { valid_options }
    }

    pub fn parse(&self, args: Vec<String>) -> Result<Command, String> {
        if args.len() == 1 {
            return Ok(Command::new(CommandType::Help, HashMap::new()));
        }

        let command = args[1].clone();

        let command = match command.as_str() {
            "list" => CommandType::List,
            "start" => CommandType::Start,
            "stop" => CommandType::Stop,
            "configure" => CommandType::Configure,
            _ => CommandType::Help,
        };

        let mut options_itr = args[2..].iter();

        let mut options = HashMap::new();

        while let Some(key) = options_itr.next() {
            self.valid_options.validate(key.clone(), &command)?;

            let value = &options_itr.next().unwrap();
            options.insert(key.to_string(), value.to_string());
        }

        // TOOD: validate options

        Ok(Command::new(command, options))
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum CommandType {
    List,
    Start,
    Stop,
    Configure,
    Help,
}

#[derive(Debug)]
pub struct Command {
    pub command: CommandType,
    pub options: HashMap<String, String>,
}

impl Command {
    fn new(command: CommandType, options: HashMap<String, String>) -> Self {
        Self { command, options }
    }
}

#[derive(Debug)]
pub struct CliOption {
    command: CommandType,
    short_name: String,
    long_name: String,
}

impl CliOption {
    pub fn new(command: CommandType, short_name: &str, long_name: &str) -> Self {
        Self {
            command,
            short_name: short_name.to_string(),
            long_name: long_name.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct CliOptions {
    valid_options: HashSet<(CommandType, String)>,
    short_to_long: HashMap<String, String>,
}

impl CliOptions {
    pub fn new() -> Self {
        Self {
            valid_options: HashSet::new(),
            short_to_long: HashMap::new(),
        }
    }

    pub fn add_option(&mut self, option: CliOption) -> &mut Self {
        let short_name = option.short_name.clone();
        let long_name = option.long_name.clone();
        self.valid_options
            .insert((option.command, long_name.clone()));
        self.short_to_long.insert(short_name, long_name);
        self
    }

    pub fn validate(&self, arg: String, command: &CommandType) -> Result<(), String> {
        let long_name = if arg.starts_with("--") {
            Some(arg.clone())
        } else {
            self.short_to_long
                .get(&arg)
                .map(|long_name| long_name.clone())
        };

        if long_name.is_none()
            || !self
                .valid_options
                .contains(&(command.clone(), long_name.unwrap()))
        {
            return Err(format!("Invalid option: {}", arg));
        }

        Ok(())
    }
}
