FROM rust:1.80-alpine as builder

WORKDIR /tembo-metrics

COPY . .

RUN apk update && apk upgrade && apk add --no-cache musl-dev openssl ca-certificates

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /tembo-metrics/target/x86_64-unknown-linux-musl/release/tembo-metrics /tembo-metrics
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

CMD ["/tembo-metrics"]
