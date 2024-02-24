from subprocess import run

from time import sleep

def run_command(command):
    p = run(command, capture_output=True, shell=True)
    return p.stdout.decode()

def uptime():
    return float(run_command("cat /proc/uptime").split()[0])

cpu_cores = int(run_command("cat /proc/cpuinfo | grep processor | wc -l"))
ticks_per_second = int(run_command("getconf CLK_TCK"))

prev_sum = None
prev_uptime = None
while True:
    stat_output = run_command("cat /proc/stat").split("\n")[0].split()[1:]
    curr_uptime = uptime()

    usr = int(stat_output[0])
    nice = int(stat_output[1])
    sys = int(stat_output[2])
    cnt_sum = usr + nice + sys

    if prev_sum is None:
        prev_sum = cnt_sum
        prev_uptime = curr_uptime
        sleep(1)
        continue

    uptime_diff = curr_uptime - prev_uptime
    prev_uptime = curr_uptime

    clock_ticks_used = cnt_sum - prev_sum
    prev_sum = cnt_sum
    clock_ticks_passed = ticks_per_second * uptime_diff
    cpu_ticks_available = clock_ticks_passed * cpu_cores
    print((clock_ticks_used / cpu_ticks_available) * 100)
    sleep(5)


