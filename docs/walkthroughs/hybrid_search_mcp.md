<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 24px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #ffffff; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);">

# Hybrid Search MCP Server: Visual Walkthrough

The Hybrid Search MCP Server provides agents with a unified search capability that adapts seamlessly between Cloud and Standalone modes.

## Architecture Flow

```mermaid
graph TD
    A[Agent Search Request] --> B{Hybrid MCP Server}
    B -->|Cloud Mode| C[CloudSearchProvider]
    C --> D[(PostgreSQL pgvector / Web API)]
    B -->|Standalone Mode| E[LocalSearchProvider]
    E --> F[(SQLite FTS5)]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F premium;
```

## Key Benefits

- **Unified Interface:** Agents use a single `unified_search` tool.
- **Data Privacy:** Local mode ensures no outbound queries occur, relying exclusively on local databases.
- **Multi-Tenant Security:** Cloud mode scopes searches to explicit `OrganizationID` boundaries.

</div>
