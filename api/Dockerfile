FROM rust:1.91-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo fetch
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/app-design-api /app/app-design-api

USER nonroot:nonroot
EXPOSE 4330
ENTRYPOINT ["/app/app-design-api"]
