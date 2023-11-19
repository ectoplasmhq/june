# Build stage
FROM rustlang/rust:nightly-slim as builder
USER 0:0
WORKDIR /home/rust/src

RUN USER=root cargo new --bin june
WORKDIR /home/rust/src/june
COPY Cargo.toml Cargo.lock .
COPY src ./src
RUN apt-get update && apt-get install -y libssl-dev pkg-config && cargo install --locked --path .

# Bundle stage
FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates ffmpeg
COPY --from=builder /usr/local/cargo/bin/june .
EXPOSE 7000
ENV JUNE_HOST 0.0.0.0:7000
CMD ["./june"]
