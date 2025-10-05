#!/bin/bash
set -e

BINARY_PATH="$1"
CPU_LIMIT="$2"
MEMORY_LIMT="$3"

CGROUP="/sys/fs/cgroup/myrunner"
mkdir -p "$CGROUP"

# Memory limit 256 MB
echo $(($MEMORY_LIMIT*1024*1024)) > "$CGROUP/memory.max"

# CPU limit 50% (quota period 100000 µs)
echo "$CPU_LIMIT" > "$CGROUP/cpu.max"

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