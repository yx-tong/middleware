FROM rustlang/rust:nightly as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM scratch AS exporter
COPY --from=builder /app/target/release/yunxi-server .
