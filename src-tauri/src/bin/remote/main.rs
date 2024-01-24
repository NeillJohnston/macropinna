mod config;
mod input;
mod server;

#[tokio::main]
async fn main() {
    // TODO load from file
    let config = config::Config {
        port: 5174,
        port_internal: 51740
    };

    server::run(config).await;
}