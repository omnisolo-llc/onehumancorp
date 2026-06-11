issue_title: "Implement Dynamic Tool Discovery via MCP & SPIFFE"
issue_description: |
  # Research Report: Dynamic Tool Discovery via MCP & SPIFFE

  **Author:** Principal Product Researcher
  **Status:** Approved

  ## 1. Problem Statement
  **The Pain Point:** Currently, agents in existing AI frameworks (like AutoGen and CrewAI) are tightly coupled to a static list of tools injected at initialization. When a novel problem arises that requires a new tool, the agent fails out, creating a high-friction loop. This negatively impacts the capability of our agents to act autonomously and accurately to solve owner operations problems efficiently.

  ## 2. Competitive Landscape & Research
  - **Tool Constraints:** Agents fail when a novel task demands a tool not present in the pre-defined bootstrap prompt.
  - **Solution Strategy:** Native "Just-In-Time" tool synthesis workflow via One Human Corp's K8s/LangGraph architecture, leveraging the Model Context Protocol (MCP) Gateway and Zero-Trust SPIFFE/SPIRE authentication. This unlocks flexible scaling without heavy prompt loading.

  ## 3. Architectural Design

  ### 3.1 Components

  1. **The Tool Registry API:** The Switchboard exposes an internal `/v1/tools/search` endpoint.
     - **Semantic Matching:** Agents can send a natural language description (e.g., "I need a tool to query AWS S3 bucket policies"). The Switchboard queries a vector database containing OpenAPI specifications of all internal and external registered MCP tools.
     - **Synthesis Response:** The API responds with the `tool_name`, `schema_url`, and required `RBAC` role to execute the tool.

  2. **Zero-Trust Tool Access (SPIFFE):**
     - **Role Negotiation:** When the agent discovers a new tool, it must request temporary access.
     - **SPIRE Integration:** The agent sends a request to the K8s Operator, which verifies the agent's current task (`thread_id`) against the tool's required permissions.
     - **Short-Lived SVID Generation:** If approved, SPIRE issues a short-lived x509 SVID (certificate) specifically authorizing the agent to call that tool for the next 15 minutes.

  3. **The LangGraph Node:**
     - We introduce a `DynamicToolDiscovery` node directly into our standard LangGraph execution template.
     - **Execution Failure Recovery:** If a standard tool call fails with a `ToolNotFound` or `NotImplemented` error, the execution graph automatically routes to the `DynamicToolDiscovery` node.
     - **Autonomous Fetch:** The agent pauses its main reasoning loop, fetches the new tool schema from the Switchboard, incorporates the strict JSON schema into its prompt, and re-enters the execution loop to retry the action.

  ## 4. Mobile UX Flow & Performance
  - **Lean Prompts:** Agents boot with an extremely minimal set of core tools (e.g., `search_tools`, `write_file`). All specific implementation tools are discovered lazily on a per-task basis.
  - **Latency Optimization:** The `/v1/tools/search` endpoint must respond in sub-50ms using optimized vector retrieval (Sync.Pool buffer reuse), ensuring that the dynamic discovery loop does not noticeably slow down the execution graph. This translates directly to an instant, "magical" experience for our non-technical owners when they request complex tasks via the mobile 375px app shell.

  ## 5. Implementation Prompt
  **Goal:** Implement the Just-in-Time Tool Synthesis and Dynamic Tool Discovery logic.
  - Implement the `/v1/tools/search` Tool Registry API on the Switchboard to query against a mocked tool specification database based on natural language queries.
  - Integrate a temporary SPIRE-based authorization mock endpoint for validating `thread_id` and generating short-lived access SVIDs.
  - Add a `DynamicToolDiscovery` failure recovery node in the existing standard LangGraph workflow template that catches `ToolNotFound` and queries the Switchboard registry.
  - Write Playwright E2E and Unit tests asserting that when an agent is given an initial prompt lacking a necessary tool, it correctly hits the discovery endpoint, negotiates an SVID, and successfully resolves the prompt.

  ## 6. Priority & Scope
  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
