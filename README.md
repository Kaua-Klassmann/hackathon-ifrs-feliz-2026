# Hackathon IFRS Feliz 2026 — Backend

Backend desenvolvido em **Rust** para o projeto desenvolvido durante a **Hackathon IFRS Campus Feliz 2026**, em que nossa equipe conquistou o **1º lugar**.

## Tecnologias

* **Rust 2024**
* **Axum** — framework HTTP
* **Tokio** — runtime assíncrono
* **SeaORM** — ORM
* **PostgreSQL** — banco de dados
* **Serde / Serde JSON** — serialização e desserialização
* **Validator** — validação de dados
* **JSON Web Token (JWT)** — autenticação
* **Argon2** — hashing de senhas
* **Tower HTTP** — CORS, compressão e tratamento de panics
* **Tracing** — logging e observabilidade
* **Docker / Docker Compose**

## Arquitetura

O backend utiliza uma arquitetura em camadas, separando as responsabilidades da aplicação:

```text
Request
   │
   ▼
Routes
   │
   ▼
Controllers
   │
   ▼
Services
   │
   ▼
Repositories
   │
   ▼
SeaORM
   │
   ▼
PostgreSQL
```

* **Routes** — definição dos endpoints da API.
* **Controllers** — recebem as requisições HTTP e retornam as respostas.
* **Services** — concentram a lógica da aplicação.
* **Repositories** — responsáveis pelo acesso aos dados.
* **Entities** — representam as entidades utilizadas pela persistência.
* **Middlewares** — funcionalidades que interceptam o fluxo das requisições, como CORS, compressão e tratamento de erros.
* **Configs** — configuração de recursos como banco de dados, JWT e CORS.

## Organização

```text
src/
├── configs/
├── connections/
├── controllers/
├── entities/
├── middlewares/
├── repositories/
├── routes/
├── services/
├── app.rs
├── error.rs
├── jwt.rs
├── main.rs
└── utils.rs

migration/
└── Migrations do banco de dados
```

As migrations ficam separadas da aplicação principal em um workspace próprio.

## Execução

O projeto possui `Dockerfile` e `docker-compose.yml` para facilitar a execução do ambiente.

---

**Desenvolvido por Kauã Klassmann — Hackathon IFRS Campus Feliz 2026.**
