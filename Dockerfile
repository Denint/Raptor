FROM rust:1.90-alpine AS chef
WORKDIR /app

RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    ca-certificates \
    && cargo install cargo-chef

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.90-alpine AS builder
WORKDIR /app

RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    ca-certificates \
    && cargo install cargo-chef

COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json --release

COPY . .
RUN cargo build --release

FROM alpine:3.19 AS runtime
RUN apk add --no-cache ca-certificates curl

WORKDIR /app

ARG BINARY_PATH=/app/target/release/raptor
COPY --from=builder /app/target/release/raptor ${BINARY_PATH}
RUN chmod +x ${BINARY_PATH} && mv ${BINARY_PATH} /app/raptor

EXPOSE 3000
ENV PORT=3000
ENV LOG_FORMAT=json
ENV FEATURE_INCR_COMMAND=true

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --quiet --tries=1 --spider http://localhost:3000/health || exit 1

CMD ["./raptor"]