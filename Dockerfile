# ==========================
# Stage 1: Build
# ==========================
FROM rust:1.89 AS builder

WORKDIR /app

# Cache das dependências
COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

RUN cargo build --release

# Copia o código real
COPY ./src ./src

# Build final
RUN cargo build --release

# ==========================
# Stage 2: Runtime
# ==========================
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/sedna-worker ./sedna-worker

EXPOSE 8000

ENV RUST_LOG=info

CMD ["./sedna-worker"]