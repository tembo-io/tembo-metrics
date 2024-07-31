FROM rust:1.80-alpine as builder

WORKDIR /tembo-metrics

COPY . .

RUN apk update && apk upgrade && apk add --no-cache musl-dev openssl ca-certificates

RUN cargo build --release

FROM alpine:3.20

RUN apk update && apk upgrade && apk add --no-cache ca-certificates && rm -rf /var/cache/apk/*

COPY --from=builder /tembo-metrics/target/release/tembo-metrics /usr/local/bin/tembo-metrics

CMD ["/usr/local/bin/tembo-metrics"]
