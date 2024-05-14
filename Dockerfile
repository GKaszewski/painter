FROM rust:1.76 as builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM rust:1.76
WORKDIR /app

COPY --from=builder /app/target/release/painter .
COPY painter-js/dist ./dist
COPY .env .env

EXPOSE 3000

CMD ["./painter"]