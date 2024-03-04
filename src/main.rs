mod cpu;
mod memory;
mod utils;

use crate::cpu::CpuReading;
use crate::utils::run_command;
use tokio::join;
use tokio::time::sleep;
use tokio::time::Duration;

async fn monitor_cpu_usage() {
    let cpu_cores = run_command("cat /proc/cpuinfo | grep processor | wc -l")
        .await
        .trim()
        .parse::<usize>()
        .unwrap();
    let ticks_per_second = run_command("getconf CLK_TCK")
        .await
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut last_reading = CpuReading::create().await;

    loop {
        sleep(Duration::from_secs(5)).await;

        let curr_reading = CpuReading::create().await;

        let wall_seconds_used = curr_reading.uptime - last_reading.uptime;
        let cpu_ticks_used = curr_reading.cpu_ticks_used_sum - last_reading.cpu_ticks_used_sum;

        last_reading = curr_reading;

        let wall_ticks_passed = ticks_per_second as f64 * wall_seconds_used;
        let cpu_ticks_available = wall_ticks_passed * cpu_cores as f64;
        println!("{}", (cpu_ticks_used as f64 / cpu_ticks_available) * 100.0);
    }
}

async fn monitor_memory_usage() {
    loop {
        let free_output = run_command("free -w").await;
        let free_output_struct = memory::FreeOutput::from_free_command(free_output);
        println!("{:?}", free_output_struct);
        sleep(Duration::from_secs(5)).await;
    }
}

#[tokio::main]
async fn main() {
    join!(monitor_cpu_usage(), monitor_memory_usage());
}
