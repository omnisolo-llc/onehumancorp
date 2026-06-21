issue_title: "Implement the Ambassador Agent for Instagram DM Auto-Reply"
issue_description: |
  # Research Report: The Ambassador Agent for Instagram DM Auto-Reply

  ## Problem Statement
  Solopreneurs and small business owners, like Maya the Home Baker, frequently miss critical sales opportunities because they are engaged in physical operations (baking, delivering) and cannot monitor social media DMs (Instagram/WhatsApp) in real-time. Existing solutions like ManyChat require building complex logic trees, which is too technical and time-consuming for the non-technical OHC target audience. They need an invisible assistant that drafts context-aware replies for them to simply approve.

  ## Research Report
  - **Competitor Analysis:** Shopify relies on third-party apps like Gorgias or ManyChat for DM automation. These tools are powerful but have a steep learning curve and act as a separate "app tax." Wix offers basic inbox functionality but lacks proactive, AI-driven draft generation based on real-time business context.
  - **The OHC Opportunity:** By natively integrating AI intent classification and RAG (Retrieval-Augmented Generation) with the business's core data (inventory, policies), OHC can provide a zero-configuration "Ambassador" agent.
  - **Persona Fit:** Maya receives a DM asking, "Do you have vegan cakes available for tomorrow?" Instead of answering manually hours later, OHC's Ambassador agent instantly checks her inventory and schedule, drafts a response ("Yes, we have 2 vegan chocolate cakes left! Would you like me to send a deposit link?"), and pushes a notification to her phone for one-tap approval.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant IG as Instagram Graph API
      participant OHC as OHC Webhook Gateway
      participant Agent as Ambassador Agent (LLM)
      participant DB as OHC Context (Inventory/Policies)
      participant UI as OHC Mobile App

      IG->>OHC: Incoming DM Webhook
      OHC->>Agent: Route Message
      Agent->>DB: Query Intent & Context (RAG)
      DB-->>Agent: Return "Vegan Cakes: 2 in stock"
      Agent->>Agent: Generate Draft Reply
      Agent->>UI: Push Notification & Action Card
      UI->>UI: Maya taps "Approve"
      UI->>OHC: Approval Signal
      OHC->>IG: Send Reply
  ```

  ### Mobile UX Flow (375px)
  1. **Notification:** Maya receives a native push notification: "New DM from @customer. Draft reply ready."
  2. **Action Card:** Tapping opens the OHC app to an "Action Card" in the Agent Feed.
     - **Card Content:** Shows the original DM and the AI-drafted reply.
     - **Actions:** Three prominent, touch-friendly buttons (≥ 44x44px): `Approve & Send`, `Edit`, `Discard`.
  3. **Edit Mode:** If `Edit` is tapped, a native mobile keyboard opens with the drafted text pre-filled.
  4. **Confirmation:** Upon sending, the card transitions to a "Sent" state with a subtle, premium translucent glass effect.

  ### AI Agent Integration Points
  - **Intent Classification:** The LLM first categorizes the incoming message (e.g., Pricing, Availability, Support).
  - **RAG Context Retrieval:** Based on the intent, the system retrieves relevant data (e.g., checking PostgreSQL for active `Product` inventory or FAQ entries).
  - **Draft Generation:** The LLM generates the response, adhering to the business's configured tone.

  ## Implementation Prompt
  **Feature Name:** The Ambassador Agent (Instagram DM Auto-Reply)
  **Target Persona:** Maya the Home Baker
  **Outcome:** Implement an end-to-end flow where an incoming webhook (simulating an Instagram DM) triggers the Ambassador Agent to draft a context-aware response based on the tenant's inventory and policies. This draft must surface in the OHC mobile UI as an actionable card for the owner to approve, edit, or discard.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. **Webhook Ingestion:** Create an endpoint to receive incoming messages (mocked IG Graph API payload for now).
  2. **Agent Processing:** Implement the logic to pass the message to the LLM (Gemini/MiniMax), retrieving tenant-specific context (inventory/FAQs) to inform the response.
  3. **UI Integration:** Develop the 375px mobile-first "Action Card" in the Agent Feed that displays the incoming message and the drafted reply.
  4. **User Action:** Implement the `Approve`, `Edit`, and `Discard` flows, ensuring the final action sends the appropriate payload back to the external service (or a mocked sink).
  5. **Verification:** Write comprehensive unit tests for the agent logic and a Playwright E2E test simulating a customer message and the owner's approval flow.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
