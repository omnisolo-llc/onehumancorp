issue_title: "Implement the Hybrid Agentic OS Extensibility Layer"
issue_description: |
  # Research Report: Hybrid Agentic OS Extensibility Layer

  ## Mission Queue Protocol Brief
  We need to bridge the gap between OHC’s current multi-tenant architecture and an open ecosystem of agentic workflows by designing an **Extensibility Layer** that allows third-party capabilities, custom agent skills, and seamless MCP (Model Context Protocol) bundles to securely plug into the OHC OS.

  ## Problem Statement
  Currently, OHC’s agents (like `OnboardingAgent`) have hardcoded capabilities and toolsets. Maya the baker and Carlos the handyman have specific needs (e.g., custom local delivery integrations or specialized quoting tools) that OHC cannot natively support for every single niche. To be a true "Work Assistant OS," OHC must allow users to install new "skills" and integrations without needing an OHC core engineering release.

  The lack of a dynamic plugin architecture restricts OHC’s ability to scale horizontally into long-tail SMB use cases, pushing owners back to fragmented, non-agentic toolchains like Shopify App Store + Zapier + ChatGPT.

  ## Research Report
  - **Shopify & Wix**: They scale via App Stores. However, their apps often result in a disjointed UX (the "App Tax" and app fragmentation).
  - **HubSpot**: Breeze agents integrate tightly, but the extensibility is limited to CRM data schemas.
  - **MCP (Model Context Protocol)**: An emerging standard for AI tools. By natively supporting MCP, OHC could instantly tap into a growing ecosystem of community-built tools.
  - **Finding**: OHC needs a **Skill/Plugin Registry** that registers MCP-compliant tools and binds them to specific tenant agent roles at runtime, using a zero-trust model.

  ## Design Doc
  ### Mobile UX Flow
  - Maya opens OHC on her 375px phone.
  - She navigates to **"Assistant Settings" -> "Skills"**.
  - The UI (glassmorphism cards) presents a marketplace of skills (e.g., "Local Courier Delivery Tracker").
  - Maya taps **"Enable"**. The system dynamically registers the new MCP bundle for her tenant.
  - She immediately tells her Work Triage agent: "Check where the cake for order #123 is." The agent seamlessly uses the newly injected tool.

  ### AI Agent Integration Points
  - The KAIROS Orchestrator or central Agent runtime dynamically fetches the registered `tools` for the current tenant before invoking the LLM provider (Gemini/MiniMax).
  - A secure execution sandbox (or proxy) intercepts tool calls and routes them to the registered MCP bundle or remote webhook.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD;
      User[Maya / Carlos] -->|Chat/Voice| OHC_Shell[OHC Mobile App];
      OHC_Shell --> KAIROS[KAIROS Orchestrator];

      subgraph Extensibility Layer
          SkillRegistry[(Tenant Skill Registry DB)]
          MCPGateway[MCP Gateway / Sandbox]
      end

      KAIROS -->|Fetch Active Skills| SkillRegistry;
      KAIROS -->|Invoke LLM with Dynamic Tools| LLM[LLM Provider];
      LLM -->|Tool Call| KAIROS;
      KAIROS -->|Execute Tool| MCPGateway;
      MCPGateway -->|API Request| ExternalService[External Vendor API];
  ```

  ## Implementation Prompt
  **To the Implementer:**
  Implement the backend infrastructure and the mobile-first UI for the "Agent Skills" extensibility layer.
  1. **Backend**: Create a mechanism to register, store, and fetch dynamic tools (skills) per tenant. The KAIROS orchestrator must dynamically inject these tools into the agent's context when communicating with the LLM.
  2. **Frontend (Tauri/Flutter)**: Build a 375px-optimized "Skills Marketplace" UI where an owner can view, enable, and disable skills. Use the OHC Premium Token library (Translucent Glass).
  3. **Verification**: Write Playwright E2E tests proving that an owner can enable a mock skill and successfully trigger it via natural language in the chat interface. Do not use mocked UI data; all state must be persisted and served by the real backend.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []