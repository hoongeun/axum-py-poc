FROM rust:1.66 as builder
WORKDIR /usr/src/axumpy-proto
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/axumpy-proto /usr/local/bin/axumpy-proto
CMD ["axum-py-poc"]
