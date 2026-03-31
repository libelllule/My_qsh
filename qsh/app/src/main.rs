use bdhandler;

#[tokio::main]
async fn main() {
    let mut bd = bdhandler::BDHandler::new().await;
    match bd.initialize_db().await {
        Ok(_) => println!("Database initialized succesfully"),
        Err(err) => println!("Error while creating db: {}", err),
    };
    match bd.create_tables().await {
        Ok(_) => println!("Tables created succesfully"),
        Err(err) => println!("Error while creating tables: {}", err),
    }
    bd.close_pool().await;
}
