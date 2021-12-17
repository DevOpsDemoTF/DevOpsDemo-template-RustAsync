FROM rust:1.57.0-alpine3.14 as fetch
WORKDIR /app

ENV CARGO_TERM_COLOR never
RUN apk add --no-cache musl-dev && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo install cargo2junit cargo-chef

COPY Cargo.lock Cargo.toml ./
RUN cargo fetch

FROM fetch as plan
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

FROM fetch as build
COPY --from=plan /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY Cargo.lock Cargo.toml ./
COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl --bin app && \
    cargo test --release --target x86_64-unknown-linux-musl -- -Z unstable-options --format json | cargo2junit > test-results.xml

FROM alpine:3.14
WORKDIR /app

RUN apk --no-cache add ca-certificates && \
    addgroup -S app && adduser -S app -G app

EXPOSE 8080
EXPOSE 9102

ENV LOG_LEVEL "DEBUG"

COPY --from=build /app/target/*/release/app /app/test-results.xml /app/

USER app
CMD ["./app"]
