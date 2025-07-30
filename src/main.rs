fn main() {
    std::process::Command::new("sh")
        .env("PS1", "$ ")
        .spawn()
        .expect("Failed to spawn shell")
        .wait()
        .expect("Failed to wait for shell's exit");
}
