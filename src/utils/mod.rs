use tokio::process::Command;

pub async fn run_command(command: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .await
        .unwrap();
    String::from_utf8(output.stdout).expect("Invalid UTF-8")
}
