fn main() {
    std::process::Command::new("sh")
        .arg("-c")
        .arg("PS1='$ ' sh")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
