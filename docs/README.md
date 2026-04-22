# One Human Corp Documentation

This directory is the source for the repository documentation site.

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

## Hybrid Agentic OS Architecture

OneHumanCorp (OHC) utilizes a powerful **Hybrid Agentic OS Architecture** designed to seamlessly bridge cloud reliability with edge autonomy.

- **Cloud-Native Mode**: Powered by Kubernetes, PostgreSQL, and Redis for highly available multi-tenant environments.
- **Standalone Desktop Mode**: Zero-infrastructure fallback leveraging SQLite for local state management.
- **Teammate Mesh**: Peer-to-peer event synchronization connecting multiple agents across edge devices.
- **AutoDream Pipelines**: Asynchronous data consolidation bridging edge SQLite telemetry back to Cloud vector stores.

For a comprehensive guide, see the [Hybrid Architecture Documentation](features/hybrid-architecture.md).

## KAIROS Features
- [Sub-Agent Queue](features/kairos/sub_agent_queue.md)
- [Distributed State Machine](features/kairos/distributed_state_machine.md)
- [AutoDream Pipelines](features/kairos/autodream_pipelines.md)

## Site Generation

The docs website is generated from markdown with MkDocs.

```bash
python3 -m pip install -r docs/requirements.txt
mkdocs serve
```
