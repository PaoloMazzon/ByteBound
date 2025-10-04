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
WORKDIR /app
COPY --from=build /app/backend/backend-server/target/x86_64-unknown-linux-musl/release/backend-server /app/server

ENTRYPOINT ["/app/server"]