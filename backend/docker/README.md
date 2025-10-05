# Top-level server
Run the following commands on the server to set it up

```bash
docker build -t server -f server.Dockerfile .
docker build -t runner -f runner.Dockerfile .
docker run -d --rm -v /var/run/docker.sock:/var/run/docker.sock -p 80:80 --name server server
docker cp API_KEY_PATH server:/app/.env
```

Kill it later simply with 

```bash
docker container kill server
```
