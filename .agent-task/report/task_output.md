issue_title: "Implement the Unified Agent Feed (Mobile MVP)"
issue_description: |
  # Unified Agent Feed Implementation Plan

  **Problem Statement:**
  OHC currently lacks a unified feed where business owners can instantly see, approve, or reject actions proposed by various agents (Operations, Customer Success, Marketing, etc.). Mobile usability is paramount.

  **Research Report:**
  We researched leading work assistant tools like Shopify Sidekick and HubSpot Breeze. Their weakness is an inability to easily approve complex workflows directly from a mobile device (375px width). An "Action Card" system will streamline agent-to-human workflows. The agent feed should ingest events and LLM-generate actionable drafts.

  **Design Doc:**
  - **Architecture Diagram:**
    ```mermaid
    graph TD;
        EventBus --> AgentFeedService;
        AgentFeedService --> LLM[Minimax/Gemini Intent & Draft Gen];
        LLM --> Database[agent_feed_items Table];
        Database --> API[REST/WebSocket Endpoint];
        API --> MobileApp[Unified Agent Feed UI];
    ```
  - **Mobile UX Flow:**
    The user logs in, accesses the Agent Feed. The feed displays a stack of vertically aligned cards. Each card contains a descriptive text of the issue/event and simple action buttons (e.g. Approve, Edit, Discard).
  - **AI Integration:**
    `AgentFeedService` receives webhook events, queries the LLM for intent classification and draft response.
  - **Key Decisions:**
    Build a simple React/Tauri component with responsive (375px base) Tailwind/CSS styling, hooking into real backend API data instead of mocks.

  **Implementation Prompt:**
  Build out the `Unified Agent Feed` component in `src/ui/tauri/src/ui/dashboard.html` (or Next.js if applicable, though Tauri is canonical). Create the SQL schema if missing for `agent_feed_items`, wire the Rust backend API for creating and fetching items, and ensure Playwright E2E tests reflect this new component. The UI MUST use 375px mobile-first design, 44x44px touch targets minimum, and "Glassmorphism" design tokens.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
