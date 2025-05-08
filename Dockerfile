FROM rust:1.86.0-slim-bookworm AS builder
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/poolc-k8s-test /
EXPOSE 8080
CMD ["/poolc-k8s-test"]
