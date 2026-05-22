# One Human Corp Documentation

This directory is the source for the repository documentation site.

## Conventions

- Source documentation lives under `docs/`.
- Source code lives under `src/`.
- GitHub issues are the task source of truth.
- Historical or superseded material belongs in `docs/archive/`.

## Start Here

- [Documentation Index](index.md)
- [Architecture](technical/architecture/index.md)
- [Developer Guide](technical/developer/index.md)
- [Operations](technical/operations/index.md)

## KAIROS Features

- [Distributed State Machine](features/kairos/distributed_state_machine.md)
- [AutoDream Pipelines](features/kairos/autodream_pipelines.md)
- [Memory Consolidation](features/kairos/memory_consolidation.md)
- [Sub-Agent Queue Design](technical/architecture/kairos/sub-agent-queue-design.md)

## Site Generation

The docs website is generated from markdown with MkDocs.

```bash
python3 -m pip install -r docs/requirements.txt
mkdocs serve
```