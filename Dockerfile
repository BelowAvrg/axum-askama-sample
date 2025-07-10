FROM rust:1.88 as builder

WORKDIR /usr/src/app

COPY . .
ENV SQLX_OFFLINE=true
RUN cargo install --path .

FROM alpine:3.19

COPY --from=builder /usr/local/cargo/bin/axum-askama-sample /usr/local/bin/axum-askama-sample

EXPOSE 3000

CMD ["axum-askama-sample"]%