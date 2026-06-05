issue_title: "Architect Unified AI-Native Customer Inbox & Triage Engine"
issue_description: |
  # Research Report: Unified AI-Native Customer Inbox & Triage Engine

  ## Problem Statement
  Small business owners, particularly "Social Sellers" like Maya (The Home Baker) and Priya (The Boutique Owner), suffer from "Instagram DM Overload" (Pain Point #2). They miss sales because they cannot reply to DMs fast enough while working, or they lose track of inquiries spread across WhatsApp, Instagram, and Email. Existing tools still require the user to manually draft and send replies, failing to alleviate the operational friction.

  ## Research Report
  ### Competitive Analysis
  - **Shopify Inbox:** Centralizes messages but relies on basic macro responses. Sidekick AI helps the merchant internally, but does not autonomously converse with the customer on their behalf.
  - **Wix Inbox:** Standard multi-channel inbox. No autonomous AI capabilities.
  - **Squarespace & GoDaddy:** Basic centralized messaging, lacking any proactive AI automation.

  ### The OHC Opportunity
  To capture the SMB market, OHC must build an **invisible, native AI Auto-Responder & Triage Engine**. The system must intercept customer queries across channels, identify intent (e.g., "Where is my order?", "Do you do vegan cakes?"), check OHC's internal systems (inventory, order status), and reply automatically. The merchant only intervenes when the AI flags a complex issue or requires explicit approval.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer (IG/WhatsApp/Email)] -->|Message| B(Omnichannel Gateway);
      B --> C[AI Customer Success Dept];
      C -->|RAG Query| D[(Tenant Embedded Memory / Order DB)];
      C -->|Determine Intent| E{Is Action Safe?};
      E -- Yes --> F[Draft & Send Auto-Reply];
      F --> B;
      E -- No / Needs Approval --> G[Unified Inbox Triage];
      G -->|Push Notification| H[Merchant Mobile App (375px)];
      H -->|1-Tap Approve/Edit| G;
      G --> F;
  ```

  ### UI Wireframes & Mobile UX Flow (375px Viewport)
  - **The "Triage" Feed:** The merchant's dashboard does not just list chronologically ordered messages. It displays "Cards".
  - **Card Type A (Handled):** "Replied to 4 customers about shipping times." (Read-only, instills confidence).
  - **Card Type B (Action Required):** "New custom order request from @sarah. Drafted quote for $50." [Approve & Send] | [Edit]
  - **Glassmorphism UI:** Adhering to the OHC Premium Token library, the inbox must utilize `backdrop-filter: blur(20px)` for a native macOS-like feel.

  ### AI Agent Integration Points
  - **Customer Success Agent:** Acts as the primary auto-responder using RAG over the tenant's past interactions and business context.
  - **Sales & Acquisition Agent:** Steps in when an intent to purchase or get a quote is detected, generating draft quotes for merchant approval.

  ### Key Design Decisions
  - **Zero-Trust Multi-Tenancy:** All inbound messages and generated embeddings are strictly isolated using PostgreSQL RLS (`tenant_id`).
  - **Autonomous by Default:** The CRM Agent must prioritize sending safe, automated replies over bothering the merchant, dramatically reducing time spent on customer support.
  - **Unified Capacity Mesh Integration:** The inbox must directly connect with the Conversational Checkout Engine to allow customers to book services directly inside the chat thread.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the Unified AI-Native Customer Inbox & Triage Engine backend services.
  - **User Journey (CUJ):** A customer sends a DM asking "Are you open on Sundays?". The system intercepts the webhook, queries the tenant's profile, and the AI agent automatically replies "Yes, 10 AM to 4 PM!" without notifying the merchant. A second customer asks for a custom quote; the AI drafts the quote and pushes an approval card to the merchant's mobile feed.
  - **Acceptance Criteria:**
    1. Create the backend data models for `UnifiedMessage`, `Thread`, and `AI_Triage_State`.
    2. Implement the Omnichannel Gateway to standardize incoming payloads.
    3. Integrate the AI Customer Success Agent to classify intents and generate draft replies based on tenant embedded memory.
    4. Ensure strict tenant isolation (`tenant_id`) on all new tables and queries.
    5. Provide an E2E test verifying both the auto-reply flow and the merchant-approval flow.

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []