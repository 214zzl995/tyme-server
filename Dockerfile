FROM rust:1.76.0-slim-bullseye AS server
RUN apt update && apt install -y libssl-dev cmake libclang-dev pkg-config g++ 
# RUN apt install -y libssl1.1
WORKDIR /app/tyme  
COPY . ./
RUN cargo build --release

FROM node:16.14.0-alpine AS web
WORKDIR /app/tyme-console
COPY ./tyme-console .
RUN npm install
RUN npm run build

FROM alpine:latest AS base
RUN apk update && apk add --no-cache openssl
ENV TYME_WORKDIR=/data/tyme/
ENV TYME_CONF=/data/tyme/tyme_conf.toml
WORKDIR /app/tyme
COPY --from=server /app/tyme/target/release/tyme-server ./tyme-server
COPY --from=web /app/tyme-console/dist ./tyme-console
RUN mkdir -p "$TYME_WORKDIR/ssl" "$TYME_WORKDIR/log" "$TYME_WORKDIR/data"
CMD ["./tyme-server -w $TYME_WORKDIR -c $TYME_CONF"]




