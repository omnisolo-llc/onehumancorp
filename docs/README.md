# One Human Corp Documentation

This directory is the source for the repository documentation site.

## Conventions

- Source documentation lives under `docs/`.
- Source code lives under `src/`.
- GitHub issues are the task source of truth.
- Historical or superseded material belongs in `docs/archive/`.

## Start Here

- `docs/index.md`
- `docs/architecture/index.md`
- `docs/developer/index.md`
- `docs/operations/index.md`

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
