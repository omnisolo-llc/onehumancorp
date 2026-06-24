issue_title: "Unified Omnichannel Inbox & Autonomous Ambassador Agent Workflow"
issue_description: |
  ## 1. Problem Statement
  Small business owners (like Maya the baker or Carlos the handyman) receive customer inquiries across multiple disconnected channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to dropped messages, slow response times, and lost sales. Traditional e-commerce platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages into a single view but still rely entirely on the business owner to manually type responses. They lack deep omnichannel memory—failing to link a customer's Instagram DM to their previous Shopify email order history automatically. This creates a reactive, labor-intensive process that does not scale for solopreneurs.

  ## 2. Research Report
  **Competitive Landscape:**
  - **Shopify Inbox:** Aggregates chat, Instagram, and email. However, its AI ("Sidekick" or basic auto-replies) is primarily reactive and rigid. It does not proactively draft contextual responses based on full customer history across all channels while the owner sleeps.
  - **Wix Inbox:** Good aggregation but AI features are limited to "improving tone" or generating generic boilerplate replies.
  - **Zendesk / Intercom:** Enterprise-grade identity resolution and routing, but far too complex, expensive, and heavy for a single-person SMB.

  **The OHC Differentiator:**
  OHC leverages its "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (linking phone numbers, emails, and social handles to a single `Customer` entity), queries the current product catalog/inventory, and proactively drafts a complete, accurate response. The owner simply opens their mobile app and sees an "Action Required: Approve Reply" card in their 375px feed. One tap to approve.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup via phone/email/handle| F[Unified Customer Graph DB]
      E --> G[Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|Query Context & Inventory| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
      K --> A/C/D
  ```

  ### Mobile UX Flow (375px First)
  - **Notification/Feed:** The owner's home feed displays a top-level action card: "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view optimized for 375px viewports (no horizontal scrolling).
    - **Top Half (Context):** Shows the customer context (e.g., "Sarah bought a vegan cake 2 months ago").
    - **Bottom Half (Draft):** Shows the AI-drafted reply (e.g., "Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** A prominent, thumb-friendly primary button (min 44x44px touch target) "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards, blurred backgrounds to maintain focus, native keyboard integration if the user decides to edit.

  ### AI Agent Integration Points
  - **Identity Resolution (`resolve_identity`):** Webhooks from Meta, Twilio, etc., hit the API. The `resolve_identity` function queries `customer_identities` and `customers` to map the sender ID to a canonical `customer_id`.
  - **The Ambassador (Customer Success Agent):** Triggered by the `tenant.omnichannel.message.received` event on the event mesh. Uses RAG (Retrieval-Augmented Generation) against the tenant's product catalog, store policies, and the customer's specific order history to draft a highly personalized reply.
  - **Cross-Agent Coordination:** If a message implies a booking or order change, the Ambassador consults the Operations Agent to verify inventory/calendar availability before finalizing the draft.

  ## 4. Implementation Prompt
  **Feature Name:** Unified Omnichannel Inbox & Autonomous Ambassador Agent Workflow
  **Target Persona:** Maya the Baker

  **User-Facing Outcome:**
  When a customer DMs Maya on Instagram asking about their past order, Maya opens the OHC app to find a pre-written, perfectly accurate response already drafted based on her inventory and the customer's history. She taps one button to send it, taking 2 seconds instead of 2 minutes.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. A simulated external message (e.g., via a test webhook payload to `/api/v1/omnichannel/webhook`) is ingested by the system.
  2. The Customer Identity Resolution Engine correctly matches the incoming identifier (e.g., social handle) to an existing customer record in the database, or safely creates a new link.
  3. The Ambassador Agent is triggered asynchronously via the event mesh (`ohc_job_queue` triage).
  4. The Agent successfully queries the customer's past orders and the current product catalog, generating a draft reply, placing it in the `ActionRequiredQueue`.
  5. **Playwright E2E Test Requirement:** A user logs into the UI (mobile viewport 375px), sees the drafted message card ("Action Required: Approve Reply") on their feed, taps "Approve", and the system records the dispatch. The UI elements must adhere to minimum 44x44px touch targets.

  ## 5. Priority & Scope
  - **Priority:** P0 (Critical path for core product differentiation)
  - **Estimated Scope:** Large (involves Webhooks, Identity Resolution, Agent Prompting/RAG, and UI Feed)

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []