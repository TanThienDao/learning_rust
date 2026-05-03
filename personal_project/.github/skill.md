# Skill Reference

This file provides shared implementation defaults for Copilot agents working in this repository.

## Repository Intent

Build a personal full-stack application with:

- **Frontend:** Vue 3, TypeScript, Vite, Pug templates, SCSS styles
- **Backend:** Rust, Axum, Tokio, SQLx
- **Database:** PostgreSQL
- **DevOps:** Docker, Docker Compose, and optional Nginx reverse proxy

## Skill Defaults

### Backend skills

- Use Rust with Axum for REST API services.
- Use Tokio for async runtime behavior.
- Use SQLx for PostgreSQL access and migration-friendly workflows.
- Use Serde for typed JSON request and response payloads.
- Use tower-http middleware when needed (for example CORS and tracing).
- Prefer explicit error handling and `Result<T, AppError>` style patterns.

### Frontend skills

- Use Vue 3 with the Composition API.
- Use TypeScript for component and API typing.
- Prefer Vue SFCs with:
  - `<template lang="pug">`
  - `<script setup lang="ts">`
  - `<style lang="scss" scoped>`
- Prefer reusable components and a dedicated API/service layer as complexity grows.

### Database and API skills

- Use PostgreSQL as the default database.
- Use migration-based schema changes.
- Keep schema, migration files, and Rust models aligned.
- Prefer resource-oriented REST endpoints under `/api/v1/...`.
- Return JSON consistently and use standard HTTP status codes.

### DevOps skills

- Use Docker for consistent local and deployment environments.
- Use Docker Compose for multi-service local development.
- Keep service naming predictable (`backend`, `frontend`, `db`, `nginx`).
- Keep environment variables explicit (for example `DATABASE_URL`, `RUST_LOG`, `VITE_API_BASE_URL`).

## Preferred Structure

```text
backend/
  src/
    routes/
    handlers/
    models/
    db/
    errors/
  migrations/

frontend/
  src/
    components/
    views/
    composables/
    services/
    router/
    assets/
```

## Prompt and Markdown Rules

- Keep all repository-specific instruction markdown inside `.github/`.
- Keep all reusable prompts inside `.github/prompts/`.
- Do not create prompt markdown files outside `.github/prompts/`.
- Prefer `mode: edit` when work is scoped to one file or a tightly related set of files.
- Prefer `mode: agent` when work spans multiple files or multiple layers.

## Agent Decision Rules

When requirements are ambiguous:

1. Prefer maintainable defaults aligned with this stack.
2. Keep frontend and backend concerns separated.
3. Prefer minimal, targeted edits over broad restructuring.
4. Preserve naming consistency across routes, models, components, and services.

