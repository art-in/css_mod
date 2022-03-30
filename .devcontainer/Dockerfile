FROM rust:latest

# install rust components
RUN rustup component add rustfmt clippy

# for running examples
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk