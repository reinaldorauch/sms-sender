FROM rust as builder
RUN USER=root apt-get update && apt-get install -y clang
RUN rustup default stable
WORKDIR /usr/src/
COPY . . 
RUN cargo build --release

FROM debian
WORKDIR /root/
COPY --from=builder /usr/src/target/release/auth ./auth
COPY --from=builder /usr/src/Rocket.toml ./Rocket.toml
EXPOSE 8001/tcp
CMD ["./auth"]
