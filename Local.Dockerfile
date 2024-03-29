FROM rust:1-bookworm as build

WORKDIR /app

ENV CARGO_HOME=.cargo_cache

COPY . ./
RUN --mount=type=cache,target=.cargo_cache \
    --mount=type=cache,target=target \
    cargo build --release --locked && \
    cp target/release/rust_todo_api .

FROM debian:bookworm-slim

ENV PORT=8080
ENV WWW_STATIC_FILES=/opt/www

COPY ./views/www /opt/www
COPY --from=build /app/rust_todo_api /usr/bin

EXPOSE 8080
# Avoiding recieving PID 1
CMD ["rust_todo_api"]
