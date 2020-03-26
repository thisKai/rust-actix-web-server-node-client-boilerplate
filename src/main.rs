mod client;

use {
    actix_web::{App, HttpServer},
    std::env,
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = server_port();

    println!("Starting server, listening on 0.0.0.0:{}", port);

    HttpServer::new(move || App::new().service(client::static_files()))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}

fn server_port() -> u16 {
    env::var("PORT")
        .map(|port| port.parse().expect("PORT must be a number"))
        .unwrap_or(3000)
}
