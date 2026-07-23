issue_title: "Implement 'The Ambassador' - Omnichannel AI Inbox for SMBs"
issue_description: |
  ## The Problem Statement
  Small business owners like 'Maya the Baker' (who runs her business via Instagram DMs) or 'Carlos the Handyman' rely heavily on messaging for sales and support. However, they lack a unified system to handle WhatsApp, Instagram, SMS, and email. Existing tools (Shopify Inbox, Wix Inbox) are just passive aggregators—they require the owner to manually type responses without context. OHC needs "The Ambassador," an autonomous AI agent that intercepts incoming omnichannel messages, checks customer context (past orders, current inventory), and drafts highly contextual replies for the owner's 1-tap approval via a mobile-first interface.

  ## Research Report
  **Competitive Landscape & Pain Points:**
  - Traditional aggregators require significant manual labor; solopreneurs don't have time to answer every DM manually while baking or servicing a home.
  - Current SMB platforms provide tools, not staff. OHC's differentiation is shifting from "Inbox tool" to "Customer Success Agent."
  - **Shopify Inbox / Wix Inbox**: Simple aggregation; basic chatbots that struggle with context and require the user to configure rigid rules.
  - **Opportunity**: Build an intent-driven agent ("The Ambassador") that proactively drafts responses based on tenant data (inventory, orders, FAQs) instead of waiting for the user to type.

  **Core Flow:**
  Message arrives (webhook) -> Identity Resolution -> Context Fetching (RAG against inventory/orders) -> LLM drafts reply -> Places in ActionRequired queue -> Owner receives mobile notification -> Owner 1-taps "Approve" -> Reply sent.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. **Notification/Feed**: The OHC app home feed displays an "Action Required" translucent glass card: "Drafted reply for Sarah (Instagram)".
  2. **Detail View**: Tapping the card opens a detail view.
     - **Top Section**: Context (Sarah bought a cake 2 weeks ago; Vegan Chocolate is in stock).
     - **Middle Section**: The AI-drafted reply ("Hi Sarah, yes we have Vegan Chocolate! Here is the booking link...").
     - **Bottom Section**: Two full-width, 44px-minimum touch target buttons: "Send Draft" (Primary, `#0066FF`) and "Edit" (Secondary).
  3. **Visual Design**: The UI must use OHC Premium Tokens: `.glassmorphism` container with `backdrop-filter: blur(30px) saturate(210%)` and `border-radius: 16px`.

  ### Architecture
  ```mermaid
  graph TD
      A[Omnichannel Gateway / Webhook] --> B[Identity Resolution]
      B --> C[The Ambassador Agent]
      C --> D[RAG: Inventory & Order DB]
      C --> E[Draft Queue]
      E --> F[Mobile App Action Card]
      F --> G[Owner 1-Tap Approve]
      G --> H[Message Dispatcher]
  ```

  ### Integration & Constraints
  - Ensure strict multi-tenant isolation (row-level security) for the Draft Queue and Message DB.
  - The Draft Queue must use PostgreSQL `SKIP LOCKED` pattern for scalability.
  - Use the built-in LLM provider configuration (`OHC_LLM_PROVIDER`) to generate the draft.

  ## Implementation Prompt
  Implement "The Ambassador" agent flow and UI.
  1. Create the backend data structures and API endpoints for receiving external messages, resolving the customer, generating a draft response via the LLM, and storing it in an approval queue.
  2. Create the mobile-first (375px) UI component in Tauri/React for reviewing and approving the drafted message, strictly adhering to the translucent glass and touch-target standards.
  3. Ensure zero mock data in the UI; state must flow end-to-end.
  4. Write comprehensive unit tests for the agent logic and at least 5 Playwright E2E tests validating the end-to-end CUJ from a mocked external message to UI approval.
  5. Ensure all `bazel test //...` passes.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
