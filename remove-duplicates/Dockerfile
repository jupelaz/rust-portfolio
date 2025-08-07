FROM rust:1.77 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/remove_duplicates /usr/local/bin/remove_duplicates
COPY static /static
COPY templates /templates

EXPOSE 8080
CMD ["remove_duplicates"]
