#!/usr/bin/env bash

# start backend
./server >> backend.log &
python3 server.py
