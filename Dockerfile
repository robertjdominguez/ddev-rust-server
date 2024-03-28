# Build stage
FROM rust:bookworm AS builder
 
WORKDIR /app
COPY . .
RUN cargo build --release
 
# Final run stage
FROM debian:bookworm-slim AS runner

EXPOSE 8080
 
WORKDIR /app
COPY --from=builder /app/target/release/server /app/server
CMD ["/app/server"]

