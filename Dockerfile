FROM rust:latest as build

WORKDIR /usr/src/short_link
COPY . .

RUN cargo run --release
