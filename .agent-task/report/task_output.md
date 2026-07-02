issue_title: "Implement the Ambassador Agent for Instagram DM Intake"
issue_description: |
  # The Ambassador Agent - Automated DM Intake

  ## Problem Statement
  Small business owners like Maya (the home baker) rely heavily on Instagram and WhatsApp DMs to receive custom orders and field customer inquiries. Manually triaging these messages while baking or managing deliveries leads to lost sales and poor response times. Existing automation tools (like ManyChat) require complex visual builders that are too technical for our non-technical owner/operator persona.

  ## Research Report
  Our competitive research indicates a critical gap in native, zero-configuration agentic workflows for unified inboxes. The Ambassador Agent directly addresses Pain Point #3 (Omnichannel Chaos) by automatically ingesting DMs, classifying intent, and drafting contextual replies based on the merchant's data (inventory, FAQs).

  ## Design Doc

  ### Architecture Diagram (Conceptual)
  - **Ingestion**: A webhook endpoint receives incoming messages (simulating Instagram Graph API).
  - **Processing**: The message payload is processed to determine the `tenant_id` and customer context.
  - **Intent Classification & RAG**: The Ambassador Agent uses an LLM (with fallback string matching for simplicity/tests) to classify the intent (e.g., pricing, availability, support) and draft a response based on tenant-specific context.
  - **Data Persistence**: The incoming message and the drafted reply are stored in `omni_inbox_messages` in PostgreSQL.
  - **Approval Flow**: The drafted message requires owner approval before sending, appearing in the unified inbox.

  ### Mobile UX Flow
  1. A customer sends a DM to Maya's Instagram: "Do you have any vegan cakes available for tomorrow?"
  2. The webhook ingests the message.
  3. The Ambassador Agent drafts a reply: "Yes, we have 2 vegan chocolate cakes left! Would you like to reserve one?"
  4. Maya receives a push notification and opens the OHC app (375px viewport).
  5. She sees the drafted reply card with clear touch targets (≥44x44px) for "Approve & Send", "Edit", or "Discard".
  6. Maya taps "Approve & Send".

  ### Agent Integration Points
  - `src/server/domain/inbox.rs`: Currently handles the approval action (`handle_inbox_action`).
  - Need to implement the ingestion/webhook side and the agent logic to generate `draft_reply`.

  ## Implementation Prompt
  Implement the Ambassador Agent intake flow.
  - Create a simulated webhook endpoint or service function that accepts an incoming DM payload (sender, message, source platform).
  - Implement the logic to save the incoming message to `omni_inbox_messages` with status 'pending'.
  - Trigger the Ambassador Agent to generate a `draft_reply` based on the message content.
  - Update the `omni_inbox_messages` record with the `draft_reply` and set status to 'drafted', ready for owner approval.
  - Ensure 100% unit test coverage for the new logic and ensure `bazel test //...` passes.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
