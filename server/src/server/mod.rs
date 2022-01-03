use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use std::io;

mod generate_one_service;
mod info_service;

pub async fn start(port: u16) -> io::Result<()> {
    println!("Starting on port {}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(info_service::get_info)
            .service(generate_one_service::generate_one)
            .default_service(
                web::resource("")
                    .route(web::route())
                    .to(HttpResponse::MethodNotAllowed),
            )
        // .service(generate_one)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
