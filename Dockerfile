FROM rust:1.72-bookworm as build

WORKDIR /app

COPY . ./
RUN --mount=type=cache,target=target \
  cargo build --release --locked && \
  cp target/release/rust_todo .

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev && apt-get clean

COPY --from=build /app/rust_todo /usr/bin

CMD ["rust_todo"]
