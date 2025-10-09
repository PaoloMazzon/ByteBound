#!/usr/bin/env bash
# This is for hot-reloads on the server. Do not call this manually.
set -e

RED='\033[0;31m'
NC='\033[0m'
GREEN='\033[0;32m'

COMPOSE_FILE="/app/docker/docker-compose.yml"
SERVICE="server"

echo -e "[${GREEN}info${NC}] building new server image"
docker compose -f "$COMPOSE_FILE" build

echo -e "[${GREEN}info${NC}] gracefully killing old server container"
docker stop -t 10 "$SERVICE" || true

echo -e "[${GREEN}info${NC}] starting new server instance"
docker compose -f "$COMPOSE_FILE" up -d "$SERVICE"

echo -e "[${GREEN}info${NC}] done"