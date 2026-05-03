---
mode: edit
description: Create or refine a Vue 3 single-file component using Pug, TypeScript, and SCSS
---

Use #file:.github/copilot-instructions.md and #file:.github/skill.md as project context.

Create or update a Vue single-file component for this repository.

## Goal

Create a maintainable Vue component that follows project defaults for Vue 3, Pug templates, and SCSS styling.

## Requirements

- Prefer Vue 3 Composition API.
- Use `<template lang="pug">`.
- Use `<script setup lang="ts">`.
- Use `<style lang="scss" scoped>` unless shared styling is explicitly needed.
- Keep the component readable and reusable.
- Use typed props, emits, and local state when applicable.

## API usage

- If the component needs backend data, prefer a dedicated API/service helper.
- Align request paths with `/api/v1/` conventions.
- Keep loading and error states explicit.

## Scope and quality

- This is a `mode: edit` prompt. Focus on the requested component and the smallest necessary related edits.
- Avoid unrelated project restructuring.
- Keep naming and component responsibilities consistent.

## Output expectations

Produce a component that is ready to drop into a Vue 3 codebase that follows the repository conventions.

