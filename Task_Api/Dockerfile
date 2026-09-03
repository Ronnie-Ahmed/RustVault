FROM rust:1.85-slim-bookworm AS builder



WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release


FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/Task_Api ./Task_Api
RUN useradd -m appuser
USER appuser
EXPOSE 3000

CMD ["./Task_Api"]
