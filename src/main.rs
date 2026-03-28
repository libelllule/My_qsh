mod logger;

fn main() {
    logger::Logger::init_logs("debug"); 
    let qsh_logger = logger::Logger::new();
    
    logger_info!(qsh_logger, "this is info level and you can use formatter here {}", 5);
    logger_debug!(qsh_logger, "this is debug level and you can use formatter here {}", 4);
    logger_warn!(qsh_logger, "this is warn level and you can use formatter here {}", 3);
    logger_error!(qsh_logger, "this is error level and you can use formatter here {}", 2);
}
