FROM rust:1.89 AS builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

COPY ./src ./src
RUN cargo build --release

COPY ./target/release/sedna-worker ./sedna-worker

EXPOSE 8000
ENV RUST_LOG=info

CMD ["./sedna-worker"]