from subprocess import run

from time import sleep

def run_command(command):
    p = run(command, capture_output=True, shell=True)
    return p.stdout.decode()

cpu_cores = int(run_command("cat /proc/cpuinfo | grep processor | wc -l"))
ticks_per_second = int(run_command("getconf CLK_TCK"))

prev_sum = None
while True:
    stat_output = run_command("cat /proc/stat").split("\n")[0].split()[1:]
    usr = int(stat_output[0])
    nice = int(stat_output[1])
    sys = int(stat_output[2])
    cnt_sum = usr + nice + sys
    if prev_sum is None:
        prev_sum = cnt_sum
        sleep(1)
    diff = cnt_sum - prev_sum
    prev_sum = cnt_sum
    print((diff / ticks_per_second / cpu_cores) * 100)
    sleep(1)


