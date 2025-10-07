#!/usr/bin/env bash

BINARY_PATH="/client/a.out"
CHALLENGE_PATH="/client/challenge.json"
RUNTIME_PATH="/client/runtime.txt"
TEST_CASE_OUTPUT="/client/test_case_output.txt"

CPU_LIMIT="$1"
MEMORY_LIMIT="$2"

# Limit virtual memory
ulimit -v $MEMORY_LIMIT

# Run the program and capture runtime -- TODO: Each test case
start=$(date +%s%6N)
cpulimit -l $CPU_LIMIT $BINARY_PATH
end=$(date +%s%6N);
echo $((end - start)) μs >> $RUNTIME_PATH