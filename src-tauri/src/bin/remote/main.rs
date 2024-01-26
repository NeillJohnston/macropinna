use shared::{
    config::load_from_path,
    util::project_dirs::PROJECT_DIRS
};

mod input;
mod server;

#[tokio::main]
async fn main() {
    let config = load_from_path(PROJECT_DIRS.config_dir().join("config.json")).unwrap();

    server::run(config).await;
}