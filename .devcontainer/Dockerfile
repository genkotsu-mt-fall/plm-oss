FROM rust:slim

RUN apt-get update

RUN rustup update && \
    rustup component add rustfmt && \
    rustup component add clippy

RUN apt-get install -y unzip libssl-dev pkg-config
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

RUN apt-get install -y unzip curl
RUN curl -fsSL https://bun.sh/install | bash

# 環境変数を通す
# ENV PATH="/root/.bun/bin:${PATH}"