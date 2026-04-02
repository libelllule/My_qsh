use std::fs;
use security;

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
}      
