#!/usr/bin/env bash

# If no parameter given then show usage instructions
if [ -z "$1" ]; then
    echo "Usage: $0 <c_file> [args...]"
    exit 1
fi

# Get c file then shift arguments
CFILE="$1"
shift

# Get name of c file
PROG="${CFILE%.c}"


# Compile c file"
gcc -o "$PROG" "$CFILE"

# If it didn't compile print error then exit
if [ $? -ne 0 ]; then
    echo "Compilation failed"
    exit 1
fi
