ARG RUST_IMAGE=rust:alpine

FROM --platform=$BUILDPLATFORM ${RUST_IMAGE} AS chef
WORKDIR /axum-askama-sample
ENV SQLX_OFFLINE=true

RUN apk add --no-cache musl-dev openssl-dev pkgconfig build-base perl zig \
  && cargo install --locked cargo-chef cargo-zigbuild \
  && rustup target add x86_64-unknown-linux-musl

FROM chef AS planner
WORKDIR /axum-askama-sample

COPY Cargo.toml Cargo.lock ./

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /axum-askama-sample

COPY --from=planner /axum-askama-sample/recipe.json recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo chef cook --release --zigbuild \
      --recipe-path recipe.json \
      --target x86_64-unknown-linux-musl

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/axum-askama-sample/target \
    set -eux && \
    cargo zigbuild -r --target x86_64-unknown-linux-musl && \
    mkdir -p linux/amd64 && \
    ls -la target/x86_64-unknown-linux-musl/release/ && \
    cp target/x86_64-unknown-linux-musl/release/axum-askama-sample linux/amd64/axum-askama-sample && \
    chmod +x linux/amd64/axum-askama-sample

FROM alpine:3.19 AS runtime
WORKDIR /axum-askama-sample

RUN apk add --no-cache ca-certificates

ARG TARGETPLATFORM

COPY --from=builder /axum-askama-sample/linux/amd64/axum-askama-sample ./axum-askama-sample

EXPOSE 8080
CMD ["./axum-askama-sample"]
