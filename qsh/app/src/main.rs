use bdhandler;
use user;
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
#[tokio::main]
async fn main() {
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

    // BDHandler section
    let mut bd = bdhandler::BDHandler::new();
    let _ = bd.create_tables().await; // Check incorrect pool interaction
    match bd.initialize_db().await {
        Ok(_) => println!("Database initialized succesfully"),
        Err(err) => println!("Error while creating db: {}", err),
    };
    match bd.create_tables().await {
        Ok(_) => println!("Tables created succesfully"),
        Err(err) => println!("Error while creating tables: {}", err),
    };
    // Creating test users, the same will be using notes
    let users = [
        user::User::constructor(
            &0, 
            &"00:1A:2B:3C:4D:5E", 
            &"Xiaomi",
            &"Some_nickname",
            &"u"
        ).expect("REASON"),
        user::User::constructor(
            &1, 
            &"01:2B:3C:4D:5E:6F",
            &"Redmi",
            &"Another_nickname",
            &"t"
        ).expect("REASON"),
    ];
    match bd.insert_users(&users).await {
        Ok(_) => println!("Users were added"),
        Err(err) => println!("Error while inserting users: {}", err),
    }
    match bd.get_all_users().await {
        Ok(new_users) => {
            for user in new_users {
                println!("{}", user.to_string());
            }
        },
        Err(err) => eprintln!("Error while getting users: {}", err),
    }
    match bd.delete_user_by_id(0).await {
        Ok(_) => println!("User with id 0 deleted"),
        Err(err) => println!("Error while deleting user with id 0: {}", err),
    }
    match bd.clear_database().await {
        Ok(_) => println!("Table successfully cleared"),
        Err(err) => println!("Error while clearing table: {}", err),
    }
    bd.close_pool().await;

}
