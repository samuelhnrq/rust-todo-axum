FROM debian:bookworm-slim

COPY ./views/www /opt/www
COPY ./target/release/rust_todo_api /usr/bin

ENV PORT=8080
ENV HT_WWW_STATIC_FILES=/opt/www

EXPOSE 8080
# Avoiding recieving PID 1
CMD ["sh", "-c", "rust_todo_api"]
