use bdhandler;
use user;

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
    let new_users = bd.get_all_users().await;
    for user in new_users {
        println!("{}", user.to_string());
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
