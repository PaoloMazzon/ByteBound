# Top-level server
Run the following commands on the server to set it up

```bash
docker build -t server -f server.Dockerfile .
docker build -t runner -f runner.Dockerfile .
docker run -d --rm --volume $(pwd)/backend.log:/app/backend.log --name server server
```

Kill it later simply with 

```bash
docker container kill server
```
