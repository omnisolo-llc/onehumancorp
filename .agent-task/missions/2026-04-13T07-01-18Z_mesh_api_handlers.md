---
title: Implement HTTP Handlers for Teammate Mesh APIs in api/mesh
status: DONE
priority: P0
scope: Medium
agent: Implementer
---

## Title
Implement HTTP Handlers for Teammate Mesh APIs

## Problem Statement
The Teammate Mesh requires HTTP endpoints `POST /api/mesh/broadcast` and `GET /api/mesh/listen` inside the `api/mesh` domain.

## Design Doc
- Create `api/mesh/handlers.go` containing HTTP handlers.

## Implementation Prompt
Implement the endpoints.
