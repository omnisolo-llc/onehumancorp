---
status: PENDING
agent: Researcher
---

# Title: Market Audit: Competitive Analysis of Hybrid DB Abstractions for Agentic Contexts

## Problem Statement
Competitors force users into a binary choice: trade privacy for cloud-scale execution, or trade scalability for local privacy. The Replit Agent and OpenClaw models operate purely in the cloud, while Claude Code indexes only local directories. OHC must leverage its Standalone Desktop (SQLite) to Cloud (Postgres) capabilities. An architectural gap exists in the orchestration layer where native operations (S3 vs Local FS) are hardcoded, rather than using mode-aware proxy interfaces for MCP.

## Research Report
A deep analysis of the OHC Hybrid architecture vs. Cloud-only alternatives.
- **Claude Code**: Single-user, CLI-centric. No persistent swarm context.
- **OpenClaw**: Cloud-orchestrated, rigid APIs. Lacks private standalone fallback.
- **OHC Vision**: A unified data layer where the same application binaries run locally backed by SQLite and Local FS, but automatically synchronize "Omni-Context" payloads to K8s Postgres and S3 when swarm scaling is required.
See also: `RESEARCH_REPORT_HYBRID_RAG.md`

## Design Doc
We need to design a "Hybrid Blob Storage Proxy MCP".
- **Local Mode**: Uses local file system `/var/tmp/ohc/blobs`.
- **Cloud Mode**: Uses AWS S3 `ohc-multi-tenant-blobs`.
- Both must be exposed behind an `mcp.BlobProvider` interface so that Agent tools execute identical code regardless of the underlying target.

## Implementation Prompt
Hello Implementer agent!
1. Please review the current MCP server integrations in `srcs/server/agents/`.
2. Abstract the file writing logic behind an interface `mcp.BlobProvider`.
3. Implement `LocalBlobProvider` and `S3BlobProvider`.
4. Ensure the factory selects the correct provider based on `OHC_STANDALONE` or `OHC_MULTITENANT` environment variables.

## Priority
P1

## Estimated Scope
Medium
