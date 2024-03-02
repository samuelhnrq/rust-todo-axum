FROM rust:1-bookworm as build

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

ENV PORT=8080
ENV WWW_STATIC_FILES=/opt/www

COPY ./www /opt/www
COPY --from=build /app/rust_todo /usr/bin

EXPOSE 8080
# Avoiding recieving PID 1
CMD ["sh", "-c", "rust_todo"]
