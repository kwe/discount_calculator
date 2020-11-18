FROM rust:1.42 as builder
WORKDIR /usr/src

# 1a: Prepare for static linking
RUN apt-get update && \
  apt-get dist-upgrade -y && \
  apt-get install -y musl-tools && \
  rustup target add x86_64-unknown-linux-musl

# 1c: Build the exe using the actual source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY shop ./shop
RUN cd shop && cargo test && cd ..
RUN cargo install --target x86_64-unknown-linux-musl --path .

# 2: Copy the exe and extra files ("static") to an empty Docker image
FROM scratch
COPY --from=builder /usr/local/cargo/bin/discount_calculator .
CMD ["./discount_calculator"]