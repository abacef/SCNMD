use crate::utils::run_command;
use tokio::fs;

pub struct CpuReading {
    pub cpu_ticks_used_sum: usize,
    pub uptime: f64,
}
impl CpuReading {
    pub async fn create() -> CpuReading {
        let stat_str = run_command("cat /proc/stat").await;
        let curr_uptime = Self::get_uptime().await;

        CpuReading {
            cpu_ticks_used_sum: Self::sum_cpu_usage_from_stat(stat_str),
            uptime: curr_uptime,
        }
    }

    pub fn get_cpu_usage_between(
        &self,
        last_reading: CpuReading,
        ticks_per_second: usize,
        cpu_cores: usize,
    ) -> f64 {
        let wall_seconds_used = self.uptime - last_reading.uptime;
        let cpu_ticks_used = self.cpu_ticks_used_sum - last_reading.cpu_ticks_used_sum;

        let wall_ticks_passed = ticks_per_second as f64 * wall_seconds_used;
        let cpu_ticks_available = wall_ticks_passed * cpu_cores as f64;

        (cpu_ticks_used as f64 / cpu_ticks_available) * 100.0
    }

    fn sum_cpu_usage_from_stat(stat_str: String) -> usize {
        let mut stat_output = stat_str.lines().next().unwrap().split_whitespace();

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

    async fn get_uptime() -> f64 {
        fs::read_to_string("/proc/uptime")
            .await
            .unwrap()
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .parse::<f64>()
            .unwrap()
    }
}
