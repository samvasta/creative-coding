use actix_web::{get, HttpResponse, Result};

use crate::generators;

#[get("/info")]
pub async fn get_info() -> Result<HttpResponse> {
    let list = generators::generator_list();

    Ok(HttpResponse::Ok().json(list))
}
