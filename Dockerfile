FROM rust:1.97.1-alpine as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:3.22.5

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/hackathon_backend /usr/local/bin/app

CMD ["app"]