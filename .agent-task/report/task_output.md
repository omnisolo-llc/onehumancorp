issue_title: "Implement Autonomous AI Website Generation for SMBs"
issue_description: |
  **Problem Statement**
  Currently, SMBs face overwhelming setup paralysis when onboarding onto digital platforms. Existing competitors like Shopify require 30-60 minutes to configure settings, install apps, and design the layout. The initial blank canvas is terrifying for non-technical users, and traditional AI tools only provide manual-like advice. Maya (the baker) and Carlos (the handyman) need a "Zero-Click Generation" flow where a single prompt creates their storefront, catalog, and DB structure.

  **Research Report**
  Market mapping of competitors such as Durable, Wix, and Shopify's Sidekick reveals a major gap: combining an AI website builder (which creates landing pages) with native operations (bookings, POS, eCommerce). While AI builders like Durable build sites in 30 seconds, they lack deep operational integration. Shopify integrates operations but is overly complex and requires an "App Tax". OHC has a unique opportunity to build an AI agentic flow that asks a single prompt, creates a tailored site, and sets up operational endpoints (like booking forms and catalog tables) instantly on mobile.

  **Design Doc**
  - **Architecture:** We will implement an "Operations Manager" Agent Protocol. A new endpoint, `/api/agent/zero-click-generation`, will accept a single user prompt. The request will be passed to the orchestration hub (KAIROS), which will dispatch sub-agents:
    - *Storefront Agent*: Generates a personalized HTML/CSS layout based on a minimal design system.
    - *Catalog Agent*: Drafts product/service schemas and seeds the database using gRPC endpoints.
    - *Settings Agent*: Configures basic tax and locale settings.
  - **Mobile UX Flow (375px first):**
    1. Welcome Screen: A prominent text field saying, "Describe your business in one sentence" (e.g., "I'm a baker in Austin").
    2. Loading State: A dynamic, Apple-style translucent glass interface showing agents at work (e.g., "Generating catalog...", "Designing storefront...").
    3. Success Screen: A preview of the mobile-first storefront with an "Edit" or "Launch" button.
  - **AI Agent Integration:** The system will use the existing Built-in Agent Harness to create a workflow (`visual_workflow.rs`) orchestrating these specialized agents. The agents will interact securely using the platform's distributed lock and multi-tenant systems.

  **Implementation Prompt**
  Implement the "Zero-Click Generation" onboarding flow for the Tauri desktop and Flutter mobile apps. Introduce a new API endpoint on the Rust backend that triggers the workflow, parsing a user prompt and executing parallel agent tasks to build a complete initial store configuration. Ensure the frontend layout utilizes the new OHC Premium Token library, focusing on mobile parity.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
