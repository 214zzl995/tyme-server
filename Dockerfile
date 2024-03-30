FROM rust:1.76.0-slim AS server
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup component add rustfmt
RUN apt update && apt install -y libssl-dev cmake libclang-dev pkg-config g++ musl-tools musl-dev llvm clang perl   
RUN update-ca-certificates
RUN ln -s /usr/bin/g++ /usr/bin/musl-g++
WORKDIR /app
COPY . ./
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM node:16.14.0-alpine AS web
WORKDIR /app
COPY ./tyme-console .
RUN npm install
RUN npm run buildDocker

FROM alpine:latest AS base
EXPOSE 12566/tcp
RUN apk update && apk add --no-cache openssl
ENV TYME_WORKDIR=/app/data
ENV TYME_CONF=/app/data/tyme_conf.toml
WORKDIR /app
COPY --from=server /app/target/x86_64-unknown-linux-musl/release/tyme-server ./tyme-server
COPY --from=web /app/assets ./assets
COPY ./script/tyme_sys.lua ./tyme_sys.lua
RUN mkdir -p "$TYME_WORKDIR/ssl" "$TYME_WORKDIR/log" "$TYME_WORKDIR/data"
CMD ./tyme-server -w $TYME_WORKDIR -c $TYME_CONF




