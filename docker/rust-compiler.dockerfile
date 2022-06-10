FROM rustlang/rust:nightly-slim

RUN cargo install xargo
RUN rustup component add rust-src

WORKDIR /root
ENV RUST_TARGET_PATH=/root

ENTRYPOINT ["xargo"]
