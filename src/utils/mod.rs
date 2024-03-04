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

pub async fn get_cpu_tics_per_second() -> usize {
    run_command("getconf CLK_TCK")
        .await
        .trim()
        .parse::<usize>()
        .unwrap()
}

pub async fn get_cpu_cores() -> usize {
    run_command("cat /proc/cpuinfo | grep processor | wc -l")
        .await
        .trim()
        .parse::<usize>()
        .unwrap()
}
