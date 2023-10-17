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
RUN addgroup -S lumen && adduser -S lumen -G lumen

# copy the binary from the builder
WORKDIR /app
COPY --from=builder /app/target/release/lumen .
RUN chown -R lumen:lumen /app && chmod +x lumen

USER lumen

ENTRYPOINT ["./lumen"]