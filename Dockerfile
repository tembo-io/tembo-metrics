FROM rust:1.80 as builder

WORKDIR /tembo-metrics

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get upgrade -y && apt-get clean && rm -rf /var/lib/apt/lists/*

COPY --from=builder /tembo-metrics/target/release/tembo-metrics /usr/local/bin/tembo-metrics

CMD ["tembo-metrics"]
