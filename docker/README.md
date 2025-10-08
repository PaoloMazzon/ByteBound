# Top-level server
Run the following command at the root directory of this repo on the server to set it up:

```bash
docker build -t server -f docker/server.Dockerfile .
docker build -t runner -f docker/runner.Dockerfile .
docker run -d --rm -v /var/run/docker.sock:/var/run/docker.sock -p 80:80 -v /share_folder:/app/clients/ --name server server
docker cp API_KEY_PATH server:/app/.env
```

Kill it later simply with 

```bash
docker kill server
```
