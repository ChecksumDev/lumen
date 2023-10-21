FROM rust:alpine as builder
ENV RUSTFLAGS="-C target-feature=-crt-static"

# install musl-dev to build static binaries
RUN apk add --no-cache musl-dev

# copy the source code
WORKDIR /app
COPY ./ /app

# do a release build
RUN cargo build --release
RUN strip target/release/lumen

# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:latest as runtime
RUN apk add --no-cache libgcc

# create a non-root user
RUN addgroup -g 1500 lumen && \
    adduser -H -D -u 1500 -G lumen lumen

# copy the binary from the builder
WORKDIR /app
COPY --from=builder /app/target/release/lumen .
RUN chown -R lumen:lumen /app && chmod +x lumen

USER lumen

ENV BIND=0.0.0.0:8080
ENV PUBLIC_URL=http://localhost:8080
ENV RUST_BACKTRACE=1
ENV RUST_LOG=info

ENTRYPOINT ["./lumen"]