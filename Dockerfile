FROM lukemathwalker/cargo-chef:latest-rust-slim-trixie AS chef

WORKDIR /app

RUN apt update && apt install lld clang -y

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:latest AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/rustmailer rustmailer

COPY configuration configuration

COPY .env .env

ENTRYPOINT [ "./rustmailer" ]




