FROM rust:1.87-slim AS builder

RUN rustup target add wasm32-unknown-unknown && \
    cargo install trunk --locked

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY index.html Trunk.toml ./

RUN trunk build --release

FROM nginxinc/nginx-unprivileged:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 8080
