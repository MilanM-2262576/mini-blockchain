pub mod block;
pub mod blockchain;
pub mod transaction;

use blockchain::Blockchain;
use transaction::Transaction;

use actix_web::{get, post, web, App, HttpServer, Responder};
use actix_cors::Cors;
use std::sync::Mutex;

#[get("/chain")]
async fn get_chain(data: web::Data<Mutex<Blockchain>>) -> impl Responder {
    let blockchain = data.lock().unwrap();
    web::Json(blockchain.chain.clone())
}
#[get("/is_valid")]
async fn is_valid(data: web::Data<Mutex<Blockchain>>) -> impl Responder {
    let mut blockchain = data.lock().unwrap();
    web::Json(blockchain.is_valid())
}

#[post("/add_block")]
async fn add_block(data: web::Data<Mutex<Blockchain>>, body: web::Json<Vec<Transaction>>) -> impl Responder {
    let mut blockchain = data.lock().unwrap();
    blockchain.add_block(body.into_inner());
    web::Json(blockchain.chain.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let blockchain = web::Data::new(Mutex::new({
        let mut bc = Blockchain::new();
        bc.add_block(vec![
            Transaction {
                sender: "Alice".to_string(),
                recipient: "Bob".to_string(),
                amount: 10,
            }
        ]);
        bc.add_block(vec![
            Transaction {
                sender: "Bob".to_string(),
                recipient: "Charlie".to_string(),
                amount: 5,
            }
        ]);
        bc
    }));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(blockchain.clone())
            .service(get_chain)
            .service(add_block)
            .service(is_valid)
    })
    .bind(("0.0.0.0", 8080))?  //.bind(("127.0.0.1", 8080))?
    .run()
    .await
}