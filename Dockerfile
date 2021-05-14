FROM rust:latest as build

WORKDIR /app
 
COPY ./Cargo.toml/ ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src/ ./src/

RUN cargo build --release

FROM ubuntu:latest

ENV DEBIAN_FRONTEND=noninteractive

RUN mkdir /data

ENV DOWNLOAD_PATH "./data"

RUN apt-get -y update && \
     apt-get -y upgrade  && \
     apt -y install ca-certificates libssl-dev libpq-dev

COPY --from=build /app/target/release/s3-download /usr/local/bin

CMD ["s3-download"]