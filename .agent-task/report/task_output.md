issue_title: "Implement the Intelligent Customer Auto-Responder (P0)"
issue_description: |
  ## Problem Statement
  Small business owners (e.g., Maya the baker, Carlos the handyman) are overwhelmed by repetitive customer inquiries across Instagram DMs, WhatsApp, and email ("Where is my order?", "Do you do vegan cakes?"). They miss sales because they cannot reply fast enough while doing the actual work. Existing tools either require complex Zapier setups (Shopify) or offer only basic text-match chatbots (GoDaddy), failing to provide an autonomous, context-aware work assistant that acts on behalf of the owner.

  ## Research Report
  Based on our analysis of the SMB platform market and the codebase, there is a clear capability gap in unified, agentic customer communication.
  - **Market Gap**: 38% of surveyed solopreneurs cite "Instagram DM Overload" as a primary pain point. They lack a unified inbox that not only aggregates messages but actively drafts and executes replies based on real-time inventory and order data.
  - **Codebase Context**: The `CustomerSuccessAgent` handles basic event-driven actions (`tenant.message.received`) and queries a basic long-term memory store. However, it lacks a fully integrated Auto-Responder workflow that can securely draft replies, check policies, request owner approval via a mobile-first UI, and automatically reply when confidence is high. The `UnifiedOrchestrator` exists and can route tasks, but a dedicated, proactive "Intelligent Auto-Responder" loop is incomplete.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Inbox as Omni-Channel Inbox API
      participant AutoResponder as CustomerSuccessAgent (AutoResponder)
      participant Memory as OHC Memory & Inventory
      participant Owner as Owner (Mobile UI)

      Customer->>OHC_Inbox: "Do you have vegan cakes for Saturday?"
      OHC_Inbox->>AutoResponder: Event: `tenant.omnichannel.message.received`
      AutoResponder->>Memory: Query policies, inventory, past orders
      Memory-->>AutoResponder: Context (Vegan options available, Saturday slots open)
      AutoResponder->>AutoResponder: Draft response (LLM)
      alt High Confidence & Auto-Execute Enabled
          AutoResponder->>OHC_Inbox: Send Reply
          OHC_Inbox->>Customer: "Yes, we do! Would you like me to hold a slot?"
          AutoResponder->>Owner: Push Notification: "Auto-replied to 1 inquiry."
      else Needs Review / Complex
          AutoResponder->>Owner: Draft pushed to Mobile App for Review
          Owner->>AutoResponder: Approves/Edits Draft
          AutoResponder->>OHC_Inbox: Send Reply
      end
  ```

  ### Mobile UX Flow (375px first)
  1. **Home Screen (The Feed)**: Owner sees an aggregated list of "Draft Replies" waiting for review.
  2. **Review Card**: A translucent glass-styled card showing the customer's original message, context (e.g., "Returning Customer"), and the AI's drafted response.
  3. **Interaction**: Swipe right to approve and send. Tap to edit text manually. Swipe left to discard or handle manually.
  4. **Settings (Advanced)**: A simple toggle for "Auto-reply to common questions" (behind an advanced settings gear), keeping the main UI clear of configuration.

  ### AI Agent Integration Points
  - The `CustomerSuccessAgent` (in `src/server/orchestration/departments/customer_success_agent.rs`) must be enhanced to classify incoming messages into "Intent Categories" (e.g., FAQ, Custom Quote, Complaint).
  - Enhance the `UnifiedOrchestrator` to queue high-risk replies for human approval and auto-dispatch low-risk replies based on a configurable `ActionRisk` threshold.
  - Ensure Zero-Trust multi-tenant isolation: the LLM must only access the `tenant_id` context via row-level security.

  ## Implementation Prompt
  **Goal:** Implement the end-to-end Intelligent Customer Auto-Responder for omnichannel messages, ensuring the owner can review drafted replies on a mobile-first interface.
  **Persona:** Maya (baker) receives Instagram DMs and wants OHC to draft context-aware replies for her to approve with one tap.
  **Acceptance Criteria:**
  1. Enhance `CustomerSuccessAgent` to draft replies with a calculated confidence score.
  2. Implement a `GET /api/v1/inbox/drafts` endpoint returning pending replies.
  3. Implement the UI: A mobile-responsive (375px baseline) component using OHC Translucent Glass styling that displays the drafted reply, the customer's message, and a 1-tap "Approve & Send" button.
  4. E2E Playwright test: A seeded message triggers the agent, the draft appears in the UI, and clicking "Approve" transitions the state and sends the message.
  5. 100% unit test coverage for the backend logic and zero mock data in the frontend.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []