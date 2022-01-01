use std::{env, io};

mod generators;
mod server;
mod util;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let mut port: u16 = 8080;

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        port = args[1].to_string().parse::<u16>().unwrap();
    }

    server::start(port).await
}
