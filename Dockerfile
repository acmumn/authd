FROM alpine:latest

ADD target/release/authd /app/authd
RUN ["/app/authd"]
