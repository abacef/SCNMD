from subprocess import run

from time import sleep

def run_command(command):
    p = run(command, capture_output=True, shell=True)
    return p.stdout.decode()

cpu_cores = int(run_command("cat /proc/cpuinfo | grep processor | wc -l"))


while True:
    uptime_output = run_command("cat /proc/uptime").split()
    uptime_seconds = float(uptime_output[0])
    cpu_time_seconds = float(uptime_output[1])
    cpu_percent = (cpu_time_seconds / cpu_cores) / uptime_seconds
    print(cpu_percent)
    sleep(1)

