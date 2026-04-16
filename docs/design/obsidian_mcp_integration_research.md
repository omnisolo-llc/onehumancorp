Parent: #4296

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# [integration] Implement Obsidian MCP for Local-First Knowledge Base Synchronization

## Problem Statement
OHC agents operating in Standalone Desktop Mode lack native access to users' private, local-first knowledge bases. While Replit Agent and Claude rely on cloud document stores, OHC needs a privacy-first integration that works offline and synchronizes contextually relevant data to the Cloud Swarm only when required.

## Research Report
Obsidian is the market leader for local, Markdown-based knowledge management. It stores files locally, making it an ideal candidate for OHC's SQLite-backed Standalone Mode. By implementing an Obsidian MCP (Model Context Protocol), local OHC agents can execute RAG against private notes. When Cloud Escalation is needed, the existing Hybrid MCP Sync Queue can replicate only the computed embeddings or relevant context vectors to the cloud Postgres orchestration engine via SPIFFE mTLS, maintaining strict data sovereignty.

## Design Doc
- **MCP Integration**: Add an `obsidian` module to `srcs/server/integrations/`.
- **Schema Update**: Rely on the existing `hybrid_mcp_sync_queue` to buffer Obsidian context metadata locally in SQLite during Standalone mode.
- **Provider Interface**: Implement a provider that reads local `.md` files in Standalone mode and mocks the interface in Cloud mode.

## Implementation Prompt
Hello Implementer agent!
1. Create a new package `srcs/server/integrations/obsidian/` containing `provider.go`.
2. Implement the `IntegrationProvider` interface for Obsidian, capable of reading local Markdown files.
3. Integrate the new provider into `srcs/server/integrations/catalog.go`.
4. Write tests in `provider_test.go` and create appropriate Bazel targets (`BUILD.bazel`) for the package. Verify with `bazelisk test //srcs/server/integrations/obsidian/...`.

## Priority
P2

## Estimated Scope
Medium

</div>
