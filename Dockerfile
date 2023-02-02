FROM alpine:latest

COPY ./target/x86_64-unknown-linux-musl/release/amir-mark-bot /amir-mark-bot
RUN chmod +x /amir-mark-bot

CMD ["/amir-mark-bot"]