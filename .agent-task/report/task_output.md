issue_title: "Omnichannel AI DM Auto-Responder and Work Intake System"
issue_description: |
  ## Title
  Omnichannel AI DM Auto-Responder and Work Intake System

  ## Problem Statement
  Small business owners like Maya (the baker) receive custom order inquiries across multiple channels (Instagram DMs, WhatsApp, Facebook Messenger, SMS). Monitoring these channels 24/7 is impossible, leading to missed opportunities and delayed responses. Current solutions require manual triage or complex third-party tools that don't deeply integrate with the owner's operational context, calendar, or inventory. They need an AI agent that can converse naturally with customers, check product availability, answer FAQs (e.g., "Do you do vegan cakes?"), and automatically triage inquiries into actionable order deposits or booking tasks directly from their mobile device.

  ## Research Report
  - **Market Context:** Platforms like ManyChat or Intercom provide chatbot capabilities but require complex logic-tree setups that are too technical for non-technical owners. Shopify Inbox offers basic automated replies but lacks autonomous conversational intelligence integrated with order creation.
  - **Competitor Gaps:** Most SMB conversational tools act merely as FAQ responders and fail to transition a conversation into a concrete business action (like a deposit payment link or calendar booking).
  - **The OHC Opportunity:** By leveraging our central orchestrator (KAIROS) and LLM-backed built-in agents, OHC can provide an out-of-the-box Customer Success Agent that securely reads tenant-scoped memory and product catalogs to instantly negotiate custom orders via DMs.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph External Channels
          IG[Instagram DMs]
          WA[WhatsApp]
          SMS[SMS]
      end

      subgraph OHC Mesh & Ingress
          WH[Webhook Gateway]
          MQ[KAIROS Shared Task List]
      end

      subgraph AI Department Coordination
          CS_Agent[Customer Success Agent]
          OP_Agent[Operations Agent]
          FI_Agent[Finance Agent]
      end

      subgraph Core Data
          T[(Tenant Knowledge/Memory)]
          I[(Inventory/Catalog)]
      end

      IG --> WH
      WA --> WH
      SMS --> WH
      WH --> MQ

      MQ -->|New Message Task| CS_Agent
      CS_Agent -->|Read FAQ/Catalog| T
      CS_Agent -->|Check Availability| I

      CS_Agent -->|Draft Reply/Send| WH
      CS_Agent -->|Propose Deposit| FI_Agent
      CS_Agent -->|Create Order Task| OP_Agent
  ```

  ### Mobile UX Flow (375px first)
  1. **Triage Feed (Owner View):** When Maya opens the OHC app, she sees a "Unified Agent Feed." A card shows: "3 new Instagram inquiries handled. 1 pending custom order deposit approval for vegan cake."
  2. **Approval Interface:** Tapping the card opens a detailed view showing the conversation summary and the proposed deposit invoice drafted by the Finance Agent.
  3. **One-Tap Action:** A large, 44x44px "Approve & Send Deposit Link" button is prominently displayed.
  4. **Settings/Onboarding:** In advanced settings, Maya can toggle connected channels via simple OAuth flows without configuring API keys.

  ### AI Agent Integration Points
  - **Work Triage:** Groups incoming webhooks from external channels and assigns them to the Customer Success Agent.
  - **Customer & Relationship Assistant (CS Agent):** Uses tenant-scoped memories and catalog context to draft context-aware replies. If a request is complex, it escalates to the owner's feed.
  - **Finance & Operations Agents:** Invoked by the CS Agent when the customer agrees to proceed, generating a Stripe deposit link and scheduling the order in the calendar.

  ### Key Design Decisions
  - **Unified Webhook Gateway:** To abstract external API complexities from the core orchestrator, a unified ingestion service normalizes incoming messages before placing them on the KAIROS queue.
  - **Human-in-the-Loop for Revenue:** While the AI can freely answer FAQs and negotiate, issuing actual payment requests or finalizing custom orders defaults to an Owner Approval Card to maintain trust and safety.
  - **Mobile-First Agent Feed:** Abandon traditional spreadsheet-like inboxes in favor of an actionable, prioritized card feed, adhering to the "Approval" interface paradigm.

  ## Implementation Prompt
  **Target Persona:** Maya the Home Baker
  **User-Facing Outcome:** Connect Instagram DMs to OHC, allow the Customer Success Agent to automatically answer inquiries using her catalog info, and surface actionable custom order requests in a mobile-first feed for her approval.
  **Critical User Journey (CUJ):**
  1. Maya connects her Instagram account via a simple OAuth card in the mobile app.
  2. A customer sends an IG DM asking for a custom vegan cake for next Saturday.
  3. The OHC CS Agent intercepts, checks knowledge base for "vegan" and calendar for "next Saturday", and replies that it is possible, asking for details.
  4. The customer provides details. The CS Agent creates a custom order draft and alerts Maya.
  5. Maya opens the OHC app, sees the summarized request on a 375px viewport, and taps "Approve & Send Deposit".

  **Acceptance Criteria:**
  - Build the unified webhook normalization layer for at least one channel (e.g., simulated Instagram).
  - Implement the CS Agent's ability to read product/catalog context and reply autonomously.
  - Render the "Agent Proposal" card in the 375px UI feed containing the conversation summary and an "Approve" button.
  - Fully tested via Playwright E2E covering the mobile viewport and simulated incoming webhooks.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
