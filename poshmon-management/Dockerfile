FROM rust:1.67 as builder
WORKDIR /usr/src/poshmon
COPY poshmon-management poshmon-management
COPY poshmon-lib poshmon-lib
RUN cargo install --path poshmon-management

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y openssl libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/poshmon-management /usr/local/bin/poshmon-management
COPY data /data
ENV DATA_ROOT="data/"
ENV POSTGRES_PASSWORD="1&W3RR08IScc*OPrSelx"
ENV DATABASE_URL="postgres://postgres:${POSTGRES_PASSWORD}@poshmon-postgres/local_dev"
EXPOSE 8443
CMD ["poshmon-management"]