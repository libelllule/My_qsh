use logger;
use logger::{logger_info, logger_debug, logger_warn, logger_error};
use std::env;
use param_handler;
use std::fs;
use security;

// Security section
fn create_and_validate(s: &security::Security) -> () {

    //Генерация сертификата и ключа 
    match s.generate_certs() {
        Ok(()) => println!("The certificate and key have been successfully created."),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
    // Проверка соответствия сертификата и ключа
    match s.validate_certs(){
        Ok(()) => println!("key and certificate match each other."),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_line() -> () {
    println!("-------------------------------------------------------------------------");
}

// Пример работы программы
fn main() {
    // Security section
    let s = security::Security::new(Some("s".to_string()));
    
    create_and_validate(&s);
    print_line();

    // Удаляем файл cert.pem
    match fs::remove_file("s/cert.pem") {
        Ok(()) => println!("File cert.pem deleted successfully"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
 
    // Проверка соответствия сертификата и ключа
    match s.validate_certs(){
        Ok(()) => println!("key and certificate match each other."),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    print_line();
    create_and_validate(&s);
    print_line();

    //Изменение содержания ключа
    match fs::write("s/privkey.pem", "-----BEGIN CERTIFICATE-----
MIIBZTCCAQqgAwIBAgIUa3FBRaGdFXo7cpFvuhQphPyTQT8wCgYIKoZIzj0EAwIw
ITEfMB0GA1UEAwwWcmNnZW4gc2VsZiBzaWduZWQgY2VydDAgFw03NTAxMDEwMDAw
MDBaGA80MDk2MDEwMTAwMDAwMFowITEfMB0GA1UEAwwWcmNnZW4gc2VsZiBzaWdu
ZWQgY2VydDBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABMOt694zj2P3FutT5937
APzxowXs8Y7d3fPX0jP+ACJMAZvfW8NOua+9D6/hTX7G/AkkK0h2pqUTtuyU31xu
NKijHjAcMBoGA1UdEQQTMBGCCWxvY2FsaG9zdIcEfwAAATAKBggqhkjOPQQDAgNJ
ADBGAiEAw+iHp/dEoBYjhjzWMAzyod+NJHXJXpAAMyraE3puaT8CIQCHbh8vEdoK
TOszU/aRJ5ljbnrY+cxJygCrU0EFyWrC1w==
-----END CERTIFICATE-----"){
        Ok(()) => println!("Contents of the file privkey.pem have been changed"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    // Проверка соответствия сертификата и ключа
    match s.validate_certs(){
        Ok(()) => println!("key and certificate match each other."),
        Err(e) => eprintln!("Error: {}", e),
    }
    print_line();

    // ParamHandler section
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

    // Logger section    
    logger::Logger::init_logs("debug"); 
    let qsh_logger = logger::Logger::new();
    
    logger_info!(qsh_logger, "this is info level and you can use formatter here {}", 5);
    logger_debug!(qsh_logger, "this is debug level and you can use formatter here {}", 4);
    logger_warn!(qsh_logger, "this is warn level and you can use formatter here {}", 3);
    logger_error!(qsh_logger, "this is error level and you can use formatter here {}", 2);
}
