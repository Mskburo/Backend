FROM lukemathwalker/cargo-chef:latest-rust-1-alpine3.17 AS chef
WORKDIR /app

FROM nim65s/cargo-binstall as binstal
RUN cargo binstall -y --target x86_64-unknown-linux-musl cargo-cache


FROM chef AS planner
COPY Cargo.toml .
COPY Cargo.lock .
COPY src/main.rs src/main.rs
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS cacher 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json


FROM chef AS builder 

ENV CARGO_HOME=/usr/local/cargo
ENV SCCACHE_DIR=/usr/local/sccache

RUN apk add musl-dev sccache
COPY --from=binstal /usr/local/cargo/bin/ /usr/local/bin/

COPY ./src ./src
COPY ./.sqlx ./.sqlx
COPY Cargo.toml .
COPY Cargo.lock .


RUN cargo cache
# Copy over the cached dependencies
COPY --from=cacher /app/target/ /app/target/
ARG SQLX_OFFLINE=true
RUN cargo build --release --target x86_64-unknown-linux-musl --bin tour_back


FROM alpine AS runtime
EXPOSE 8090
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/tour_back /usr/local/bin/tour_back
CMD ["/usr/local/bin/tour_back"]