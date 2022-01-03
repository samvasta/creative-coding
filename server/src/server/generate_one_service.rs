use std::collections::HashMap;

use actix_web::{dev::Body, post, web, HttpResponse};
use serde::Deserialize;
use serde_json::Value;

use crate::generators;

#[derive(Deserialize)]
pub struct GenerateOnePathParams {
    width: i32,
    height: i32,
    generator_type: String,
}

#[derive(Deserialize)]
pub struct GenerateOneQueryParams {
    seed: Option<u64>,
}

struct CompleteGenerateOneQueryParams {
    seed: u64,
}

/**
 Takes raw query params, which may have some optional values,
 and "fills in" the missing or invalid values.
*/
fn complete_query_params(raw_params: GenerateOneQueryParams) -> CompleteGenerateOneQueryParams {
    let realized_seed = match raw_params.seed {
        Some(s) => s,
        _ => fastrand::u64(..),
    };

    CompleteGenerateOneQueryParams {
        seed: realized_seed,
    }
}

#[post("/generate_one/{width}/{height}/{generator_type}")]
pub async fn generate_one(
    path: web::Path<GenerateOnePathParams>,
    query: web::Query<GenerateOneQueryParams>,
    dimensions_json: web::Json<HashMap<String, Value>>,
) -> HttpResponse {
    let query_params = complete_query_params(query.0);

    match generators::generate_one(generators::GenerateArgs {
        width: path.0.width,
        height: path.0.height,
        generator_type: path.0.generator_type,
        seed: query_params.seed,
        dimensions: dimensions_json.0,
    }) {
        Ok(bytes) => {
            return HttpResponse::Ok()
                .header("Content-Type", "image/png")
                .body(bytes)
        }
        Err(_) => {
            return HttpResponse::BadRequest().message_body(Body::from("Invalid generator type"))
        }
    }
}
