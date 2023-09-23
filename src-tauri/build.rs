fn main() {
    // TODO this doesn't do anything because if remote-static is gitignore'd,
    // then it doesn't get tracked for changes even if explicitly mentioned.
    // Right now the process is to `npm run build:watch` from the remote project
    // and manually trigger a recompile from the Tauri project
    // println!("cargo:rerun-if-changed=remote-static");

    tauri_build::build()
}
