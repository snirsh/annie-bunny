FROM rust:latest

RUN USER=root rustup override set nightly
RUN USER=root cargo new --bin annie-bunny
WORKDIR /annie-bunny

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/annie-bunny*

RUN cargo install --path
EXPOSE 8000
CMD ["annie-bunny"]
