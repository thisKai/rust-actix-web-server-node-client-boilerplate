use {
    actix_web::{App, HttpServer},
    actix_web_static_files::ResourceFiles,
    std::{collections::HashMap, env},
};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let generated = generate();
        App::new().service(ResourceFiles::new("/", generated))
    })
    .bind(("0.0.0.0", server_port()))?
    .run()
    .await
}

fn server_port() -> u16 {
    env::var("PORT")
        .map(|port| port.parse().expect("PORT must be a number"))
        .unwrap_or(3000)
}
