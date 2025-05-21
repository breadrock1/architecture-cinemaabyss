ARG FEATURES='--features default'

FROM rust:1.85 AS builder

WORKDIR /app

COPY . .

RUN cargo install ${FEATURES} --bins --path .


# Target layer based on tiny official ubuntu image with neccessary binaries and data to run.
FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl
WORKDIR /app

COPY ./config /app/config
COPY --from=builder /app/target/release/events .

# Execute to initliaze elasticsearch environment
CMD ["/app/events"]

ENTRYPOINT ["/app/events"]

EXPOSE 8082
