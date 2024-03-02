mod memory;

use std::process::Command;
use std::{thread, time};
use futures::future;
use tokio::join;

fn run_command(command: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");
    String::from_utf8(output.stdout).expect("Invalid UTF-8")
}

fn uptime() -> f64 {
    run_command("cat /proc/uptime")
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

fn sum_cpu_usage_from_stat(stat_str: String) -> usize {
    let mut stat_output = stat_str
        .lines()
        .next()
        .unwrap()
        .split_whitespace();

    // Skip the column that just says "cpu"
    stat_output.next();

    let usr = stat_output.next().unwrap().parse::<usize>().unwrap();
    let nice = stat_output.next().unwrap().parse::<usize>().unwrap();
    let sys = stat_output.next().unwrap().parse::<usize>().unwrap();

    // Skip idle and IOWait time
    stat_output.next();
    stat_output.next();

    let hirq = stat_output.next().unwrap().parse::<usize>().unwrap();
    let sirq = stat_output.next().unwrap().parse::<usize>().unwrap();

    usr + nice + sys + hirq + sirq
}

async fn monitor_cpu_usage() {
    let cpu_cores = run_command("cat /proc/cpuinfo | grep processor | wc -l")
        .trim()
        .parse::<usize>()
        .unwrap();
    let ticks_per_second = run_command("getconf CLK_TCK")
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut prev_sum = None;
    let mut prev_uptime = None;

    loop {
        let stat_str = run_command("cat /proc/stat");
        let curr_uptime = uptime();

        let cnt_sum = sum_cpu_usage_from_stat(stat_str);

        if prev_sum.is_none() {
            prev_sum = Some(cnt_sum);
            prev_uptime = Some(curr_uptime);
            tokio::time::sleep(time::Duration::from_secs(5)).await;
            continue;
        }

        let wall_seconds_used = curr_uptime - prev_uptime.unwrap();
        prev_uptime = Some(curr_uptime);

        let cpu_ticks_used = cnt_sum - prev_sum.unwrap();
        prev_sum = Some(cnt_sum);

        let wall_ticks_passed = ticks_per_second as f64 * wall_seconds_used;
        let cpu_ticks_available = wall_ticks_passed * cpu_cores as f64;
        println!("{}", (cpu_ticks_used as f64 / cpu_ticks_available) * 100.0);
        tokio::time::sleep(time::Duration::from_secs(5)).await;
    }
}

async fn monitor_memory_usage() {
    loop {
        let free_output = run_command("free -w");
        let free_output_struct = memory::FreeOutput::from_free_command(free_output);
        println!("{:?}", free_output_struct);
        tokio::time::sleep(time::Duration::from_secs(5)).await;
    }
}

#[tokio::main]
async fn main() {
    join!(monitor_cpu_usage(), monitor_memory_usage());
}