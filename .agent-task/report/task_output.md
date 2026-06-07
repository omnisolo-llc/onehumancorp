issue_title: "Implement True RAG Pipeline & Tauri UI for Ambassador Agent"
issue_description: |
  ## Problem Statement
  Solopreneurs like Maya (the baker) are currently missing out on potential sales because they cannot actively monitor and respond to Instagram DMs and WhatsApp messages while working. Although OHC has an initial foundation for the "Ambassador Agent", it currently uses placeholder embeddings rather than a true RAG (Retrieval-Augmented Generation) pipeline connected to real business data (inventory, availability, and store policies). Furthermore, the interface for reviewing and approving these drafted replies is still housed in the legacy Next.js web application, which contradicts OHC's mandate that new features be mobile-first and built in the canonical Tauri v2 application.

  ## Research Report
  - **Codebase Findings**: The `CustomerSuccessAgent` handles incoming omnichannel messages (`tenant.omnichannel.message.received`) and triggers draft generation. However, the vector embedding and retrieval logic currently utilizes dummy zero vectors and merely logs the action instead of retrieving dynamic inventory data or real business policies.
  - **UI Deficiencies**: The `ApprovalInbox` component exists only in `src/ui/next/src/app/team/components/ApprovalInbox.tsx` (the legacy prototype). The primary platform architecture requires migrating this workflow to `src/ui/tauri/`.
  - **Competitive Edge**: Standard legacy tools (like Shopify) rely on separate third-party apps for automated messaging, which don't have direct access to inventory and operations data. OHC can differentiate itself by making the Ambassador Agent natively aware of real-time inventory and fulfillment constraints via the RAG pipeline.

  ## Design Doc
  ### Mobile UX Flow (375px)
  1. A customer sends an inquiry via Instagram (e.g., "Do you have vegan cakes today?").
  2. The owner receives a push notification on their phone via the Tauri mobile-first shell.
  3. The owner opens the "Action Card" feed on their 375px mobile viewport.
  4. The card displays the original customer inquiry and the AI-drafted reply. The drafted reply must explicitly reference real-time availability.
  5. The owner taps a prominent (min 44x44px target) "Approve & Send", "Edit", or "Decline" button.

  ### Architecture
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Webhook
      participant Ambassador_Agent
      participant RAG_Engine
      participant Inventory_DB
      participant Tauri_Mobile_Feed

      Customer->>OHC_Webhook: Sends DM (Instagram)
      OHC_Webhook->>Ambassador_Agent: Dispatches "tenant.omnichannel.message.received"
      Ambassador_Agent->>RAG_Engine: Queries Intent (e.g. "vegan cakes availability")
      RAG_Engine->>Inventory_DB: Fetches exact inventory count for requested items
      RAG_Engine-->>Ambassador_Agent: Returns relevant context (Inventory = 3 left)
      Ambassador_Agent->>Ambassador_Agent: LLM drafts response using context
      Ambassador_Agent->>Tauri_Mobile_Feed: Creates "DraftForReview" Action Card
      Tauri_Mobile_Feed-->>Ambassador_Agent: Owner Approves
      Ambassador_Agent->>Customer: Sends reply via Webhook API
  ```
  *Key Decisions*:
  - Do not invent policies; rely strictly on retrieved vector context.
  - Implement the review inbox natively in Tauri using the translucent glass design system and modular Unifi-style cards.

  ## Implementation Prompt
  Implement the Ambassador Agent's full end-to-end RAG pipeline and Tauri UI.
  1. Update `CustomerSuccessAgent` in `src/server/orchestration/departments/customer_success_agent.rs` to replace the dummy embeddings with an actual RAG query that fetches real-time product inventory and policy documents.
  2. Migrate the `ApprovalInbox` feature from the legacy Next.js prototype to the canonical `src/ui/tauri` application. Build it mobile-first (375px) using the OHC Premium Token design system. Ensure touch targets are at least 44x44px.
  3. Create an E2E Playwright test simulating Maya receiving an availability inquiry, the agent fetching the correct inventory count, and Maya approving the generated response on the mobile viewport.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
