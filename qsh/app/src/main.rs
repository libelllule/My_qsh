use std::env;
use param_handler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut handler = param_handler::ParamHandler::new();

    match handler.parse_args(&args) {
        Ok(config) => {
            if config.help.is_some() {
                println!("Usage: program [options]");
                println!("Options:");
                println!("  --port <number>  Set the port");
                println!("  --help           Show this help message");
                println!("  --debug          Enable debug mode");
            }
            println!("Configuration: {:?}", config);
        }
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            std::process::exit(1);
        }
    }
}

