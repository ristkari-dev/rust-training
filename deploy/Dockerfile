# syntax=docker/dockerfile:1.7

# --- Stage 1: build the static dist/ ---
FROM rust:1.85-slim-bookworm AS builder
WORKDIR /src
COPY . .
RUN cargo build --release --package build-index \
    && ./target/release/build-index \
        --lessons /src/lessons \
        --shared /src/shared/reveal \
        --out /dist

# --- Stage 2: serve dist/ with non-root nginx ---
FROM nginxinc/nginx-unprivileged:alpine
USER root
COPY deploy/nginx.conf.template /etc/nginx/templates/default.conf.template
COPY --from=builder /dist /usr/share/nginx/html
# Drop back to the unprivileged user the image ships with (UID 101).
USER 101
EXPOSE 8080
