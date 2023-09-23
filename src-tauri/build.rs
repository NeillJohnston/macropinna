use std::process::Output;

fn ensure_command(output: Output, task: &str) {
    use std::io::{Write, stdout, stderr};

    stdout().write_all(&output.stdout).unwrap();
    stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success(), "Failed: {}", task);
}

fn main() {
    use std::process::Command;

    println!("cargo:rerun-if-changed=../src-rc/");

    let output = Command::new("npm")
        .current_dir("../src-rc")
        .args(["run", "build"])
        .output()
        .unwrap();
    ensure_command(output, "build rc");

    // TODO i could probably just change the build location and make this all
    // a single command
    let output = Command::new("rm")
        .args(["-r", "./remote-static"])
        .output()
        .unwrap();
    ensure_command(output, "remove old remote-static files");

    let output = Command::new("cp")
        .args(["-r", "../src-rc/build", "./remote-static"])
        .output()
        .unwrap();
    ensure_command(output, "copy new remote-static files");

    tauri_build::build()
}
