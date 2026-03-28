mod bd_handler;

#[tokio::main]
async fn main() {
    let _bd = bd_handler::BDHandler::new();
}
