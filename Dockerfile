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

EXPOSE 8000

ENV RUST_LOG=info

COPY --from=builder /app/target/release/sedna-worker ./sedna-worker

CMD ["./sedna-worker"]