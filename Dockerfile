FROM rust:alpine3.23

WORKDIR /app

COPY . .

RUN cargo build --release

ENTRYPOINT [ "./target/release/rustmailer" ]

