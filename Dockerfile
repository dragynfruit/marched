FROM rust:alpine as builder

WORKDIR /usr/src/marched
COPY . .

RUN apk add --no-cache -U musl-dev openssl-dev
RUN cargo build --release

FROM alpine:latest

COPY --from=builder /usr/src/marched/target/release/marched /usr/local/bin/marched/marched
COPY --from=builder /usr/src/marched/templates /usr/local/bin/marched/templates

WORKDIR /usr/local/bin/marched
CMD ["./marched"]