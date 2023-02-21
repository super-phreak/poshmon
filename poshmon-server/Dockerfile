FROM rust:1.67 as builder
WORKDIR /usr/src/poshmon-server
COPY poshmon-server .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y openssl libsqlite3-0 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/poshmon-server /usr/local/bin/poshmon-server
COPY data /data
ENV DATA_ROOT="data/"
EXPOSE 8080
CMD ["poshmon-server"]