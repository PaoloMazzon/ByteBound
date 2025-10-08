#!/usr/bin/env bash

BINARY_PATH="/client/a.out"
CHALLENGE_PATH="/client/challenge.json"
RUNTIME_PATH="/client/runtime.txt"
TEST_CASE_OUTPUT="/client/test_case_output.txt"

CPU_LIMIT="$1"
MEMORY_LIMIT="$2"

# Iterate over each test case in the challenge
for n in $(jq -r '.test_cases[].input' $CHALLENGE_PATH); do
    # Run the program and capture runtime -- TODO: Each test case
    start=$EPOCHREALTIME
    bash -c "ulimit -v $MEMORY_LIMIT; exec cpulimit -l $CPU_LIMIT -- $BINARY_PATH $n"
    end=$EPOCHREALTIME
    elapsed_microseconds=$(echo "$end - $start" | bc -l | awk '{printf "%.0f\n", $1 * 1000000}')
    echo $elapsed_microseconds >> $RUNTIME_PATH
done
