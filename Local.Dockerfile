FROM rust:1-bookworm as build

WORKDIR /app

ENV CARGO_HOME=.cargo_cache

RUN rustup component add rustfmt clippy

COPY . ./
RUN --mount=type=cache,target=.cargo_cache \
  --mount=type=cache,target=target \
  cargo fmt --check --all && \
  cargo clippy --release --locked && \
  cargo build --release --locked && \
  cp target/release/ht-rs-api .

FROM debian:bookworm-slim

RUN mkdir /app
WORKDIR /app
COPY ./views/www /app/www
COPY ./config.toml /app/config.toml
COPY --from=build /app/ht-rs-api /app/

ENV HT_WWW_STATIC_FILES=/app/www

EXPOSE 8889

# Avoiding recieving PID 1
CMD ["sh", "-c", "/app/ht-rs-api"]
