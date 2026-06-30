issue_title: "Implement Agentic AI Omnichannel Inbox with Zero-Trust Multi-Tenancy"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (Home Baker) and Carlos (Field Service Owner) receive a fragmented stream of customer inquiries across Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to slow response times, disjointed context, and ultimately lost sales. Existing platforms like Shopify Inbox simply aggregate messages without deep context or autonomous capabilities. The owner must still manually type responses without instant access to the customer's purchase history across all channels. This reactive process does not scale for a solopreneur who is actively doing physical work, and current systems lack strong Zero-Trust guarantees to separate sensitive cross-channel data between businesses.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify & Wix Inboxes:** They aggregate chat and email but rely heavily on manual responses or rigid, simple auto-replies. They do not proactively draft contextual responses based on full customer history across all channels (e.g., matching an Instagram DM to a previous website purchase).
  - **Zendesk/Intercom:** Enterprise-grade tools that are overly complex, expensive, and require a steep learning curve not suitable for a single-person SMB.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy to create an Omnichannel AI Inbox powered by "The Ambassador" Customer Success Agent. The agent doesn't just aggregate; it reads, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner sees an "Action Required: Approve Reply" card in their 375px mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel API Gateway)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Zero-Trust Identity Resolution Engine}
      E -->|SPIFFE/SPIRE Auth Lookup| F[Tenant-Isolated Customer Graph DB]
      E --> G[Secure Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|Context Query| F
      H -->|Draft Reply Generation| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
      K --> A/C/D
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** The user's feed prominently displays actionable cards, e.g., "1 New Message from Sarah (Insta DM)".
  - **Unified View:** Tapping the card opens a unified interaction view. The top half displays customer context (e.g., "Sarah bought a vegan cake 2 months ago"), and the bottom half shows the AI-drafted reply.
  - **Action Buttons:** Clear, large touch-friendly buttons: a primary "Approve & Send" and a secondary "Edit".
  - **Design System:** Follows the OHC Premium Token library with translucent glass styling and clean Apple/Ubiquiti-style hierarchy.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Hooks into the incoming message stream via the event mesh, pulls tenant-scoped contextual data (past orders, previous conversations) using strict RAG, and uses the Gemini Pro LLM to generate a natural, on-brand draft response.
  - **Zero-Trust Multi-Tenancy:** All context queries and webhook ingestions are strictly isolated per tenant using SPIFFE SVIDs, ensuring no data leakage across different businesses.

  ### Key Design Decisions
  - **Proactive Over Reactive:** Instead of waiting for the user to open an inbox, the system pushes AI-drafted replies into the central feed.
  - **Zero-Trust Backbone:** Employs SPIFFE/SPIRE workload identities for all inter-agent communications and database queries to ensure absolute tenant data isolation.

  ## Implementation Prompt
  Implement the Omnichannel AI Inbox backend and mobile-first feed integration.
  - Develop the Omnichannel API Gateway to receive webhooks from social platforms.
  - Integrate "The Ambassador" agent to classify intent, fetch tenant-isolated customer history, and draft contextual replies.
  - Build the Mobile UX component (375px optimized) that displays the AI-drafted reply as an actionable card in the Agent Feed with a 1-tap "Approve & Send" button.
  - Ensure all internal data access strictly adheres to the Zero-Trust multi-tenancy model using the existing SPIFFE/SPIRE infrastructure.
  - **Acceptance Criteria:** A simulated inbound message must trigger the agent, generate a draft response using tenant context, appear in the mobile UI feed, and allow the user to approve and dispatch the reply successfully.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []