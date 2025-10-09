#!/usr/bin/env bash
# Bootstraps a fresh linux install into a server for bytebound, mainly in case
# someone deletes the EC2 by accident

# We will work in /app
sudo mkdir /app
pushd /app

# Install Docker
installed=$(docker --version)
if [[ $installed -ne 0 ]]; then
    sudo apt-get update
    sudo apt-get install ca-certificates curl
    sudo install -m 0755 -d /etc/apt/keyrings
    sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
    sudo chmod a+r /etc/apt/keyrings/docker.asc
    echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
    $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}") stable" | \
    sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
    sudo apt-get update
    sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

    # Add Docker user
    sudo usermod -aG docker $USER
    newgrp docker
fi

# Copy repo
sudo apt install -y git
git clone https://github.com/PaoloMazzon/ByteBound .
docker build -t runner docker/runner.Dockerfile .

# Add systemd server for server and start it
sudo cp docker/bytebound.service /etc/systemd/system/
sudo systemctl 

popd

echo "ByteBound is now installed. To get automatic updates from prod, you will need to add an SSH private key for this server to Github so actions can access this server."