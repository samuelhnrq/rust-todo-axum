FROM rust:1.72-bookworm as build

WORKDIR /app

COPY . ./
RUN --mount=type=cache,target=.cargo_cache \
  --mount=type=cache,target=target \
  --mount=type=cache,target=entity/target \
  --mount=type=cache,target=migration/target \
  export "CARGO_HOME=.cargo_cache" && \
  cargo build --release --locked && \
  cp target/release/rust_todo .

FROM debian:bookworm-slim

RUN apt-get update && \
apt-get install -y libssl3 && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

COPY --from=build /app/rust_todo /usr/bin

CMD ["rust_todo"]
