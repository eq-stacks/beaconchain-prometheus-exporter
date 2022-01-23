FROM rust:1.58 as builder

WORKDIR /exporter

ADD src ./src/
ADD Cargo.lock ./Cargo.lock
ADD Cargo.toml ./Cargo.toml

RUN cargo build --release

FROM debian:stable-slim

COPY --from=builder /exporter/target/release/beaconchain-prometheus-exporter /exporter

RUN apt-get update
RUN apt-get install ca-certificates -y

ENV RUST_LOG=info
ENV ROOT_URL=https://prater.beaconcha.in

ENTRYPOINT [ "/exporter" ]