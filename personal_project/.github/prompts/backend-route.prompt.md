---
mode: agent
description: Create or extend an Axum REST resource across routes, handlers, models, and database layers
---

Use #file:.github/copilot-instructions.md and #file:.github/skill.md as project context.

Generate or update a backend REST resource for this repository.

## Goal

Create a resource-oriented backend implementation using Rust, Axum, SQLx, and PostgreSQL.

## Requirements

- Inspect the existing backend structure first.
- Prefer versioned routes under `/api/v1/`.
- Keep route registration separate from handler logic.
- Use typed request/response payloads.
- Use PostgreSQL-friendly data models.
- Prefer SQLx-compatible patterns when database access is needed.
- Keep errors explicit and predictable.
- Return JSON consistently.

## Expected files to consider

- `backend/src/main.rs`
- `backend/src/routes/`
- `backend/src/handlers/`
- `backend/src/models/`
- `backend/src/db/`
- `backend/migrations/`

## Expected outputs

When appropriate, create or update:

- route module
- handler module
- model structs
- database access logic
- migration stub or migration update
- registration changes in the app bootstrap

## Scope and quality

- This is a `mode: agent` prompt and may coordinate multi-file changes.
- Keep naming aligned across route, handler, model, and table names.
- Return JSON responses.
- Use maintainable module boundaries.
- Prefer minimal but complete implementation.

