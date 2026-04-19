
# Title: Scout (Resource Scout & Tool Integrator)

## Problem Statement
The OHC Hybrid Agentic OS lacks a dedicated sub-agent capable of actively discovering, profiling, and integrating new APIs, tools, or resources at runtime (Dynamic MCP). Currently, agents fail when they encounter unknown tool requirements, reducing their absolute autonomy. A "Resource Scout" is needed to search the internal tool registry and public internet to synthesize missing tools.

## Research Report
An audit of the OHC orchestration layer reveals a gap in dynamic tool acquisition. Agents are limited to pre-configured tools. A "Scout" agent would leverage the Dynamic Tool Discovery MCP (`[integrations]_hybrid_dynamic_tool_discovery_mcp.md`) and the Hybrid Vector DB MCP to parse API documentation, generate new MCP bundles, and dynamically inject them into the Sub-Agent Queue.

## Design Doc
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

**Scout Architecture Overview**
- **Discovery Engine**: Polls internal `ToolRegistry` and external registries.
- **Synthesizer**: Uses LLM to map API specs (OpenAPI/Swagger) into MCP definitions.
- **Integrator**: Submits the generated bundle via `bazelisk` or dynamic linking.

</div>

## Implementation Prompt
Hello Implementer agent! Your task is to implement the Scout sub-agent.
1. Create a new worker in `srcs/server/agents/scout/`.
2. Implement a loop that queries the `ToolRegistry` for missing capabilities.
3. Use the Hybrid RAG API to synthesize new MCP tools based on missing capability requests.
4. Add robust E2E testing to ensure the Scout successfully discovers and integrates a mock tool.

## Priority
P1

## Estimated Scope
Large
