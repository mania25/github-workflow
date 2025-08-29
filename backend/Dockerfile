FROM rust:latest AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y build-essential && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

COPY . .
RUN touch src/main.rs
RUN cargo build --release

FROM alpine:3.20 AS runtime
WORKDIR /app

RUN apk add --no-cache ca-certificates
RUN addgroup -g 1001 -S appgroup
RUN adduser -S appuser -u 1001 -G appgroup

COPY --from=builder /app/target/release/todo-backend ./app
RUN chown appuser:appgroup ./app

USER appuser

EXPOSE 8080

CMD ["./app"]