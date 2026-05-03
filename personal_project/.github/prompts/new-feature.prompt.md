---
mode: agent
description: Scaffold a complete full-stack feature across frontend, backend, and database layers
---

Use #file:.github/copilot-instructions.md and #file:.github/skill.md as project context.

Create a complete feature for this repository using the preferred stack:

- Frontend: Vue 3 + TypeScript + Pug + SCSS
- Backend: Rust + Axum + SQLx
- Database: PostgreSQL
- Local orchestration: Docker-friendly conventions

## Goal

Deliver a cohesive full-stack feature with aligned API, database, and frontend integration.

## Requirements

- Inspect the current repository structure before making changes.
- If the backend does not exist yet, scaffold it using Axum-friendly structure.
- If the frontend does not exist yet, scaffold it using Vue 3 conventions.
- If database persistence is required, create or update PostgreSQL migrations.
- Keep API endpoints versioned under `/api/v1/`.
- Keep naming consistent across route names, models, services, and UI labels.

## Backend output expectations

When relevant, create or update:

- router registration
- handler functions
- request/response models
- database model or query layer
- migration files
- environment variable documentation

## Frontend output expectations

When relevant, create or update:

- pages or views
- reusable components
- service or API layer
- router configuration
- typed interfaces

## Style requirements

- Prefer `<template lang="pug">` in Vue SFCs.
- Prefer `<script setup lang="ts">`.
- Prefer `<style lang="scss" scoped>`.
- Prefer small, maintainable modules.
- Add concise documentation if introducing major structure.

## Scope and quality

- This is a `mode: agent` prompt and may coordinate changes across backend, frontend, and database files.
- Make the smallest complete set of changes needed.
- Explain any assumptions briefly.
- If the requested feature spans multiple files, keep the implementation cohesive and production-minded.

