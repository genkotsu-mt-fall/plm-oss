# ビルドステージ
FROM rust:slim AS builder

RUN apt-get update && apt-get install -y curl

RUN rustup update && \
    rustup component add rustfmt && \
    rustup component add clippy

RUN apt-get install -y unzip libssl-dev pkg-config
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

WORKDIR /usr/src/app
# COPY . .

# 必要な依存キャッシュを効かせるために先にCargoファイルをコピー
COPY Cargo.toml Cargo.lock ./
# RUN mkdir src && echo "fn main() {}" > src/main.rs
# ビルド
# 一度失敗しても依存ダウンロードは終わっている
# RUN cargo build --release || true  

# COPY . .
COPY .sqlx .sqlx
COPY src ./src
COPY .env .env
COPY migrations ./migrations

ENV SQLX_OFFLINE=true

# RUN cargo install --path .
RUN cargo build --release
# RUN cargo install --path . --locked

# ランタイムステージ
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y unzip libssl-dev pkg-config ca-certificates \
    && rm -rf /var/lib/apt/lists/*
    
    # && apt-get install -y libssl-dev pkg-config \

WORKDIR /workspace
# COPY --from=builder /usr/local/cargo/bin/app /usr/local/bin/app
# COPY --from=builder /usr/src/app/target/release/app /usr/local/bin/app
COPY --from=builder /usr/src/app/target/release/ .
COPY --from=builder /usr/src/app/.env .env
# WORKDIR /usr/local/bin

# EXPOSE 8080
CMD ["./app"]
# ENTRYPOINT ["./app"]