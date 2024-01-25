use shared::config::Config;

mod input;
mod server;

#[tokio::main]
async fn main() {
    // TODO load from file once
    let config: Config = todo!();

    server::run(config).await;
}