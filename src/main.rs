use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    name: String,
    description: String,
    price: f64,
    stock: u8,
}

#[derive(Serialize)]
struct ResponseObj {
    status: String,
    message: String,
    product: Product,
}

#[post("/v1/products")]
async fn create_product(product: web::Json<Product>) -> Result<impl Responder, actix_web::Error> {
    let response = ResponseObj {
        status: "success".to_string(),
        message: "product created".to_string(),
        product: product.into_inner(),
    };
    Ok(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(create_product))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
