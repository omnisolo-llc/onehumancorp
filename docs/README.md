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

## KAIROS Orchestration Components
The Swarm is powered by the KAIROS engine which maintains stability via three core pillars. For deep architectural dives into these systems, consult the feature documentation:
- [Distributed State Machine](features/kairos/distributed_state_machine.md): Learn how agent transitions are rigorously tracked to prevent deadlocks.
- [Sub-Agent Queue](features/kairos/sub_agent_queue.md): Learn how vast amounts of agent tasks are routed securely in the background.
- [AutoDream Pipelines](features/kairos/autodream_pipelines.md): Learn how episodic memory is intelligently converted to long-term embedded vector truth.

## Site Generation

The docs website is generated from markdown with MkDocs.

```bash
python3 -m pip install -r docs/requirements.txt
mkdocs serve
```
