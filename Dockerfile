FROM rust:1.97.1-alpine

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/hackathon_backend"]