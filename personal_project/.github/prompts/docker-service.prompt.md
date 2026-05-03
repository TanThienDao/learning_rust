---
mode: agent
description: Add or refine Docker and Compose services for the full-stack project
---

Use #file:.github/copilot-instructions.md and #file:.github/skill.md as project context.

Add or update Docker-related configuration for this repository.

## Goal

Create or refine container configuration for one or more project services.

## Requirements

- Inspect existing Docker-related files first.
- Prefer service names that match repository conventions such as `backend`, `frontend`, `db`, and `nginx`.
- Keep environment variables explicit.
- Prefer Docker Compose for local multi-service orchestration.
- Keep networking and port exposure easy to understand.
- Keep PostgreSQL container settings aligned with backend expectations.

## Expected outputs

When appropriate, create or update:

- `Dockerfile`
- `docker-compose.yml`
- `docker-compose.dev.yml`
- reverse proxy config
- environment variable documentation

## Scope and quality

- This is a `mode: agent` prompt and may coordinate multi-file infrastructure updates.
- Prefer minimal, composable configuration.
- Keep container roles clear.
- Align database settings with PostgreSQL conventions.
- Avoid changing unrelated application code unless required by the requested container setup.

