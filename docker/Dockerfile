# ビルド用
FROM rust:latest AS builder

WORKDIR /docker

# 依存クレートのキャッシュのため先に空ビルド
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY ./ ./
RUN rm -f target/release/deps/rust_actix_server_prototype*
RUN cargo build --release

# ビルド環境での実行
RUN cargo install --path .

# リリース用
FROM debian:buster-slim

RUN apt update && apt install -y libsqlite3-dev libpq-dev libmariadb-dev
COPY --from=builder /docker/target/release/rust_actix_server_prototype /usr/local/bin/rust_actix_server_prototype
# +1 がサーバー側で認識しない模様
COPY rsrc/cert/localhost+1-key.pem /etc/ssl/private/localhost-key.pem
COPY rsrc/cert/localhost+1.pem /etc/ssl/certs/localhost.pem

CMD ["rust_actix_server_prototype"]
