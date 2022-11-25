FROM rust:buster
RUN cargo --version
RUN lscpu
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y pkg-config build-essential cmake libssl-dev
COPY . /
RUN cargo build --release
