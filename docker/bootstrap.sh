#!/usr/bin/env bash
# Installs Docker and the systemd service on a fresh system.

RED='\033[0;31m'
NC='\033[0m'
GREEN='\033[0;32m'

if [[ $UID -ne 0 ]]; then
    echo -e "[${RED}error${NC}]This script needs to be run as root."
    exit 1
fi

set -e

# Install Docker
if ! command -v docker; then
    apt-get update
    apt-get install ca-certificates curl
    install -m 0755 -d /etc/apt/keyrings
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
    chmod a+r /etc/apt/keyrings/docker.asc
    echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
    $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}") stable" | \
    tee /etc/apt/sources.list.d/docker.list > /dev/null
    apt-get update
    apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
fi

# Add systemd server for server and start it
cp docker/bytebound.service /etc/systemd/system/
systemctl enable bytebound
systemctl start bytebound

echo -e "[${GREEN}success${NC}] ByteBound is now installed. To get automatic updates from prod, you will need to add an SSH private key for this server to Github so actions can access this server."