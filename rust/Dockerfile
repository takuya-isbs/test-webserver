FROM rust:bullseye as builder

RUN cd / && cargo new --bin app
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm -rf src
COPY ./src ./src
RUN touch -am ./src/main.rs
RUN cargo install --locked --path .

FROM debian:bullseye-slim
RUN apt-get update \
    ###  && apt-get install -y extra-runtime-dependencies \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rustweb /usr/local/bin/rustweb
ENV RUST_LOG=debug
CMD ["rustweb"]
