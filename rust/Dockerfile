FROM rust:bullseye AS builder

RUN cd / && cargo new --bin app && ln -s /app /SRC
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
# to cache dependencies
RUN cargo build --release
RUN rm -rf src

COPY ./src ./src
COPY ./test.sh ./test.sh
RUN touch -am ./src/main.rs
RUN cargo install --locked --path .
RUN rustup component add clippy

#FROM debian:bullseye-slim
RUN apt-get update
#RUN apt-get install -y extra-runtime-dependencies
#RUN rm -rf /var/lib/apt/lists/*
#COPY --from=builder /usr/local/cargo/bin/rustweb /usr/local/bin/rustweb

ENV RUST_LOG=debug
ENV TZ=Asia/Tokyo
CMD ["rustweb"]
