mod controller;

use actix_web::{web, App, HttpServer};
use controller::product::create_product;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/v1/products", web::post().to(create_product)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
