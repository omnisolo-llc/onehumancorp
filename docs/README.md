# OmniSolo Documentation

This directory is the source for the repository documentation site.

## Conventions

- Source documentation lives under `docs/`.
- Source code lives under `src/`.
- Canonical desktop UI is under `src/ui/tauri/`.
- GitHub issues are the task source of truth.
- Superseded material is removed; source history remains available in Git.

## Start Here

- [Documentation Index](index.md)
- [Architecture Hub](technical/architecture/architecture-overview.md)
- [Developer Setup](technical/developer/setup.md)
- [Developer Guide](technical/developer/developer-guide.md)
- [Multi-Harness Compatibility](omnisolo-harness-compatibility-inventory.md)
- [Interactive API Playbook](api/playbook.md)
- [User Guide](user_guide.md)

## Core Capabilities

- [Distributed State Machine](features/kairos/distributed_state_machine.md)
- [AutoDream Pipelines](features/kairos/autodream_pipelines.md)
- [Memory Consolidation](features/kairos/memory_consolidation.md)
- [Sub-Agent Queue Design](technical/architecture/kairos/sub-agent-queue-design.md)
- [Dynamic Workflows](features/dynamic_workflows.md)
- [Hybrid MCP Config Sync](features/hybrid_mcp_config_sync.md)

## Site Generation

The docs website is generated from markdown with MkDocs through Bazel:

```bash
# Build the documentation site
bazelisk run //:docs_build

# Serve documentation locally on http://127.0.0.1:8000
bazelisk run //:docs_serve
```
