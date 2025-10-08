#!/usr/bin/env bash
# This script bootstraps a server together into /app/, also creating a systemd server for it

docker build -t server -f docker/server.Dockerfile .
docker build -t runner -f docker/runner.Dockerfile .
docker cp 

# /app/backend/backend-server/target/x86_64-unknown-linux-musl/release/backend-server /app/server
# /app/react_frontend/bytebound/dist/ /app/
# /app/challenges/ /app/challenges/