mod client;

use {
    actix_web::{App, HttpServer},
    listenfd::ListenFd,
    std::env,
};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    actix_rt::spawn(client::dev_tools());

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || App::new().service(client::static_files()));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => {
            let port = listener.local_addr()?.port();
            println!("Restarting server on 0.0.0.0:{}", port);
            server.listen(listener)?
        },
        None => {
            let port = server_port();
            println!("Starting server on 0.0.0.0:{}", port);
            server.bind(("0.0.0.0", port))?
        },
    };

    server.run().await
}

fn server_port() -> u16 {
    env::var("PORT")
        .map(|port| port.parse().expect("PORT must be a number"))
        .unwrap_or(3000)
}
