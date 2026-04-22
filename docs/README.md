# One Human Corp Documentation

This directory is the source for the repository documentation site.

## The Hybrid Agentic OS Architecture

One Human Corp is designed with a **Hybrid Agentic OS Architecture**. This means the platform operates in two distinct but unified modes:

- **Cloud-Native Mode**: A robust, horizontally scalable multi-tenant Kubernetes deployment designed for maximum throughput and managed by the central orchestrator.
- **Standalone Desktop Mode**: A powerful local execution model that brings the full capability of OHC to individual machines without relying on centralized infrastructure, utilizing local SQLite fallbacks and the Teammate Mesh for peer-to-peer collaboration.

For a comprehensive overview of how these modes interact, fall back, and synchronize via the AutoDream pipeline, see the **[Hybrid Architecture Guide](features/hybrid-architecture.md)**.

## Conventions

- Source documentation lives under `docs/`.
- Source code lives under `srcs/`.
- GitHub issues are the task source of truth.
- Historical or superseded material belongs in `docs/archive/`.

## Start Here

- `docs/index.md`
- `docs/architecture/index.md`
- `docs/developer/index.md`
- `docs/operations/index.md`

## KAIROS Features
- [Hybrid Architecture](features/hybrid-architecture.md)
- [Sub-Agent Queue](features/kairos/sub_agent_queue.md)
- [Distributed State Machine](features/kairos/distributed_state_machine.md)
- [AutoDream Pipelines](features/kairos/autodream_pipelines.md)

## Site Generation

The docs website is generated from markdown with MkDocs.

```bash
python3 -m pip install -r docs/requirements.txt
mkdocs serve
```
