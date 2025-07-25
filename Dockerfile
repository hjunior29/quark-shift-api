# Etapa de build
FROM clux/muslrust:stable AS builder

WORKDIR /app

# Copia tudo para o container
COPY . .

# Compila com target estático
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Etapa de runtime (mínima possível)
FROM scratch

# Copia o binário estático para o container final
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/quark-shift-api /quark-shift-api

# Define ponto de entrada
ENTRYPOINT ["/quark-shift-api"]