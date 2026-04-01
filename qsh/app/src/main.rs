use logger;
use logger::{logger_info, logger_debug, logger_warn, logger_error};
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
        
    logger::Logger::init_logs("debug"); 
    let qsh_logger = logger::Logger::new();
    
    logger_info!(qsh_logger, "this is info level and you can use formatter here {}", 5);
    logger_debug!(qsh_logger, "this is debug level and you can use formatter here {}", 4);
    logger_warn!(qsh_logger, "this is warn level and you can use formatter here {}", 3);
    logger_error!(qsh_logger, "this is error level and you can use formatter here {}", 2);
}