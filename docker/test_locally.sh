#!/usr/bin/env bash
# Use this to test the server locally

docker build -t server -f docker/server.Dockerfile .
docker build -t runner -f docker/runner.Dockerfile .
docker run -d --rm -v /var/run/docker.sock:/var/run/docker.sock -p 80:80 -v /share_folder:/app/clients/ --name server server
echo "Press CTRL+C to stop displaying the logs. Use \"docker kill server\" to stop the server"
docker logs -f server