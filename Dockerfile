# Build stage
FROM clux/muslrust:stable AS builder

WORKDIR /app

# Copy everything to container
COPY . .

# Compile with static target
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Runtime stage (minimal possible)
FROM scratch

# Copy static binary to final container
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/quark-shift-api /quark-shift-api

# Define entrypoint
ENTRYPOINT ["/quark-shift-api"]