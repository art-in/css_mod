FROM rust:latest

# install rust components
RUN rustup component add rustfmt clippy

# for yew-trunk example
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk