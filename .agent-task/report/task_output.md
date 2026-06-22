issue_title: "Integration: Meta WhatsApp Cloud API for Conversational Operations"
issue_description: |
  ## Problem Statement
  Owners and operators like Maya (Home Baker) and Carlos (Field Service Owner) rely heavily on WhatsApp to coordinate with customers. Traditional software platforms force them to switch between their primary communication channel (WhatsApp) and their management tools. This causes missed leads, delayed responses, and scattered context. A non-technical owner needs an assistant that brings their management tools (scheduling, quoting, reminders) directly into the conversation stream where the customer actually is.

  ## Research Report
  - **Tool Evaluated:** Meta WhatsApp Cloud API
  - **Relevance:** WhatsApp is the dominant messaging app globally, particularly for small businesses. Competitors like WeCom and WhatsApp Business app handle basic messaging, but lack the unified integration of OHC's capabilities (scheduling, invoicing, payments).
  - **Ease of Use:** For the end-customer, it's frictionless. For the owner, OHC abstracts the complexity of API limits and template approvals. The setup flow will require a clear step-by-step connection to a Meta Developer account or a BSP (Business Solution Provider), which OHC can guide the user through or manage via embedded signup.
  - **Pricing:** The API uses conversation-based pricing. The first 1,000 service conversations each month are free, making it highly viable for OHC's small business personas. Marketing/utility templates carry specific per-message costs depending on the region.
  - **Capabilities:** Supports rich media, interactive messages (buttons, list messages), and automated flows. Webhooks deliver real-time notifications of incoming messages and status updates (sent, delivered, read).
  - **SaaS Viability:** Fully multi-tenant. OHC can store individual WABA (WhatsApp Business Account) credentials per tenant or act as an ISV (Independent Software Vendor) using embedded signup.

  ## Design Doc
  - **Setup:** A dedicated setup flow where the owner connects their WhatsApp Business number. OHC provides a clear, jargon-free checklist for Meta Business Manager verification.
  - **Work Triage:** Incoming WhatsApp messages are ingested via webhooks and routed to OHC's Work Triage queue. The assistant groups them logically by customer.
  - **Customer Relationship Assistant:** The agent contextually analyzes the conversation and suggests draft replies. The owner can approve, modify, or let the agent auto-reply within predefined boundaries (e.g., standard FAQs).
  - **Operations & Sales Assistants:** Agents can automatically send appointment reminders, deposit links, or project updates as interactive WhatsApp messages (e.g., "Confirm your appointment: [Yes] [Reschedule]").
  - **Observability:** OHC handles 24-hour customer service window restrictions and warns the owner if a template message must be used instead of a free-form reply.

  ## Implementation Prompt
  Implement the WhatsApp Cloud API integration.
  - Create a setup UI that allows a non-technical owner to connect their WhatsApp Business number.
  - Set up a robust webhook ingestion pipeline that safely stores incoming messages in the tenant's workspace and triggers the Work Triage agent.
  - Build a chat interface inside OHC where the owner can view WhatsApp conversations, see AI-suggested drafts, and send replies.
  - Ensure the system automatically tracks the 24-hour reply window and prompts the user to use an approved template if the window has closed.
  - Acceptance Criteria: A user can successfully connect a test number, receive a message from a customer in the OHC feed, have the assistant suggest a reply, and successfully send that reply back to the customer's WhatsApp device. All operations must be cleanly tenant-isolated.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
