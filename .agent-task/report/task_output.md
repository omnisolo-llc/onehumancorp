issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. We are retiring the third-party Chatwoot dependency to build a high-performance, strictly multi-tenant, native Rust chat system directly into OHC.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup| F[Unified Customer Graph DB]
      E --> G[Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
      K --> A/C/D
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh. Uses RAG against the tenant's product catalog and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** If the message implies an order change or booking request, The Manager agent is called to verify inventory/calendar availability before The Ambassador drafts the reply.

  ## Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes.

  **CUJ & Acceptance Criteria:**
  1. A simulated external message (e.g., via a test webhook) is ingested by the Omnichannel Gateway.
  2. The Customer Identity Resolution Engine correctly matches the incoming identifier (e.g., social handle) to an existing customer record in the database.
  3. The Ambassador Agent is triggered and successfully queries the customer's past orders and the current product catalog.
  4. The Agent generates a draft reply and places it in the `ActionRequiredQueue` for the specific tenant.
  5. Provide Playwright E2E tests for the 375px mobile feed showcasing the drafted reply and 1-tap approval.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
