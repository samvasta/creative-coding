use actix_web::{get, HttpResponse, Result};

use crate::generators;

#[get("/info")]
pub async fn get_info() -> Result<HttpResponse> {
    println!("hello");
    let list = generators::generator_list();

    println!("{:?}", list);
    Ok(HttpResponse::Ok().json(list))
}
