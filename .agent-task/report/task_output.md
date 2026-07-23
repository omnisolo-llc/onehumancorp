issue_title: "[Architecture] Implement Mode-Aware Hybrid MCP Proxies"
issue_description: |
  # Research Report: Mode-Aware Hybrid MCP Proxies

  ## 1. Problem Statement
  As identified in `hybrid-mcp-proxy-report.md`, current Agentic OS implementations suffer from rigid, environment-locked integrations. For example, agents operating on local file systems fail when executed in a cloud context because storage paradigms differ (Local FS vs Cloud S3). To maintain OHC's competitive advantage and ensure agents are completely portable across execution tiers (Standalone Desktop vs Multi-tenant Cloud), we must decouple the tool execution logic from the underlying storage mechanism.

  ## 2. Research Report
  Our competitive market audit evaluated Claude Code, OpenClaw, and Replit Agent against our vision for OneHumanCorp.

  *   **Claude Code** relies strictly on local file systems, making cloud execution impossible without significant rewrites.
  *   **OpenClaw** relies heavily on cloud storage (S3), rendering its agents useless when local/offline execution is required.
  *   **OHC Vision:** Agents interact with a unified `mcp.BlobProvider` or `mcp.FSProvider` proxy. Depending on the environment variables (`OHC_STANDALONE=true` vs `OHC_MULTITENANT=true`), the exact same agent logic is routed seamlessly to the appropriate backend (Local FS vs AWS S3).

  ## 3. Design Doc
  ### 3.1 Architecture Model
  We will introduce a **Mode-Aware MCP Gateway** that sits between the agent tools and the storage providers.

  ```mermaid
  graph TD
      Agent[Agent Execution Layer] -->|JSON-RPC| MCP_Proxy[Mode-Aware MCP Proxy]
      MCP_Proxy -->|OHC_STANDALONE=true| LocalFS[Local File System]
      MCP_Proxy -->|OHC_MULTITENANT=true| S3[Multi-Tenant Cloud S3 Bucket]
  ```

  ### 3.2 Mobile UX Flow
  For the non-technical small business owner, this means zero configuration changes when they scale from a single-device local business to a cloud-managed multi-location enterprise. The setup is completely hidden. The UI provides a fluid, macOS Translucent Glass aesthetic where users simply see their files and assets, completely unaware of the underlying tier changes.

  ### 3.3 AI Agent Integration Points
  - **Tool Interfaces:** AI agents utilize unified tool interfaces (e.g., `blob.read`, `blob.write`, `fs.list`).
  - **Proxy Routing:** The OHC Hub and MCP Gateway dynamically route requests based on cluster configuration, ensuring SPIFFE-gated identity and authorization are enforced regardless of the storage backend.

  ## 4. Implementation Prompt
  **User-Facing Outcome:** Business owners experience uninterrupted workflows regardless of deployment mode. An agent generating a quote (PDF) seamlessly stores it in the correct location (local disk or S3) without the agent needing distinct code paths.

  **Acceptance Criteria:**
  1. Define unified MCP interfaces for Blob and FileSystem operations.
  2. Implement a routing mechanism that inspects `OHC_STANDALONE` vs `OHC_MULTITENANT` and dispatches calls to the appropriate driver.
  3. All existing tools utilizing direct S3 or Local FS must be migrated to the new Mode-Aware MCP Proxy.
  4. 100% unit test and Playwright E2E coverage demonstrating identical agent behavior across both deployment modes.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
