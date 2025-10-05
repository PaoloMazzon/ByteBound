#!/bin/bash
set -e

BINARY_PATH="$1"

CGROUP="/sys/fs/cgroup/myrunner"
mkdir -p "$CGROUP"

# Memory limit 256 MB
echo $((256*1024*1024)) > "$CGROUP/memory.max"

# CPU limit 50% (quota period 100000 µs)
echo "50000" > "$CGROUP/cpu.max"

# Move this shell (and child processes) into the cgroup
echo $$ > "$CGROUP/cgroup.procs"


# Verify the limits were set
echo "=== CGROUP VERIFICATION ==="
echo "Memory limit: $(cat $CGROUP/memory.max)"
echo "CPU quota: $(cat $CGROUP/cpu.max)"
echo "Processes in cgroup: $(cat $CGROUP/cgroup.procs)"
echo "Current cgroup: $(cat /proc/$$/cgroup)"
echo "==========================="

# Run the binary
exec "$BINARY_PATH" "${@:2}"