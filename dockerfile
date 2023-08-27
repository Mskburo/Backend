FROM lukemathwalker/cargo-chef:latest-rust-1-alpine3.17 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release  --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin tours_back

FROM alpine AS runtime
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/tours_back /usr/local/bin/tours_back
CMD ["/usr/local/bin/tours_back"]