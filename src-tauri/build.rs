fn main() {
    use std::{
        io::{Write, stdout, stderr},
        process::Command
    };

    let build_rc = Command::new("npm")
        .current_dir("../src-rc")
        .args(["run", "build"])
        .output()
        .unwrap();

    stdout().write_all(&build_rc.stdout).unwrap();
    stderr().write_all(&build_rc.stderr).unwrap();
    assert!(build_rc.status.success(), "Failed to build rc");

    let copy_rc = Command::new("cp")
        .args(["-r", "../src-rc/build", "./rc-build"])
        .output()
        .unwrap();

        stdout().write_all(&copy_rc.stdout).unwrap();
        stderr().write_all(&copy_rc.stderr).unwrap();
        assert!(build_rc.status.success(), "Failed to copy rc build");

    tauri_build::build()
}
