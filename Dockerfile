FROM debian:bookworm-slim

COPY ./views/www /opt/www
COPY --chmod=755 ./target/release/rust_todo_api /usr/bin

ENV PORT=8889
ENV HT_WWW_STATIC_FILES=/opt/www

EXPOSE 8889
# Avoiding recieving PID 1
CMD ["sh", "-c", "rust_todo_api"]
