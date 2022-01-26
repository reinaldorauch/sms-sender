FROM rust:alpine as builder
RUN apk add musl-dev
RUN rustup default stable
RUN cargo install cargo-build-deps
WORKDIR /usr/src/ping/
COPY . .  
RUN cargo build --release

FROM alpine
WORKDIR /root/
COPY --from=builder /usr/src/ping/target/release/ping ./ping
COPY --from=builder /usr/src/ping/Rocket.toml ./Rocket.toml
EXPOSE 8000/tcp
CMD ["./ping"]
