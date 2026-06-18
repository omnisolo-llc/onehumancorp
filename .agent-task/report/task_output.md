issue_title: "Implement AI Unified Inbox & Omnichannel Customer Memory"
issue_description: |
  # Title: AI Unified Inbox Differentiation & Omnichannel Customer Memory

  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.

  ## Research Report
  - **Competitor Analysis:** Shopify Inbox aggregates chat and email but relies on manual responses or rigid auto-replies. Wix Inbox has limited AI to improve tone. Enterprise tools like Zendesk/Intercom are too complex and expensive for a single-person SMB.
  - **The Gap:** There is a lack of proactive, AI-driven customer success agents that maintain context across all channels.

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

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view showing customer context (e.g., Sarah bought a vegan cake 2 months ago) and an AI-drafted reply.
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".

  ### AI Agent Integration
  - **Customer Success Agent (The Ambassador):** Uses RAG (Retrieval-Augmented Generation) against the tenant's product catalog and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** Verifies inventory or calendar availability before the Ambassador drafts the reply.

  ## Implementation Prompt
  **Target Persona**: Maya the Baker
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it.

  **Acceptance Criteria:**
  1. A simulated external message (e.g., via a test webhook) is ingested by the Omnichannel Gateway.
  2. The Customer Identity Resolution Engine correctly matches the incoming identifier to an existing customer record.
  3. The Ambassador Agent queries the customer's past orders and current product catalog, and generates a draft reply.
  4. The draft reply appears in the `ActionRequiredQueue` for the specific tenant.
  5. Include E2E Playwright tests verifying the mobile-sized feed approval flow and dispatch.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
