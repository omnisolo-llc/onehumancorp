# [Architecture] Issue Brief: Dynamic Tool Discovery via Model Context Protocol (MCP)

**Title:** Dynamic Tool Discovery via Model Context Protocol (MCP)

**Problem Statement:**
Currently, when a small business owner (like Carlos, the Freelance Handyman) needs his AI assistant to check local weather before booking an outdoor repair, or when a boutique owner (like Priya) needs her AI to pull inventory from a niche supplier's API, the AI fails unless an OHC engineer has explicitly hardcoded that specific integration. Our agents are trapped by static tool schemas, making the platform inflexible for the diverse, long-tail needs of small businesses. We need a way for agents to dynamically discover and use external tools at runtime.

**Research Report:**
*   **Competitor Audit:** Traditional builders like Shopify rely heavily on App Stores, creating "subscription hell" (Cost Creep is a Top 10 SMB Pain Point) and requiring manual user configuration.
*   **Framework Ingestion Data:** Leading AI frameworks (OpenClaw, CrewAI, AutoGen) still rely heavily on static binding schemas for tool definitions.
*   **Opportunity:** Adopting a Model Context Protocol (MCP) approach, secured by OHC's existing zero-trust infrastructure, allows agents to search registries and dynamically bind to new tools at runtime, enabling infinite extensibility without platform bloat or hardcoded schemas.

**Design Doc:**
*   **High-Level Architecture:**
    *   **MCP Gateway (Switchboard):** A centralized registry service where internal OHC tools and approved third-party integrations expose their capabilities via a standardized protocol (MCP).
    *   **SPIFFE/SPIRE Security:** Runtime tool synthesis and RPC endpoint discovery are secured and RBAC-enforced using our existing identity mesh.
    *   **Agent Execution Flow:** When an agent encounters a goal it lacks tools for, it pauses, queries the MCP Gateway (`discover_tools`), dynamically imports the required tool schema, and resumes execution.
*   **UI/UX (Mobile-First):**
    *   For the user, this is mostly invisible. If a tool requires authentication (e.g., connecting a custom supplier API), a simple, 1-tap "Connect Provider" card appears in their Action Feed.
*   **AI Agent Integration:** The system prompt for core routing agents must include instructions to query the `discover_tools` endpoint when faced with unknown constraints.

**Implementation Prompt:**
Implement the Model Context Protocol (MCP) Gateway and dynamic discovery mechanism. The system must allow an agent to query a registry, retrieve a tool's OpenAPI schema or execution contract, and dynamically invoke it during runtime. The solution must utilize our existing SPIFFE/SPIRE infrastructure for zero-trust authentication between the agent runtime and the tool endpoints. Create a proof-of-concept tool (e.g., a simple external data fetcher) and an E2E test where an agent successfully completes a task using this dynamically discovered tool. Do not prescribe specific database schemas or API contracts; design for flexibility and security.

**Priority:** P0

**Estimated Scope:** Large
