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

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
RUN groupadd -g 1001 appgroup
RUN useradd -r -u 1001 -g appgroup appuser

COPY --from=builder /app/target/release/todo-backend ./todo-backend
COPY init-db.sh ./init-db.sh
RUN chown appuser:appgroup ./todo-backend ./init-db.sh
RUN chmod +x ./init-db.sh

USER appuser

EXPOSE 8080

CMD ["./init-db.sh"]