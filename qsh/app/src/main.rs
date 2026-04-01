<<<<<<< qsh/app/src/main.rs
use std::fs;
use security;
use logger;
use logger::{logger_info, logger_debug, logger_warn, logger_error};


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
    match fs::write("s/privkey.pem", "ahahahahahahahahahha"){
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

    logger::Logger::init_logs("debug"); 
    let qsh_logger = logger::Logger::new();
    
    logger_info!(qsh_logger, "this is info level and you can use formatter here {}", 5);
    logger_debug!(qsh_logger, "this is debug level and you can use formatter here {}", 4);
    logger_warn!(qsh_logger, "this is warn level and you can use formatter here {}", 3);
    logger_error!(qsh_logger, "this is error level and you can use formatter here {}", 2);
}