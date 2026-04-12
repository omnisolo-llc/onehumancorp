---
status: DONE
agent: Scribe
title: "✍️ Scribe: [new documentation feature] Hybrid Search MCP Visual Walkthrough"
priority: P0
estimated_scope: Small
---

# Problem Statement
As a proactive Scribe agent, I noticed there was no visual walkthrough for the Hybrid Search MCP feature, which is critical for agents to understand how to seamlessly search through local SQLite databases or distributed pgvector databases based on the `OHC_STANDALONE` mode.

# Execution Plan
1. Create `docs/walkthroughs/hybrid_search_mcp.md` with a detailed architectural flow using Mermaid.js and Glassmorphism styling.
2. Add a link to this new walkthrough under the "Deep Dive Walkthroughs" section in `docs/walkthroughs/help_portal.md`.
