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


# FROM server
# RUN apt-get update && apt-get install -y libssl1.1 && apt clean && rm -rf /var/lib/apt/lists/*
# WORKDIR /app/tyme  
# COPY --from=server /app/tyme/target/release/tyme-server ./tyme-server
# COPY --from=web /app/tyme-console/dist ./tyme-console
# CMD ["./tyme-server"]


# FROM scratch
# COPY --from=server /usr/lib/x86_64-linux-gnu/libssl.so.1.1 /usr/lib/x86_64-linux-gnu/
# COPY --from=server /usr/lib/x86_64-linux-gnu/libcrypto.so.1.1 /usr/lib/x86_64-linux-gnu/
# COPY --from=server /app/tyme/target/release/tyme-server ./tyme-server
# COPY --from=web /app/tyme-console/dist ./tyme-console

FROM alpine:latest AS base
RUN apk update && apk add --no-cache openssl
WORKDIR /app/tyme
COPY --from=server /app/tyme/target/release/tyme-server ./tyme-server
COPY --from=web /app/tyme-console/dist ./tyme-console
# 初始化配置文件
# 创建ssl文件夹 log文件夹 data文件夹 
CMD ["./tyme-server"]




