use directories::ProjectDirs;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PROJECT_DIRS: ProjectDirs = {
        ProjectDirs::from(
            "",
            "Macropinna",
            "Macropinna"
        ).unwrap()
    };
}

/// Ensure project directories are initialized.
pub fn ensure() {
    use std::fs;

    if let Err(err) = fs::create_dir_all(PROJECT_DIRS.config_dir()) {
        log::error!("{}", err);
        panic!();
    }

    if let Err(err) = fs::create_dir_all(PROJECT_DIRS.data_dir()) {
        log::error!("{}", err);
        panic!();
    }
}