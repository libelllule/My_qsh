use std::env;
use std::fmt;

#[derive (Clone, Default, Debug)]

pub struct Config {
    port: Option<u16>,
    help: Option<bool>,
    debug: Option<bool>,
}

#[derive(Debug)]

pub enum ParseError {
    MissingValue(String),
    InvalidValue(String, String),
    UnknownOption(String),
    AlreadySpecified(String),
    UnexpectedArgument(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::MissingValue(opt) => write!(f, "Missing value for option: {}", opt),
            ParseError::InvalidValue(opt, val) => write!(f, "Invalid value '{}' for option: {}", val, opt),
            ParseError::UnknownOption(opt) => write!(f, "Unknown option: {}", opt),
            ParseError::AlreadySpecified(opt) => write!(f, "Option '{}' already specified", opt),
            ParseError::UnexpectedArgument(arg) => write!(f, "Unexpected argument: {}", arg),
        }
    }
}

pub struct ParamHandler {
    config: Config,
}

impl ParamHandler {

    fn new() -> Self {
        ParamHandler {
            config: Config::default(),
        }
    }
    pub fn parse_args(&mut self, params: &[String]) -> Result<Config, ParseError> {
        let mut i = 1; // Skip the program name

        while i < params.len() {
            let arg = &params[i];

            if arg.starts_with('-') {
                match arg.as_str() {
                    "--port" | "-port" => {
                        if i + 1 >= params.len() {
                            return Err(ParseError::MissingValue(arg.clone()));
                        }
                        self.parse_port(&params[i + 1])?;
                        i += 2;
                    }
                    "--help" | "-help" | "-h" => {
                        self.config.help = Some(true);
                        i += 1;
                    }
                    "--debug" | "-debug" | "-d" => {
                        self.config.debug = Some(true);
                        i += 1;
                    }
                    _ => return Err(ParseError::UnknownOption(arg.clone())),
                }
            } else {
                return Err(ParseError::UnexpectedArgument(arg.clone()));
            }
        }

        Ok(self.config.clone())
    }

    fn parse_port(&mut self, p: &str) -> Result<(), ParseError> {
        if self.config.port.is_some() {
            return Err(ParseError::AlreadySpecified("--port".to_string()));
        }

        match p.parse::<u16>() {
            Ok(number) => {
                self.config.port = Some(number);
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue("--port".to_string(), p.to_string())),
        }
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }
}    

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut handler = ParamHandler::new();

    match handler.parse_args(&args) {
        Ok(config) => {
            if config.help.is_some() {
                println!("Usage: program [options]");
                println!("Options:");
                println!("  --port <number>  Set the port");
                println!("  --help           Show this help message");
                println!("  --debug          Enable debug mode");
                return;
            }
            println!("Configuration: {:?}", config);
        }
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            std::process::exit(1);
        }
    }
}

