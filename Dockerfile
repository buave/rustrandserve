FROM debian:bookworm AS builder

RUN apt update && apt install -y curl build-essential clang gcc pkg-config \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/rustrandserve /app/

CMD ["./rustrandserve"]
