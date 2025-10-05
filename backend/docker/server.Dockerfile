# The docker container that will run the entire webserver front and back

# This step clones the remote repo and builds the server
FROM rust:latest AS build

RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*
RUN git clone https://github.com/PaoloMazzon/storm-surge-team.git /app

WORKDIR /app/backend/backend-server

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# This step creates a slim server image to run on
FROM alpine:latest AS server

RUN mkdir /app

# Install docker & gcc
RUN apk add --no-cache \
    docker \
    docker-cli \
    bash \
    curl \
    openrc \
    shadow \
    sudo \
    git \
    iptables \
    tini \
    gcc \
    musl-dev \
    make \
    bash \
    nginx \
    && mkdir -p /run/docker
WORKDIR /app

# Expose HTTP port
EXPOSE 80

# Copy over the rust backend
COPY --from=build /app/backend/backend-server/target/x86_64-unknown-linux-musl/release/backend-server /app/server

# Copy over the React frontend
# TODO: This

# Entrypoint stuff
COPY server_entry.sh /app/entry.sh
RUN chmod 777 /app/entry.sh
ENTRYPOINT ["/app/entry.sh"]