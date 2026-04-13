<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid MCP RAG Protocol: Visual Walkthrough

This guide details the architectural flow of the Hybrid Model Context Protocol (MCP) RAG Protocol, illustrating how OHC agents seamlessly transition between local document indexing and distributed enterprise search.

## Overview

The Hybrid MCP RAG system ensures that agents have a unified search context regardless of whether they are operating in Standalone Desktop Mode or Cloud-native Mode.

```mermaid
graph TD
    Agent[OHC Agent] --> |Search Request| MCP[Unified MCP Interface]

    MCP --> |Decision Engine| Router{Environment Check}

    Router -->|Standalone Mode| Local[Local Indexing (SQLite FTS/Vector)]
    Router -->|Cloud Mode| Dist[Distributed Search (PostgreSQL pgvector / Web APIs)]

    Local --> Result[Unified Search Result]
    Dist --> Result

    Result --> Agent

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Agent,MCP,Router,Local,Dist,Result premium;
```

## API Interactions

Agents interface with the RAG system via the standard MCP API endpoints, simplifying integration.

### Example Search Request

```json
{
  "jsonrpc": "2.0",
  "method": "search",
  "params": {
    "query": "architecture decisions for OHC-SIP",
    "limit": 5
  },
  "id": 1
}
```

The system autonomously resolves the query using the configured backend.

## Architectural Flow

1. **Query Generation:** The Agent issues a semantic query.
2. **Context Resolution:** The MCP Server determines the operational mode.
3. **Retrieval:**
   - *Standalone:* Queries the local `.agent-task/swarm.db` using SQLite vector extensions.
   - *Cloud:* Connects to the centralized PostgreSQL instance with `pgvector` for enterprise-wide knowledge retrieval.
4. **Synthesis:** The retrieved context is formatted and returned to the Agent for generation.

</div>
