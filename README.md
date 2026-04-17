# Quark Shift API

Um serviço simples de encurtador de URLs construído em Rust, usando Axum e SeaORM com PostgreSQL.

## Tecnologias

- [Rust](https://www.rust-lang.org/) (edition 2024)
- [Axum](https://github.com/tokio-rs/axum) — framework web
- [SeaORM](https://www.sea-ql.org/SeaORM/) — ORM para PostgreSQL
- [Tokio](https://tokio.rs/) — runtime assíncrono
- PostgreSQL 14
- Docker / Fly.io para deploy

## Estrutura

```
src/
├── main.rs        # Bootstrap da aplicação
├── routes.rs      # Configuração de rotas, CORS e tracing
├── handlers.rs    # Handlers HTTP (home, shorten, redirect, ping)
├── models.rs      # Entidade `urls` (SeaORM)
├── db.rs          # Conexão com o banco
└── response.rs    # Padronização de respostas da API
migration/         # Migrações SeaORM
```

## Variáveis de ambiente

Crie um arquivo `.env` na raiz:

```env
DATABASE_URL=postgres://admin:admin@localhost:5432/database
APP_URL=http://localhost:3000
ORIGIN_URL=*
```

## Rodando localmente

1. Suba o PostgreSQL via Docker:

```bash
docker network create shared_network
docker compose up -d
```

2. Execute as migrações:

```bash
cd migration
cargo run
```

3. Inicie a API:

```bash
cargo run
```

A API ficará disponível em `http://localhost:3000`.

## Endpoints

| Método | Rota        | Descrição                                |
|--------|-------------|------------------------------------------|
| GET    | `/`         | Informações da API                       |
| GET    | `/ping`     | Health check                             |
| POST   | `/shorten`  | Encurta uma URL                          |
| GET    | `/{code}`   | Redireciona para a URL original          |

### Exemplo

```bash
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'
```

Resposta:

```json
{
  "status": 200,
  "message": "URL shortened successfully",
  "data": { "shortened_url": "http://localhost:3000/abc12345" }
}
```

## Deploy

O projeto inclui `Dockerfile` e `fly.toml` prontos para deploy no [Fly.io](https://fly.io):

```bash
fly deploy
```

## Autor

[github.com/hjunior29](https://github.com/hjunior29)
