# GitHub Copilot Instructions

This repository is intended to grow into a personal full-stack project with the following default stack:

- **Frontend:** Vue 3, TypeScript, Vite, Pug templates, SCSS styles
- **Backend:** Rust, Axum, Tokio, SQLx
- **Database:** PostgreSQL
- **DevOps:** Docker, Docker Compose, and optional Nginx reverse proxy

See also: #file:.github/skill.md

## Repository Goal

Prefer a clean monorepo structure with clear separation between frontend, backend, database migrations, and deployment assets.

Preferred high-level structure:

```text
.github/
  copilot-instructions.md
  skill.md
  prompts/
backend/
frontend/
infrastructure/ or docker/
```

## Stack Defaults

### Backend

- Use **Rust + Axum** for REST API services.
- Prefer a **microservice-friendly REST API** design with versioned routes such as `/api/v1/...`.
- Use async Rust with **Tokio**.
- Use **SQLx** with **PostgreSQL**.
- Prefer a structured backend layout such as:
  - `routes/`
  - `handlers/`
  - `models/`
  - `db/`
  - `errors/`
- Return JSON consistently.
- Prefer explicit types and clear error handling.
- When practical, use `Result<T, AppError>`-style patterns.

### Frontend

- Use **Vue 3** with the Composition API.
- Prefer **`<script setup lang="ts">`**.
- Prefer **Pug** in Vue SFC templates:
  - use `<template lang="pug">`
- Prefer **SCSS** for component styling:
  - use `<style lang="scss" scoped>` unless shared styling is intentionally needed.
- Keep components small, reusable, and strongly typed.
- Prefer API access through a dedicated service layer rather than inline request logic as the project grows.

### API

- The backend should act as a **REST API** service.
- Use resource-oriented endpoints.
- Use standard HTTP status codes.
- Use JSON request and response bodies unless there is a strong reason not to.
- Validate request payloads clearly and return predictable error responses.

### Database

- Use **PostgreSQL** as the default database.
- Prefer migration-based schema changes.
- Keep schema definitions, migrations, and Rust models aligned.
- Prefer UUID primary keys for distributed-service-friendly design unless requirements suggest otherwise.

### Docker

- Prefer containerized local development.
- Expected services may include:
  - `backend`
  - `frontend`
  - `db`
  - `nginx` as an optional reverse proxy or gateway
- Prefer `docker-compose.yml` for local orchestration.
- Keep environment variable names explicit, for example:
  - `DATABASE_URL`
  - `RUST_LOG`
  - `VITE_API_BASE_URL`

## Markdown and Prompt Rules

- Keep all Copilot-related markdown files under **`.github/`**.
- Keep all reusable prompt files under **`.github/prompts/`**.
- Do not create new prompt markdown files outside `.github/prompts/`.
- Any new prompt created by an agent should be placed under **`.github/prompts/`**.
- Prefer `mode: edit` for prompts that mainly update one file or a tightly scoped set of files.
- Prefer `mode: agent` for prompts that coordinate changes across multiple files or layers.

## Default Decision Rules

When asked to scaffold features for this repository:

1. Respect the target full-stack architecture.
2. Keep backend and frontend concerns separated.
3. Use Pug and SCSS by default on the frontend.
4. Use Axum and PostgreSQL by default on the backend.
5. Prefer Docker-friendly decisions for ports, hostnames, service names, and local orchestration.
6. Add concise documentation when introducing major project structure.

## Editing Principles

- Prefer minimal, targeted changes.
- Preserve naming consistency across files.
- When introducing new folders, keep naming predictable and lowercase.
- When requirements are ambiguous, choose maintainable defaults aligned with the stack above.
- Avoid restructuring unrelated parts of the repository unless the task clearly requires it.

