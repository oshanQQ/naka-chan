FROM rust:1.57-slim

COPY . /naka-chan
WORKDIR /naka-chan

RUN apt update && apt upgrade -y && apt install -y build-essential libopus0 libopus-dev automake autoconf libtool

RUN cargo build --release
