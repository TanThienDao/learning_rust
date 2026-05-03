---
mode: edit
description: Create or refine PostgreSQL migration files and aligned Rust model updates
---

Use #file:.github/copilot-instructions.md and #file:.github/skill.md as project context.

Create or update database migration-related files for this repository.

## Goal

Produce PostgreSQL-oriented migration content and align it with the Rust backend model layer.

## Requirements

- Inspect the current backend and migration layout first.
- Prefer migration-based schema changes.
- Keep table names, column names, and Rust model fields aligned.
- Prefer explicit constraints and indexes where useful.
- If a Rust model must also be updated, keep the change minimal and consistent.
- Preserve compatibility with SQLx-friendly PostgreSQL patterns.

## Expected outputs

Depending on the request, create or update:

- `.up.sql` migration content
- `.down.sql` migration content
- Rust model structs
- notes about required environment variables or migration commands when helpful

## Scope and quality

- This is a `mode: edit` prompt. Focus on migration files and tightly related schema/model files.
- Avoid unrelated application changes.
- Keep the implementation minimal, explicit, and reversible.

