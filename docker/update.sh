#!/usr/bin/env bash
# This is for hot-reloads on the server. Do not call this manually.
set -e

pushd /app

echo "[update] Pulling latest code..."
git fetch origin prod
git reset --hard origin/prod

echo "[update] Building new container..."
docker compose -f docker-compose.yml build runner

echo "[update] Gracefully restarting server..."
docker compose -f docker-compose.yml up -d --no-deps --force-recreate server

echo "[update] Done."

popd