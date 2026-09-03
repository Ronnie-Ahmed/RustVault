FROM rust:1.85-slim-bookworm AS builder

RUN useradd -m appuser
USER appuser

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release


FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/Task_Api ./Task_Api
EXPOSE 3000

CMD ["./Task_Api"]
