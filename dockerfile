FROM lukemathwalker/cargo-chef:latest-rust-1-alpine3.17 AS chef
WORKDIR /app


FROM chef AS planner
COPY Cargo.toml .
COPY Cargo.lock .
COPY src/main.rs src/main.rs
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY ./src ./src/
COPY ./.sqlx ./.sqlx/
ARG SQLX_OFFLINE=true
RUN cargo build --release --target x86_64-unknown-linux-musl --bin tour_back


FROM alpine AS runtime
EXPOSE 8090
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/tour_back /usr/local/bin/tour_back
CMD ["/usr/local/bin/tour_back"]git 