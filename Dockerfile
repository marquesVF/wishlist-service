ARG RUST_VERSION=1.74.1

FROM rust:${RUST_VERSION}-slim-bookworm AS builder

WORKDIR /app
COPY . .

RUN apt update
RUN apt install libsqlite3-dev sqlite3 -y
RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  cargo build --locked --release && \
  cp ./target/release/wishlist-service /app

FROM debian:bookworm-slim AS final
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "10001" \
  appuser
COPY --from=builder /app/wishlist-service /usr/local/bin
RUN chown appuser /usr/local/bin/wishlist-service
USER appuser
ENV RUST_LOG="wishlist_service=debug,info"
WORKDIR /opt/wishlist-service
ENTRYPOINT ["wishlist-service"]
EXPOSE 3000/tcp

