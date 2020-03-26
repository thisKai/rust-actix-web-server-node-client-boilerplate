use {
    std::env,
    actix_web::{App, HttpServer},
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
    })
    .bind(("0.0.0.0", server_port()))?
    .run()
    .await
}

fn server_port() -> u16 {
    env::var("PORT")
        .map(|port| {
            port.parse()
                .expect("PORT must be a number")
        })
        .unwrap_or(3000)
}
