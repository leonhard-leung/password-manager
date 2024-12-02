FROM rust:1.82

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

CMD ["./target/release/password-manager"]