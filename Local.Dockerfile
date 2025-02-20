FROM rust:1-bookworm as build

WORKDIR /app

ENV CARGO_HOME=.cargo_cache

COPY . ./
RUN --mount=type=cache,target=.cargo_cache \
  --mount=type=cache,target=target \
  cargo fmt --check --all && \
  cargo clippy --release --locked && \
  cargo build --release --locked && \
  cp target/release/rust_todo_api .

FROM debian:bookworm-slim

ENV PORT=8080
ENV WWW_STATIC_FILES=/opt/www

COPY ./views/www /opt/www
COPY --from=build /app/rust_todo_api /usr/bin

EXPOSE 8889
# Avoiding recieving PID 1
CMD ["rust_todo_api"]
