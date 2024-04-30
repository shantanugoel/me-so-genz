FROM rust:latest as builder

WORKDIR /usr/src/mesogenz
COPY . .

RUN cargo build --release
RUN cargo install --path .

FROM debian:stable-slim
RUN apt update && apt install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/mesogenz /usr/local/bin/mesogenz
ENTRYPOINT ["mesogenz"]
