use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
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

pub async fn create_product(product: web::Json<Product>) -> impl Responder {
    let response = ResponseObj {
        status: "success".to_string(),
        message: "product created".to_string(),
        product: product.into_inner(),
    };
    HttpResponse::Created().json(response)
}
