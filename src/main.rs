mod cpu;
mod memory;
mod utils;

use crate::cpu::CpuReading;
use crate::utils::{get_cpu_cores, get_cpu_tics_per_second, run_command};
use tokio::join;
use tokio::time::sleep;
use tokio::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct CpuUsed {
    cpu_used: f64,
}

async fn monitor_cpu_usage() {
    let ticks_per_second = get_cpu_tics_per_second().await;
    let cpu_cores = get_cpu_cores().await;

    let mut last_reading = CpuReading::create().await;
    loop {
        sleep(Duration::from_secs(5)).await;

        let curr_reading = CpuReading::create().await;
        let cpu_usage_between =
            curr_reading.get_cpu_usage_between(last_reading, ticks_per_second, cpu_cores);

        last_reading = curr_reading;

        println!("{}", cpu_usage_between);

        let cpu_usage_struct = CpuUsed {
            cpu_used: cpu_usage_between,
        };
        let json_string = serde_json::to_string(&cpu_usage_struct).unwrap();
        let client = reqwest::Client::new();
        let response = client
            .post("http://localhost:8000/metric_data/cpu_usage")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(json_string.to_owned())
            .send()
            .await;
        println!("Response status: {:?}", response);
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
