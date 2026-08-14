FROM rust:1.97.1-alpine AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --workspace

FROM alpine:3.22.5

RUN apk add --no-cache ca-certificates jq

COPY --from=builder /app/target/release/hackathon_backend /usr/local/bin/app
COPY --from=builder /app/target/release/migration /usr/local/bin/migration

ENTRYPOINT ["sh", "-c", "export DATABASE_URL=\"postgres://${DB_USER}:$(printf '%s' \"$DB_PASSWORD\" | jq -sRr @uri)@${DB_HOST}:${DB_PORT}/${DB_NAME}${DB_SSL_MODE:+?sslmode=${DB_SSL_MODE}}\" && exec \"$@\"", "--"]
CMD ["app"]