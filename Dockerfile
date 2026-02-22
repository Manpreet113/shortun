FROM rust:1.88 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM gcr.io/distroless/cc

WORKDIR /app

COPY --from=builder /app/target/release/shortun .

EXPOSE 8080

CMD ["./shortun"]