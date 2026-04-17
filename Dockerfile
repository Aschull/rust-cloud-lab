# Estágio 1: Planejamento
FROM rust:latest AS planner
WORKDIR /app
RUN cargo install cargo-chef --version 0.1.77
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Estágio 2: Builder
FROM rust:latest AS builder
WORKDIR /app
COPY --from=planner /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

# Estágio 3: Runtime — mesma base do builder!
FROM rust:latest
WORKDIR /app
COPY --from=builder /app/target/release/rust-cloud-lab .
EXPOSE 3000
CMD ["./rust-cloud-lab"]
